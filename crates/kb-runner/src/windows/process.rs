//! Launch the child inside the job and the AppContainer, pump stdio, enforce the
//! wall-clock and output limits, and collect accounting.

use std::fs::File;
use std::io::{Read, Write};
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::{
    CloseHandle, HANDLE, HANDLE_FLAG_INHERIT, SetHandleInformation,
};
use windows_sys::Win32::Security::SECURITY_CAPABILITIES;
use windows_sys::Win32::System::Pipes::CreatePipe;
use windows_sys::Win32::System::SystemServices::{
    JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT, JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO,
    JOB_OBJECT_MSG_END_OF_JOB_TIME, JOB_OBJECT_MSG_JOB_MEMORY_LIMIT,
};
use windows_sys::Win32::System::Threading::{
    CREATE_DEFAULT_ERROR_MODE, CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT, CreateProcessW,
    DeleteProcThreadAttributeList, EXTENDED_STARTUPINFO_PRESENT, GetExitCodeProcess,
    InitializeProcThreadAttributeList, LPPROC_THREAD_ATTRIBUTE_LIST,
    PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY, PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
    PROC_THREAD_ATTRIBUTE_JOB_LIST, PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES,
    PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOEXW, UpdateProcThreadAttribute,
};

use super::acl::{self, Access};
use super::appcontainer::{self, AppContainerSid};
use super::job::{Job, JobLimits};
use super::{last_error, wide, workdir};
use crate::cmdline::quote_windows_arg;
use crate::validate::{self, Resolved};
use crate::{APP_CONTAINER_NAME, Outcome, SandboxError, SandboxSpec, Termination};

/// From WinBase.h (lives in the large WindowsProgramming module of windows-sys).
const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 0x01;

/// Exit codes the runner uses when it kills the tree itself ("KB" + reason).
const KILLED_WALL_TIME: u32 = 0x4B42_0001;
const KILLED_OUTPUT: u32 = 0x4B42_0002;
const KILLED_CPU_TIME: u32 = 0x4B42_0003;

const POLL: Duration = Duration::from_millis(20);
/// How long to wait for the tree to die after a kill before giving up on it.
const KILL_GRACE: Duration = Duration::from_secs(5);

/// Our child stdio handles are made inheritable only while this lock is held, so two
/// concurrent runs can never leak pipe ends into each other's children.
static SPAWN_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn run(spec: &SandboxSpec) -> Result<Outcome, SandboxError> {
    // Validate before creating or granting anything.
    let paths = validate::resolve(spec)?;
    let sid = appcontainer::ensure_profile(APP_CONTAINER_NAME)?;
    let result = grant_and_execute(spec, &paths, &sid);
    // `execute` has returned, so its job is closed and every sandboxed process is dead.
    let cleanup = workdir::finish(&paths.work_dir, &sid, spec.keep_work_dir);
    match (result, cleanup) {
        (Ok(outcome), Ok(())) => Ok(outcome),
        (Err(e), _) | (Ok(_), Err(e)) => Err(e),
    }
}

fn grant_and_execute(
    spec: &SandboxSpec,
    paths: &Resolved,
    sid: &AppContainerSid,
) -> Result<Outcome, SandboxError> {
    appcontainer::seal_profile_storage(APP_CONTAINER_NAME, sid)?;
    for dir in &paths.read_only_dirs {
        acl::grant(dir, sid, Access::ReadExecute)?;
    }
    acl::grant(&paths.work_dir, sid, Access::ReadWriteExecute)?;
    execute(spec, paths, sid)
}

/// Owns the job: whichever way this returns, the job handle closes and
/// KILL_ON_JOB_CLOSE ends any process still in it.
fn execute(
    spec: &SandboxSpec,
    paths: &Resolved,
    sid: &AppContainerSid,
) -> Result<Outcome, SandboxError> {
    let job = Job::new(&JobLimits {
        cpu_time: spec.cpu_time,
        memory_bytes: spec.memory_bytes,
        max_processes: spec.max_processes,
    })?;

    let (stdin_r, stdin_w) = pipe()?;
    let (stdout_r, stdout_w) = pipe()?;
    let (stderr_r, stderr_w) = pipe()?;
    let process = spawn(
        spec,
        paths,
        sid,
        &job,
        [
            stdin_r.as_raw_handle(),
            stdout_w.as_raw_handle(),
            stderr_w.as_raw_handle(),
        ],
    )?;
    let start = Instant::now();
    // Close our copies of the child ends so readers see EOF when the tree exits.
    drop((stdin_r, stdout_w, stderr_w));

    let output = Arc::new(OutputState::default());
    let out_t = spawn_reader(stdout_r, &output, spec.max_output_bytes);
    let err_t = spawn_reader(stderr_r, &output, spec.max_output_bytes);
    let stdin_data = spec.stdin.clone();
    let in_t = thread::spawn(move || {
        let mut f = File::from(stdin_w);
        let _ = f.write_all(&stdin_data); // the child may exit without reading
    });

    let mut termination: Option<Termination> = None;
    let mut memory_limit_hit = false;
    let mut process_limit_hit = false;
    let mut max_active = 0u32;
    let deadline = start + spec.wall_time;
    let mut killed_at: Option<Instant> = None;

    let mut handle_msg = |msg: u32, termination: &mut Option<Termination>| -> bool {
        match msg {
            JOB_OBJECT_MSG_END_OF_JOB_TIME => {
                termination.get_or_insert(Termination::CpuTimeLimit);
            }
            JOB_OBJECT_MSG_JOB_MEMORY_LIMIT => memory_limit_hit = true,
            JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT => process_limit_hit = true,
            JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO => return true,
            _ => {}
        }
        false
    };

    loop {
        let now = Instant::now();
        if termination.is_none() && output.exceeded.load(Ordering::SeqCst) {
            job.terminate(KILLED_OUTPUT);
            termination = Some(Termination::OutputLimit);
        }
        if termination.is_none() && now >= deadline {
            job.terminate(KILLED_WALL_TIME);
            termination = Some(Termination::WallTimeLimit);
        }
        if termination.is_some() && killed_at.is_none() {
            killed_at = Some(now);
        }
        let wait = if termination.is_some() {
            POLL
        } else {
            deadline.saturating_duration_since(now).min(POLL)
        };
        let mut all_exited = false;
        if let Some(msg) = job.next_message(wait) {
            all_exited = handle_msg(msg, &mut termination);
        }
        let acct = job.accounting();
        max_active = max_active.max(acct.active_processes);
        if all_exited || acct.active_processes == 0 {
            break;
        }
        // The CPU limit is user + kernel time, enforced here from accounting (overshoot
        // <= one POLL). The job's own limit counts user time only, and Windows checks it
        // lazily (seen overshooting 2x), so it is just a backstop.
        if termination.is_none() && acct.user_time + acct.kernel_time >= spec.cpu_time {
            job.terminate(KILLED_CPU_TIME);
            termination = Some(Termination::CpuTimeLimit);
        }
        if killed_at.is_some_and(|t| t.elapsed() > KILL_GRACE) {
            break; // reported via active_processes_after; KILL_ON_JOB_CLOSE still applies on drop
        }
    }
    // Late notifications (e.g. a memory-limit hit just before exit).
    while let Some(msg) = job.next_message(Duration::ZERO) {
        handle_msg(msg, &mut termination);
    }
    let wall_time = start.elapsed();

    // The tree can exit right as the output cap is crossed.
    if termination.is_none() && output.exceeded.load(Ordering::SeqCst) {
        termination = Some(Termination::OutputLimit);
    }
    let acct = job.accounting();
    if termination.is_none() && acct.user_time + acct.kernel_time >= spec.cpu_time {
        termination = Some(Termination::CpuTimeLimit); // accounting is authoritative
    }

    let mut exit_code = 0u32;
    // SAFETY: valid process handle and out-pointer.
    if unsafe { GetExitCodeProcess(process.as_raw_handle(), &mut exit_code) } == 0 {
        // Returning drops `job`, which kills anything left; detached readers then see EOF.
        return Err(last_error("GetExitCodeProcess"));
    }
    if acct.active_processes > 0 {
        return Err(SandboxError::Internal(
            "processes survived termination; job closed to kill them".into(),
        ));
    }

    let stdout = out_t.join().unwrap_or_default();
    let stderr = err_t.join().unwrap_or_default();
    let _ = in_t.join();

    Ok(Outcome {
        mode: if spec.less_privileged {
            crate::SandboxMode::LessPrivilegedAppContainer
        } else {
            crate::SandboxMode::AppContainer
        },
        termination: termination.unwrap_or(Termination::Exited),
        exit_code,
        stdout,
        stderr,
        memory_limit_hit,
        process_limit_hit,
        user_time: acct.user_time,
        kernel_time: acct.kernel_time,
        wall_time,
        peak_memory_bytes: job.peak_memory(),
        total_processes: acct.total_processes,
        max_active_processes: max_active,
        active_processes_after: acct.active_processes,
    })
}

#[derive(Default)]
struct OutputState {
    total: AtomicUsize,
    exceeded: AtomicBool,
}

/// Reads one pipe to EOF. It only flags the cap; the wait loop does the kill, so the
/// job is never shared with these threads.
fn spawn_reader(h: OwnedHandle, state: &Arc<OutputState>, limit: usize) -> JoinHandle<Vec<u8>> {
    let state = Arc::clone(state);
    thread::spawn(move || {
        let mut f = File::from(h);
        let mut buf = vec![0u8; 16 * 1024];
        let mut out = Vec::new();
        // Keep draining after the cap so the child never blocks on a full pipe.
        while let Ok(n) = f.read(&mut buf) {
            if n == 0 {
                break;
            }
            let before = state.total.fetch_add(n, Ordering::SeqCst);
            let room = limit.saturating_sub(before);
            out.extend_from_slice(&buf[..n.min(room)]);
            if before + n > limit {
                state.exceeded.store(true, Ordering::SeqCst);
            }
        }
        out
    })
}

/// Anonymous pipe with both ends non-inheritable.
fn pipe() -> Result<(OwnedHandle, OwnedHandle), SandboxError> {
    let mut r: HANDLE = null_mut();
    let mut w: HANDLE = null_mut();
    // SAFETY: valid out-pointers; null attributes => non-inheritable handles.
    if unsafe { CreatePipe(&mut r, &mut w, std::ptr::null(), 0) } == 0 {
        return Err(last_error("CreatePipe"));
    }
    // SAFETY: both handles are fresh and owned by nobody else.
    Ok(unsafe {
        (
            OwnedHandle::from_raw_handle(r),
            OwnedHandle::from_raw_handle(w),
        )
    })
}

fn set_inherit(h: HANDLE, on: bool) -> Result<(), SandboxError> {
    let flags = if on { HANDLE_FLAG_INHERIT } else { 0 };
    // SAFETY: `h` is a valid handle we own.
    if unsafe { SetHandleInformation(h, HANDLE_FLAG_INHERIT, flags) } == 0 {
        return Err(last_error("SetHandleInformation"));
    }
    Ok(())
}

/// PROC_THREAD_ATTRIBUTE_LIST in an 8-byte-aligned buffer.
struct AttrList(Vec<u64>);

impl AttrList {
    fn new(count: u32) -> Result<Self, SandboxError> {
        let mut size = 0usize;
        // SAFETY: the first call only reports the required size (it "fails" by design).
        unsafe { InitializeProcThreadAttributeList(null_mut(), count, 0, &mut size) };
        let mut buf = vec![0u64; size.div_ceil(8)];
        // SAFETY: `buf` has at least `size` bytes.
        if unsafe { InitializeProcThreadAttributeList(buf.as_mut_ptr() as _, count, 0, &mut size) }
            == 0
        {
            return Err(last_error("InitializeProcThreadAttributeList"));
        }
        Ok(AttrList(buf))
    }

    fn ptr(&mut self) -> LPPROC_THREAD_ATTRIBUTE_LIST {
        self.0.as_mut_ptr() as _
    }

    /// # Safety
    /// `value` must point to `size` valid bytes that outlive every use of this list.
    unsafe fn set<T>(
        &mut self,
        attribute: u32,
        value: *const T,
        size: usize,
    ) -> Result<(), SandboxError> {
        // SAFETY: forwarded from the caller's contract; the list was initialized in `new`.
        let ok = unsafe {
            UpdateProcThreadAttribute(
                self.ptr(),
                0,
                attribute as usize,
                value as *const _,
                size,
                null_mut(),
                std::ptr::null(),
            )
        };
        if ok == 0 {
            Err(last_error("UpdateProcThreadAttribute"))
        } else {
            Ok(())
        }
    }
}

impl Drop for AttrList {
    fn drop(&mut self) {
        // SAFETY: the list was successfully initialized in `new`.
        unsafe { DeleteProcThreadAttributeList(self.ptr()) };
    }
}

fn command_line(spec: &SandboxSpec, paths: &Resolved) -> Result<String, SandboxError> {
    let program = paths
        .program
        .to_str()
        .ok_or_else(|| SandboxError::InvalidSpec("program path is not valid Unicode".into()))?;
    let mut cmd = String::new();
    quote_windows_arg(program, &mut cmd);
    for a in &spec.args {
        cmd.push(' ');
        quote_windows_arg(a, &mut cmd);
    }
    Ok(cmd)
}

/// Sorted, NUL-separated, double-NUL-terminated UTF-16 environment. Never inherits
/// the host environment.
fn env_block(spec: &SandboxSpec, paths: &Resolved) -> Vec<u16> {
    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());
    let work = paths.work_dir.display().to_string();
    // CreateProcess fails with ERROR_ENVVAR_NOT_FOUND for an AppContainer unless
    // LOCALAPPDATA is present; Windows then rewrites LOCALAPPDATA, TEMP and TMP to the
    // (sealed, unwritable) profile folder, so the child cannot rely on TEMP.
    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let mut vars: Vec<(String, String)> = vec![
        ("SystemRoot".into(), system_root.clone()),
        ("windir".into(), system_root),
        ("LOCALAPPDATA".into(), local_app_data),
        ("TEMP".into(), work.clone()),
        ("TMP".into(), work),
    ];
    for (k, v) in &spec.env {
        match vars.iter_mut().find(|(ek, _)| ek.eq_ignore_ascii_case(k)) {
            Some(slot) => slot.1 = v.clone(),
            None => vars.push((k.clone(), v.clone())),
        }
    }
    vars.sort_by_key(|(k, _)| k.to_uppercase());
    let mut block: Vec<u16> = Vec::new();
    for (k, v) in vars {
        block.extend(format!("{k}={v}").encode_utf16());
        block.push(0);
    }
    block.push(0);
    block
}

fn spawn(
    spec: &SandboxSpec,
    paths: &Resolved,
    sid: &AppContainerSid,
    job: &Job,
    stdio: [HANDLE; 3],
) -> Result<OwnedHandle, SandboxError> {
    // Everything the attribute list points at is declared before it, so it outlives it.
    let caps = SECURITY_CAPABILITIES {
        AppContainerSid: sid.as_psid(),
        Capabilities: null_mut(), // zero capabilities: no network, no libraries, no devices
        CapabilityCount: 0,
        Reserved: 0,
    };
    let jobs: [HANDLE; 1] = [job.raw()];
    let lpac_policy: u32 = PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT;
    let mut attrs = AttrList::new(if spec.less_privileged { 4 } else { 3 })?;
    // SAFETY: each value is a local declared above `attrs` with the exact size given.
    unsafe {
        attrs.set(
            PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES,
            &caps,
            size_of::<SECURITY_CAPABILITIES>(),
        )?;
        // Assign the job atomically at creation: no window where the child runs unconfined.
        attrs.set(
            PROC_THREAD_ATTRIBUTE_JOB_LIST,
            jobs.as_ptr(),
            size_of_val(&jobs),
        )?;
        // Inherit exactly the three stdio handles and nothing else.
        attrs.set(
            PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
            stdio.as_ptr(),
            size_of_val(&stdio),
        )?;
        if spec.less_privileged {
            attrs.set(
                PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY,
                &lpac_policy,
                size_of::<u32>(),
            )?;
        }
    }

    // SAFETY: plain-old-data struct; zeroed is the documented starting state.
    let mut si: STARTUPINFOEXW = unsafe { std::mem::zeroed() };
    si.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
    si.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
    si.StartupInfo.hStdInput = stdio[0];
    si.StartupInfo.hStdOutput = stdio[1];
    si.StartupInfo.hStdError = stdio[2];
    si.lpAttributeList = attrs.ptr();

    let app = wide(&paths.program);
    let mut cmd: Vec<u16> = command_line(spec, paths)?
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let env = env_block(spec, paths);
    let cwd = wide(&paths.work_dir);
    // SAFETY: plain-old-data out-struct.
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    let flags = EXTENDED_STARTUPINFO_PRESENT
        | CREATE_UNICODE_ENVIRONMENT
        | CREATE_NO_WINDOW
        | CREATE_DEFAULT_ERROR_MODE;

    let _lock = SPAWN_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for h in stdio {
        set_inherit(h, true)?;
    }
    // SAFETY: all strings are NUL-terminated and live across the call; `cmd` is mutable
    // as CreateProcessW requires; `si` is a fully initialized STARTUPINFOEXW whose
    // attribute list and pointed-to values are alive.
    let ok = unsafe {
        CreateProcessW(
            app.as_ptr(),
            cmd.as_mut_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1, // inherit handles, restricted to PROC_THREAD_ATTRIBUTE_HANDLE_LIST
            flags,
            env.as_ptr() as *const _,
            cwd.as_ptr(),
            &si.StartupInfo,
            &mut pi,
        )
    };
    let spawn_err = (ok == 0).then(|| last_error("CreateProcessW"));
    for h in stdio {
        let _ = set_inherit(h, false);
    }
    if let Some(e) = spawn_err {
        return Err(e);
    }
    // SAFETY: CreateProcessW succeeded, so both handles are valid and ours to own.
    unsafe {
        CloseHandle(pi.hThread);
        Ok(OwnedHandle::from_raw_handle(pi.hProcess))
    }
}

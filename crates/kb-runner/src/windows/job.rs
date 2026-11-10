//! Job Object: one per run. Caps memory, CPU time and process count for the whole
//! tree, kills everything when the handle closes, and reports limit hits on an I/O
//! completion port.

use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::ptr::null_mut;
use std::time::Duration;

use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::IO::{
    CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED,
};
use windows_sys::Win32::System::JobObjects::{
    CreateJobObjectW, JOB_OBJECT_LIMIT_ACTIVE_PROCESS, JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION,
    JOB_OBJECT_LIMIT_JOB_MEMORY, JOB_OBJECT_LIMIT_JOB_TIME, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    JOB_OBJECT_UILIMIT_DESKTOP, JOB_OBJECT_UILIMIT_DISPLAYSETTINGS, JOB_OBJECT_UILIMIT_EXITWINDOWS,
    JOB_OBJECT_UILIMIT_GLOBALATOMS, JOB_OBJECT_UILIMIT_HANDLES, JOB_OBJECT_UILIMIT_READCLIPBOARD,
    JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS, JOB_OBJECT_UILIMIT_WRITECLIPBOARD,
    JOBOBJECT_ASSOCIATE_COMPLETION_PORT, JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION,
    JOBOBJECT_BASIC_UI_RESTRICTIONS, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOBOBJECTINFOCLASS,
    JobObjectAssociateCompletionPortInformation, JobObjectBasicAndIoAccountingInformation,
    JobObjectBasicUIRestrictions, JobObjectExtendedLimitInformation, QueryInformationJobObject,
    SetInformationJobObject, TerminateJobObject,
};

use super::last_error;
use crate::SandboxError;

pub(crate) struct JobLimits {
    pub cpu_time: Duration,
    pub memory_bytes: u64,
    pub max_processes: u32,
}

pub(crate) struct Accounting {
    pub user_time: Duration,
    pub kernel_time: Duration,
    pub total_processes: u32,
    pub active_processes: u32,
}

pub(crate) struct Job {
    job: OwnedHandle,
    port: OwnedHandle,
}

fn owned(h: HANDLE, op: &'static str) -> Result<OwnedHandle, SandboxError> {
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return Err(last_error(op));
    }
    // SAFETY: `h` is a fresh, valid handle that nothing else owns.
    Ok(unsafe { OwnedHandle::from_raw_handle(h) })
}

/// 100-nanosecond ticks, the unit Windows uses for job times.
fn ticks(d: Duration) -> i64 {
    i64::try_from(d.as_nanos() / 100).unwrap_or(i64::MAX)
}

fn from_ticks(t: i64) -> Duration {
    Duration::from_nanos(u64::try_from(t).unwrap_or(0).saturating_mul(100))
}

impl Job {
    pub(crate) fn new(limits: &JobLimits) -> Result<Self, SandboxError> {
        // SAFETY: null attributes and name create an anonymous, non-inheritable job.
        let job = owned(
            unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) },
            "CreateJobObjectW",
        )?;
        // SAFETY: INVALID_HANDLE_VALUE + null existing port creates a new port.
        let port = owned(
            unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, null_mut(), 0, 1) },
            "CreateIoCompletionPort",
        )?;
        let this = Job { job, port };

        let assoc = JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
            CompletionKey: null_mut(),
            CompletionPort: this.port.as_raw_handle(),
        };
        this.set(
            JobObjectAssociateCompletionPortInformation,
            &assoc,
            "associate completion port",
        )?;

        // SAFETY: plain-old-data struct; all-zero is a valid "no limits" value.
        let mut ext: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        ext.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_JOB_MEMORY
            | JOB_OBJECT_LIMIT_JOB_TIME
            | JOB_OBJECT_LIMIT_ACTIVE_PROCESS
            | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
            | JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION;
        // Breakaway flags are deliberately absent: no process may leave the job.
        ext.BasicLimitInformation.PerJobUserTimeLimit = ticks(limits.cpu_time);
        ext.BasicLimitInformation.ActiveProcessLimit = limits.max_processes;
        ext.JobMemoryLimit = usize::try_from(limits.memory_bytes).unwrap_or(usize::MAX);
        this.set(JobObjectExtendedLimitInformation, &ext, "set job limits")?;

        let ui = JOBOBJECT_BASIC_UI_RESTRICTIONS {
            UIRestrictionsClass: JOB_OBJECT_UILIMIT_DESKTOP
                | JOB_OBJECT_UILIMIT_DISPLAYSETTINGS
                | JOB_OBJECT_UILIMIT_EXITWINDOWS
                | JOB_OBJECT_UILIMIT_GLOBALATOMS
                | JOB_OBJECT_UILIMIT_HANDLES
                | JOB_OBJECT_UILIMIT_READCLIPBOARD
                | JOB_OBJECT_UILIMIT_WRITECLIPBOARD
                | JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS,
        };
        this.set(JobObjectBasicUIRestrictions, &ui, "set UI restrictions")?;
        Ok(this)
    }

    fn set<T>(
        &self,
        class: JOBOBJECTINFOCLASS,
        info: &T,
        op: &'static str,
    ) -> Result<(), SandboxError> {
        // SAFETY: `info` is the struct matching `class` and lives for the call.
        let ok = unsafe {
            SetInformationJobObject(
                self.job.as_raw_handle(),
                class,
                info as *const T as *const _,
                size_of::<T>() as u32,
            )
        };
        if ok == 0 { Err(last_error(op)) } else { Ok(()) }
    }

    fn query<T>(&self, class: JOBOBJECTINFOCLASS) -> Option<T> {
        // SAFETY: T is the plain-old-data struct matching `class`; zeroed is valid and
        // the call writes at most size_of::<T>() bytes.
        unsafe {
            let mut out: T = std::mem::zeroed();
            let ok = QueryInformationJobObject(
                self.job.as_raw_handle(),
                class,
                &mut out as *mut T as *mut _,
                size_of::<T>() as u32,
                null_mut(),
            );
            (ok != 0).then_some(out)
        }
    }

    pub(crate) fn raw(&self) -> HANDLE {
        self.job.as_raw_handle()
    }

    pub(crate) fn terminate(&self, exit_code: u32) {
        // SAFETY: valid job handle. Failure only means the tree is already gone.
        unsafe { TerminateJobObject(self.job.as_raw_handle(), exit_code) };
    }

    /// Next job notification (a `JOB_OBJECT_MSG_*` id), or None on timeout.
    pub(crate) fn next_message(&self, timeout: Duration) -> Option<u32> {
        let mut msg = 0u32;
        let mut key = 0usize;
        let mut overlapped: *mut OVERLAPPED = null_mut();
        let ms = u32::try_from(timeout.as_millis()).unwrap_or(u32::MAX - 1);
        // SAFETY: valid port; out-pointers are valid locals. For job notifications
        // `overlapped` carries a process id, not a pointer, and we never dereference it.
        let ok = unsafe {
            GetQueuedCompletionStatus(
                self.port.as_raw_handle(),
                &mut msg,
                &mut key,
                &mut overlapped,
                ms,
            )
        };
        (ok != 0).then_some(msg)
    }

    pub(crate) fn accounting(&self) -> Accounting {
        let info: Option<JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION> =
            self.query(JobObjectBasicAndIoAccountingInformation);
        match info {
            Some(i) => Accounting {
                user_time: from_ticks(i.BasicInfo.TotalUserTime),
                kernel_time: from_ticks(i.BasicInfo.TotalKernelTime),
                total_processes: i.BasicInfo.TotalProcesses,
                active_processes: i.BasicInfo.ActiveProcesses,
            },
            None => Accounting {
                user_time: Duration::ZERO,
                kernel_time: Duration::ZERO,
                total_processes: 0,
                active_processes: 0,
            },
        }
    }

    pub(crate) fn peak_memory(&self) -> u64 {
        self.query::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>(JobObjectExtendedLimitInformation)
            .map_or(0, |i| i.PeakJobMemoryUsed as u64)
    }
}

// Dropping `Job` closes the job handle; KILL_ON_JOB_CLOSE then terminates any
// process still in it, even if the runner panics.

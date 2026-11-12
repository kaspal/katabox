; KataBox NSIS installer hooks (ADR 0021), included by Tauri's installer template.

; Before the files are removed: revoke the sandbox's folder grants and delete the
; AppContainer profile through kb-runner's cleanup (`KataBox.exe --uninstall-cleanup`).
; Skipped when an update runs the old uninstaller (`/UPDATE`): the next version keeps
; using the same sandbox. User data is removed only by Tauri's own "Delete the
; application data" checkbox (off by default).
!macro NSIS_HOOK_PREUNINSTALL
  ${If} $UpdateMode <> 1
    ${If} ${FileExists} "$INSTDIR\${MAINBINARYNAME}.exe"
      ExecWait '"$INSTDIR\${MAINBINARYNAME}.exe" --uninstall-cleanup'
    ${EndIf}
  ${EndIf}
!macroend

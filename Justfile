set shell := ["powershell", "-NoProfile", "-Command"]

install_dir := "${env:LOCALAPPDATA}\\GamepadOnSteamOn"
exe_path := install_dir + "\\gamepad-on-steam-on.exe"

# Admin privilege required
default: install-task

# Admin privilege required
install-task:
    cargo build --release
    New-Item -ItemType Directory -Force -Path "{{install_dir}}" | Out-Null
    Copy-Item -Force "${PWD}\target\release\gamepad-on-steam-on.exe" "{{exe_path}}"
    
    $Action = New-ScheduledTaskAction -Execute "{{exe_path}}"; \
    $Trigger = New-ScheduledTaskTrigger -AtLogOn; \
    $Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit 0; \
    Register-ScheduledTask -TaskName "GamepadOnSteamOn" -Action $Action -Trigger $Trigger -Settings $Settings -RunLevel Highest -Force
    
    Start-ScheduledTask -TaskName "GamepadOnSteamOn"

# Admin privilege required
uninstall-task:
    -schtasks /Delete /F /TN "GamepadOnSteamOn"
    -Get-Process -Name "gamepad-on-steam-on" -ErrorAction SilentlyContinue | Stop-Process -Force
    -Remove-Item -Recurse -Force "{{install_dir}}"

open-exe-dir:
    explorer "{{install_dir}}"

status-task:
    & { try { $task = Get-ScheduledTask -TaskName "GamepadOnSteamOn" -ErrorAction Stop; $info = $task | Get-ScheduledTaskInfo; Write-Host "Task: GamepadOnSteamOn (State: $($info.State), LastRunTime: $($info.LastRunTime), LastResult: $($info.LastTaskResult))" } catch { Write-Host "Task: GamepadOnSteamOn (not found)" }; $procs = Get-Process -Name "gamepad-on-steam-on" -ErrorAction SilentlyContinue; if ($null -eq $procs) { Write-Host "Process: gamepad-on-steam-on (not running)" } else { $lines = $procs | ForEach-Object { $cpu = if ($_.CPU -ne $null) { [math]::Round([double]$_.CPU, 2) } else { 0 }; $ws = [math]::Round(([double]$_.WorkingSet64 / 1MB), 1); "PID: $($_.Id), CPU: ${cpu}s, WS: ${ws}MB" }; Write-Host ("Process: gamepad-on-steam-on (running: " + ($lines -join "; ") + ")") } }


set shell := ["powershell", "-NoProfile", "-Command"]

install_dir := "${env:LOCALAPPDATA}\\GamepadOnSteamOn"
exe_path := install_dir + "\\gamepad-on-steam-on.exe"

# Admin privilege required
default: install-task

# Admin privilege required
install-task:
    cargo build --release
    New-Item -ItemType Directory -Force -Path "{{install_dir}}" | Out-Null
    Copy-Item -Force "${PWD}\\target\\release\\gamepad-on-steam-on.exe" "{{exe_path}}"
    schtasks /Create /F /RL LIMITED /SC ONLOGON /TN "GamepadOnSteamOn" /TR "{{exe_path}}"

# Admin privilege required
uninstall-task:
    schtasks /Delete /F /TN "GamepadOnSteamOn"
    Remove-Item -Recurse -Force "{{install_dir}}"


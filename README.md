# Gamepad on Steam on

gamepad-on-steam-on is a small and lightweight exe and a scheduled task that runs on Windows startup, which will automatically open Steam in background when a gamepad is connected (if Steam is not already running).

## How to setup

### Pre-requisite

- [cargo](https://rust-lang.org/tools/install/)
- [just](https://github.com/casey/just)

### Run

Open terminal with admin privilege

```bash
    # clone repo
    git clone

    # install the scheduled task
    just install-task

    # check if the task is runing successfully
    just status-task
```

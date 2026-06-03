# Gamepad on Steam on

gamepad-on-steam-on is a small and lightweight exe and a scheduled task that runs on Windows startup, which will automatically open Steam in background when a gamepad is connected (if Steam is not already running).

<video
    src="https://github.com/user-attachments/assets/b8ca0b25-20fe-444e-a56e-b313d7cff3b5"
    autoplay
    >
</video>

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

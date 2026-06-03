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
- [cargo-wix](https://github.com/volks73/cargo-wix)

### Run

Open terminal with admin privilege

```bash
    # clone repo
    git clone https://github.com/JoseIgnacioGC/gamepad-on-steam-on.git

    # build code and execute msi installer
    cargo wix --install
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

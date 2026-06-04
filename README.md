# Gamepad on Steam on

`gamepad-on-steam-on` is a small and lightweight background utility that runs silently on Windows startup. It automatically opens Steam in the background whenever a gamepad is connected (if Steam is not already running).

<video
    src="https://github.com/user-attachments/assets/b8ca0b25-20fe-444e-a56e-b313d7cff3b5"
    autoplay
    >
</video>

## Install

Download the latest `.msi` release from [releases](https://github.com/JoseIgnacioGC/gamepad-on-steam-on/releases) (or build and install it yourself, see [below](#how-to-setup)).

## Uninstall

1.  "Installed apps" on Windows 11.

1.  Search for gamepad-on-steam-on.

1.  Then uninstall it.

## How to setup

### Pre-requisite

- [git](https://git-scm.com/downloads)
- [cargo](https://rust-lang.org/tools/install/)
- [cargo-wix](https://github.com/volks73/cargo-wix)

### Build and install

Open terminal with admin privilege

```bash
    # clone repo
    git clone https://github.com/JoseIgnacioGC/gamepad-on-steam-on.git

    # build code and execute msi installer
    cargo wix --install
```

Run and debug code

```bash
    cargo run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

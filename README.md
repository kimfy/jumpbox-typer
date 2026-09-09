# Jumpbox Typer

Small tool for typing through jump hosts and remote sessions that block clipboard paste.

The app supports Ubuntu 26.04. macOS support is experimental until remote-client validation is complete.

![Image of the app](docs/jumpbox-typer-app.png)

## What It Does

- Types pasted text into remote sessions where clipboard paste is blocked
- Lets you extract text from a screenshot of a jumphost or terminal and turn it into plain text when copy/paste back to the host is blocked

## Install

Use the local installer:

```bash
./install.sh
```

See [BUILD.md](BUILD.md) for build and packaging notes.

On macOS, the installer creates a local Homebrew-linked application in `~/Applications` by default. It does not create a portable release.

The app stores typing preferences in `~/.config/jumpbox-typer/config.txt` by default, or under `XDG_CONFIG_HOME` when set. If that file is missing, the built-in defaults are used.

## Safety

The app sends real keystrokes to whichever window is focused when the delay ends. Test with harmless text first.

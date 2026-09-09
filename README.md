# Jumpbox Typer

Small tool for typing through jump hosts and remote sessions that block clipboard paste.

The app supports Ubuntu 26.04. macOS typing is experimental until remote-client validation is complete. The macOS build boundary is macOS 14 or later on Apple Silicon.

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

## Keyboard-event permission on macOS

Jumpbox Typer checks permission to post keyboard events when it starts, but startup does not request access. If access is missing, Start remains disabled.

Choose Check System to start the user-initiated permission flow. Grant access in macOS System Settings, then return to Jumpbox Typer. The app checks readiness again when it becomes active. Choose Check System again if the status does not update. Replacing an ad-hoc signed development build can require granting permission again.

## Keyboard layouts on macOS

Select the same keyboard layout in Jumpbox Typer as the active macOS input source. The US layout guarantees physical key mappings for printable US ASCII. The Norwegian layout guarantees physical key mappings for Norwegian letters and the layout-specific symbols supported by the Linux backend.

Other characters use a best-effort Core Graphics Unicode keyboard event. This fallback sends UTF-16, including surrogate pairs, but target applications and remote clients can reject or change the text. Arbitrary Unicode input is not guaranteed.

## OCR on macOS

Install Tesseract with Homebrew before building or using clipboard-image OCR. Finder-launched apps often receive a smaller `PATH` than terminal apps, so Jumpbox Typer first searches inherited `PATH` and then the standard Apple Silicon and Intel Homebrew binary locations. System checks and OCR use the same resolved Tesseract executable.

## Safety

The app sends real keystrokes to the focused window when the delay ends. Later focus changes redirect remaining text to the newly focused window. Stop cancels after the current character sequence; it cannot retract text already sent. Test with harmless text first.

## macOS compatibility

Only environments that pass the manual compatibility procedure belong in this matrix. No remote client has passed validation, so no remote client is currently claimed as compatible.

| Target client | Client version | macOS version | Hardware | Input source | Result and limitations |
| --- | --- | --- | --- | --- | --- |
| No validated environment recorded | — | — | — | — | Experimental; no compatibility claim |

## macOS support boundary

The current application bundle is for local use on the Mac that built it. It links to Homebrew GTK and libadwaita libraries and has only an ad-hoc signature. It is not a portable Developer ID-signed or notarized public release.

Public distribution, Intel support, guaranteed arbitrary Unicode input, a native macOS UI redesign, and App Store work remain out of scope. Untested remote clients are not supported by implication.

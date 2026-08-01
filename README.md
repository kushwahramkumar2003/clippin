# ClipPin

Lightweight macOS menu-bar clipboard history manager. Local-first, native AppKit (Rust), no Electron.

**Requires:** macOS 13+, [Rust](https://rustup.rs/), Xcode Command Line Tools.

---

## Install

### One-liner (recommended)

Installs ClipPin as a standalone binary in `~/.cargo/bin` (independent of this repo):

```bash
cargo install --git https://github.com/kushwahramkumar2003/clippin.git --locked
```

Make sure Cargo’s bin directory is on your `PATH` (usually already is after installing Rust):

```bash
# If `clippin` is not found after install:
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### From a local clone

```bash
git clone https://github.com/kushwahramkumar2003/clippin.git
cd clippin
cargo install --path . --locked
```

### Run

**Detached (recommended)** — starts independently; you can close the terminal:

```bash
clippin --detach
# or
clippin -d
```

**Foreground** — keeps the terminal attached (useful for debugging):

```bash
clippin
```

ClipPin lives in the **menu bar** (no Dock icon).

**Launch at login** (Settings checkbox) installs a user LaunchAgent that starts ClipPin with `--detach` — no Terminal window at login.

| | |
|--|--|
| **Stop** | Quit from the popover, or `pkill -x clippin` |
| **Logs** | `~/Library/Logs/ClipPin/clippin.log` |
| **Help** | `clippin --help` |

### Update

```bash
cargo install --git https://github.com/kushwahramkumar2003/clippin.git --locked --force
```

### Uninstall

```bash
cargo uninstall clippin
# Optional: remove history & settings
rm -rf ~/Library/Application\ Support/com.clippin.app
# Optional: remove login agent if you used Launch at login (dev binary)
rm -f ~/Library/LaunchAgents/com.clippin.app.plist
launchctl bootout "gui/$(id -u)/com.clippin.app" 2>/dev/null || true
```

---

## Features

- Menu bar history (text, RTF, HTML, images, files, URLs)
- Search, pin, multi-select delete, clear unpinned
- Global hotkey (default **⌘⇧V**)
- Optional auto-paste (needs Accessibility permission)
- Privacy-aware (skips password-manager concealed/transient pasteboard types)
- Launch at login
- Local SQLite storage under `~/Library/Application Support/com.clippin.app/`

---

## Usage

| Action               | How                                                        |
| -------------------- | ---------------------------------------------------------- |
| Open                 | Click menu bar icon, or **⌘⇧V**                            |
| Search               | Type in the search field                                   |
| Copy item            | Click a row                                                |
| Pin / unpin / delete | Right-click a row                                          |
| Settings             | Gear icon in the popover                                   |
| Auto-paste           | Enable in the toolbar; grant Accessibility when macOS asks |

---

## Permissions

| Permission                   | Why                                            |
| ---------------------------- | ---------------------------------------------- |
| **Accessibility**            | Auto-paste simulates ⌘V into the frontmost app |
| **Login item / LaunchAgent** | Launch at login                                |

Grant Accessibility under:

**System Settings → Privacy & Security → Accessibility**

---

## License

MIT — see [LICENSE](LICENSE).

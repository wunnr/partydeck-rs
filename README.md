<img src=".github/assets/icon.png" align="left" width="100" height="100">

### `PartyDeck`

A split-screen game launcher for Linux/SteamOS

---

<img src=".github/assets/launcher.png" align="center">

> [!IMPORTANT]
> This is the first serious software project I've ever done. It surely contains many violations of software best practices and security flaws; use at your own discretion! If you are experienced in software I would love to know what aspects of the codebase could be improved and how I can do better.

## Features

- Runs up to 6 instances of a game at a time and automatically fits each game window onto the screen
- Supports native Linux games as well as Windows games through Proton
- Handler system that tells the launcher how to handle game files, meaning very little manual setup is required
- Steam multiplayer API is emulated, allowing for multiple instances of Steam games
- Works with most game controllers without any additional setup, drivers, or third-party software
- Uses sandboxing software to mask out controllers so that each game instance only detects the controller assigned to it, preventing input interference
- Profile support allows each player to have their own persistent save data, settings, and stats for games

## Installing & Usage

Download the latest release [here](https://github.com/wunnr/partydeck-rs/releases). Download game handlers [here](https://drive.proton.me/urls/D9HBKM18YR#zG8XC8yVy9WL). Extract the zip into a folder.

This project uses KWin (included with KDE Plasma), Gamescope, and Bubblewrap; SteamOS already includes these, but if you're on a desktop Linux distro you may need to install these yourself. On Arch, these packages are `kwin`, `gamescope`, and `bubblewrap`.

If you're already running a KDE Plasma session, you can simply run the executable `partydeck-rs` to get started. If you're on Steam Deck and want to access PartyDeck from Gaming Mode, simply add `PartyDeckKWinLaunch.sh` as a non-Steam game by right-clicking that file and selecting "Add to Steam". This is a simple script that launches a KWin session from within Gaming Mode, then runs PartyDeck inside of that session.

**IMPORTANT:** Steam Input interferes with PartyDeck's controller detection, causing issues like duplicate controllers showing up! If PartyDeck is added as a non-Steam game, go to the properties in Steam and make sure that Steam Input is disabled. If Steam is open at all, make sure that your controllers aren't using a Desktop layout.

Any controller that SDL supports is theoretically supported: Specifically, Xbox/PlayStation/Switch controllers have all been tested with PartyDeck. Some native Linux games use older versions of SDL that have incorrect mappings with newer controllers, so the launcher has an option to force a game to use the Steam Runtime's version of SDL, which usually fixes this.

On first launch, the app will automatically download UMU Launcher and Goldberg Steam Emu. This may take a while depending on your download speed, but it only needs to be done once.

> [!NOTE]
> **SteamOS Users:** This app requires KDE Plasma 6 for the KWin split-screen. The current stable version of SteamOS still uses Plasma 5, but for now you can update to the Preview channel in the system settings to get Plasma 6.

Note that you'll also need a Handler to actually run a game; These will be uploaded to a separate repository, and eventually the project will include a program that helps you generate your own Handler.

## Building

You'll need a Rust toolchain installed with the 2024 Edition. Clone the repo, and run `build.sh`. This will place the executable, as well as the relevant data files, into the "build" folder.


## How it Works

PartyDeck uses a few software layers to provide a console-like split-screen gaming experience:

- **KWin Session:** This KWin Session displays all running game instances and runs a script to automatically resize and reposition each Gamescope window.
- **Gamescope:** Contains each instance of the game to its own window. Also has the neat side effect of receiving controller input even when the window is not currently active, meaning multiple Gamescope instances can all receive input simultaneously
- **Bubblewrap:** Uses bindings to mask out evdev input files from the instances, so each instance only receives input from one specific controller. Also uses directory binding to give each player their own save data and settings within the games.
- **Runtime (Steam Runtime/Proton):** If needed, the app can run native Linux games through a Steam Runtime (currently, 1.0 (scout) and 2.0 (soldier) are supported) for better compatibility. Windows games are launched through UMU Launcher
- **Goldberg Steam Emu:** On games that use the Steam API for multiplayer, Goldberg is used to allow the game instances to connect to each other, as well as other devices running on the same LAN.
- **And finally, the game itself.**

## Known Issues, Limitations and To-dos

- AppImages and Flatpaks are not supported yet for native Linux games. Handlers can only run regular executables inside folders.
- "Console-like splitscreen experience" means single-screen and controllers only. Multi-monitor support is possible but will require a better understanding of the KWin Scripting API. Support for multiple keyboards and mice is also theoretically possible, but I'll have to look into how I would go about implementing it.
- The launcher is built synchronously, meaning there isn't any visual indicators of progress or loading when things are happening, it will just freeze up. This obviously isn't ideal.
- Controller navigation support in the launcher is super primitive; I'd love to try making a more controller-friendly, Big-Picture-style UI in the future, but have no immediate plans for it.
- Games using Goldberg might have trouble discovering LAN games from other devices. If this happens, you can try adding a firewall rule for port 47584. If connecting two Steam Decks through LAN, their hostnames should be changed from the default "steamdeck".

## Credits/Thanks

- MrGoldberg & Detanup01 for [Goldberg Steam Emu](https://github.com/Detanup01/gbe_fork/)
- GloriousEggroll and the rest of the contributors for [UMU Launcher](https://github.com/Open-Wine-Components/umu-launcher)
- Inspired by [Tau5's Coop-on-Linux](https://github.com/Tau5/Co-op-on-Linux) and [Syntrait's Splinux](https://github.com/Syntrait/splinux)
- Talos91 and the rest of the Splitscreen.me team for [Nucleus Coop](https://github.com/SplitScreen-Me/splitscreenme-nucleus), and for helping with handler creation

## Disclaimer
This software has been created purely for the purposes of academic research. It is not intended to be used to attack other systems. Project maintainers are not responsible or liable for misuse of the software. Use responsibly.

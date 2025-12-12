# pipewire-eq-switcher

## Description
I make music. I listen to music. I like tuning my headphones and earbuds. Pipewire has the incredible ability to create equalizer sinks for your output, but no easy way to switch them around. The solution? This script! For more information on how to set up and where to put your equalization configurations, check the _Use_ section below.

## Installation

### Dependencies
I believe all the dependencies you need (besides, obviously, Sway and Pipewire) are:
- Mako
- Rust

### Arch Linux
You can download the PKGBUILD from [this repository](https://github.com/jefrecantuledesma/pkgbuilds/tree/main/pipewire-eq-switcher). Then, you can issue `makepkg -si` and you should be good to go!

### Other Distributions
You can `git clone https://github.com/jefrecantuledesma/pipewire-eq-switcher`, enter that directory, issue the command `cargo build --release`, enter the directory `./target/release`, and place that binary in `/usr/bin` or wherever is deemed appropriate.

## Use

This is a script I've made for myself, and it may not be perfect for your configuration. In order for this script to work, you must have all of your EQ configurations stored in your `~/.config/pipewire/pipewire.conf.d/` folder.

### Important File Naming Convention
- **Default EQ** (the active one): Must be named `sink-eq6.conf` with the `.conf` extension
- **Alternative EQs** (the ones you switch between): Must have **NO** file extension at all

For example, your directory might look like this:
```
~/.config/pipewire/pipewire.conf.d/
├── sink-eq6.conf          ← Active EQ (has .conf extension)
├── bass-boost             ← Alternative EQ (no extension)
├── classical              ← Alternative EQ (no extension)
└── vocals                 ← Alternative EQ (no extension)
```

Each configuration file needs a `media.name` field so the script can display a friendly name. For example:
```
media.name = "Bass Boost"
```

To use the script, simply issue the command `pipewire-eq-switcher`, select your desired EQ configuration from the numbered menu, and enjoy! The script will copy your selection over `sink-eq6.conf` and reload Pipewire for you. Type `q` to quit without making changes.

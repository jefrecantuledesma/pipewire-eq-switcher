# pipewire-eq-switcher
## Description
I make music. I listen to music. I like tuning my headphones and earbuds. Pipewire has the incredible ability to create equalizer sinks for your output, but no easy way to switch them around. The solution? This script! For more information on how to rename and where to put your equalization configurations, check the _Use_ section below. 

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
This is a script I've made for myself, and it may not be perfect for your configuration. In order for this script to work, you must have all of your EQs stored in your 
`~/.config/pipewire/pipewire.conf.d/` folder. EQs that are **NOT** the default EQ must have no file extension. The default EQ **MUST** be named `sink-eq6.conf`. Besides that,
everything should work!

To use the script, simply issue the command `pipewire-eq-switcher`, select your desired EQ configuration, and enjoy! The script will reload Pipewire for you.

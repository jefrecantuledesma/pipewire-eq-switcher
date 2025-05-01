# pipewire-eq-switcher
## Use
This is a script I've made for myself, and it may not be perfect for your configuration. In order for this script to work, you must have all of your EQs stored in your 
`~/.config/pipewire/pipewire.conf.d/` folder. EQs that are **NOT** the default EQ must have no file extension. The default EQ **MUST** be named `sink-eq6.conf`. Besides that,
everything should work!

To use the script, simply issue the command `pipewire-eq-switcher`, select your desired EQ configuration, and enjoy! The script will reload Pipewire for you.

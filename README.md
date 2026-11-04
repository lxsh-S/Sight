# Sight

<img width="899" height="1076" alt="2026-10-27-212605_impasto" src="https://github.com/user-attachments/assets/c84ee446-6705-40ee-bd53-ce2b02e88e8e" />

An app launcher written in rust.

## Overview

`Sight` is a simple lightweight gtk launcher that searches for applications on your linux machine and lets you run whatever application you choose to from the app menu by searching or moving through with the arrow keys (just like how app launchers work!)

Sight uses `relm4` with `GTK4` for it's UI :)

## Build

Need rust and cargo installed to build

```
git clone https://github.com/lxsh-S/Sight.git
cd Sight
cargo build
```

## INSTALLATION
> Note: `sight` is named as `sight-launcher` when installing using cargo  
- Currently `sight` can only be installed via cargo: https://crates.io/crates/sight-launcher (or)
```
cargo install sight-launcher
```


## Configure with Hyprland
Add this to your autostart.lua or `hyprland.lua` if you wanna keep things in one file:
- set a variable
```
local menu = "sight-launcher"
```
- Add a keybind `:P`
```
hl.bind(mainMod .. " + R", hl.dsp.exec_cmd(menu))
```
- You also would need to write a window rule for the launcher
```
overlayLayerRule:set_enabled(false)

hl.window_rule({
    name = "sight-launcher",
    match = {
        class = "^(dev\\.lxsh\\.sight)$",
    },

    float = true,
    center = true,
    size = "500 600",
    stay_focused = true,
    pin = true,
})
```


## Configure with Gnome and Kde

I recommend:

Setting | Value |
|---|---|
| Name | Sight |
| Command | `sight-launcher` |
| Shortcut | `Super + R` | 

Feel free to customize the shortcut keys!

### Gnome
- Open `settings` -> `keyboard`
- find `View and customize shortcuts` -> `Custom shortcuts`
- Add a new shortcut
![Uploading Screenshot From 2026-11-04 20-27-16.png…]()

### Kde Plasma
- Open `System settings` -> `keyboard` -> `shortcuts`
- Add a new `command` shortcut
![Uploading Screenshot From 2026-11-03 21-50-40.png…]()

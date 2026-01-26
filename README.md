<h1 align="center">Constellate 🌌</h1>
<h6 align="center"> link and control configuration files across your system </h6>

### Current features:
- 📎 Store config templates and copy them to target directories
- 🖌️ Set variables in your templates and easily their values across all configs
- 🖥️ Choose a 'theme' file to apply to all templates
- ❄️ Nix flake

### Planned features:
- 🏠 home-manager integration

If you have any suggestions, please open an issue.

## Installation
Add 
```nix
constellate = {
  url = "github:averyfrog/constellate";
};
```
to your flake inputs.

Next, add
```nix
inputs.constellate.packages.${system}.default
```
to your `environment.systemPackages`.

Done! you can now run the program using `constellate`.

## Instructions  

This will be a quick (and likely awful) rundown on how to get a config up and running.

### Setup

It is recommended to have your `.config/constellate` layed out like this:
```
/ .
├── / scripts
│   └── . qt.sh
├── / templates
│   ├── . kitty-colors.conf
│   └── . qt-colors.colors
├── / themes
│   ├── . catppuccin-latte.toml
│   ├── . catppuccin-mocha.toml
│   └── . rose-pine-dawn.toml
└── . templates.toml
```

An example templates file: 
```toml
# templates.toml

[kitty]
input = ".config/constellate/templates/kitty-colors.conf"
output = ".config/kitty/colors.conf"
hook = "kill -SIGUSR1 $(pgrep kitty)"

[qt6]
input = ".config/constellate/templates/qt-colors.colors"
output = ".local/share/color-schemes/constellate.colors"
hook = "/home/$USER/.config/constellate/scripts/qt.sh"
```
A 'hook' is a bash command ran once the output file is successfully placed.

An example theme file:

```toml
# themes/catppuccin-mocha.toml

base00 = "1e1e2e" # base
base01 = "181825" # mantle
base02 = "313244" # surface0
base03 = "45475a" # surface1
base04 = "585b70" # surface2
base05 = "cdd6f4" # text
base06 = "f5e0dc" # rosewater
base07 = "b4befe" # lavender
base08 = "f38ba8" # red
base09 = "fab387" # peach
base0A = "f9e2af" # yellow
base0B = "a6e3a1" # green
base0C = "94e2d5" # teal
base0D = "89b4fa" # blue
base0E = "cba6f7" # mauve
base0F = "f2cdcd" # flamingo

accent = "f9e2af"
text = "cdd6f4"

rounding = "8"
```

Configuration:
   
Now that you have the basic setup down, you can configure individual apps within your 'templates' directory. This is quite simple, you create a file for each app and then set values to certain variables, i.e:

```conf
# templates/kitty-colors.conf

cursor #{{text}}
cursor_text_color #{{base00}}

foreground            #{{text}}
background            #{{base00}}
selection_foreground  #{{base00}}
selection_background  #{{base06}}
url_color             #{{base0C}}

# black
color8   #{{base02}}
color0   #{{base02}}

...
```
### Using

- run `constellate theme [theme-name]` and all your configs will be updated using the matching `themes/theme-name.toml`. (Ooh shiny!) 
- `constellate var [theme-name] [variable-name]` can also be used to quickly grab a value from a theme without applying it.
- `constellate list` will show all available themes.

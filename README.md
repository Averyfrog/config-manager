<h1 align="center">Config-Manager</h1>



### Current features:
- 📎 Store config templates and copy them to target directories
- 🖌️ Set variables in your templates and easily their values across all configs
- ❄️ Nix flake

### Planned features:
- 🏠 home-manager integration
- 🖥️ Proper CLI (changing values, changing defaults)

If you have any suggestions, please open an issue.

## Installation
Add 
```nix
config-manager = {
  url = "github:averyfrog/config-manager";
};
```
to your flake inputs.

Next, add
```nix
inputs.config-manager.packages.${system}.default
```
to your `environment.systemPackages`.

Done! you can now run the program using `config-manager`.

## Instructions   

Welcome to my very amazing instructions that I definitely didn't write at 1AM while sleep deprived purely because I decided that I wanted to share this project with people!

This will be a quick (and likely awful) rundown on how to get a config up and running.

Setup:

You want to create a 'config-manger' folder in ur '.config' path and a 'templates' folder within that 'config-manager' folder. Then you want a 'templates.toml' file in your 'config-manager' folder. In ur 'templates.toml' file you want to create a group for each app to theme i.e:

[hyprland]
input = ".config/config-manager/templates/hyprland-theme.conf"
output = ".config/hypr/theme.conf"

You can also create a 'hook' for each app which is a bash command that will be run once the file is successfully in the directory.

In your 'config-manager' folder you will also want a 'defaults.toml' for defining colors i.e:

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

wallpaper = "/home/user/Pictures/wallpapers/wallpaper.png"

Configuration:
   
Now that you have the basic setup down, you can configure individual apps within your 'templates' directory. This is quite simple, you create a file for each app and then set values to certain variables, i.e:

cursor #{text}
cursor_text_color #{base00}

foreground            #{text}
background            #{base00}
selection_foreground  #{base00}
selection_background  #{base06}
url_color             #{base0C}

# black
color8   #{base02}
color0   #{base02}

# red
color1   #{base08}
color9   #{base08}

# green
color2   #{base0B}
color10  #{base0B}

# yellow
color3   #{base0A}
color11  #{base0A}

# blue
color4  #{base0D}
color12 #{base0D}

# magenta
color5   #{base08}
color13  #{base08}

# cyan
color6   #{base0C}
color14  #{base0C}

# white
color15  #{base06}
color7   #{base06}

Now you should have a basic config-manager setup, run 'config-manager' in your terminal and watch as everything has new colors ooh shiny! 

(this was written by @adairaa to explain why it's so shit) 

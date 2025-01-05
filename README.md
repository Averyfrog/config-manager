<h1 align="center"> config-manager </h1>

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

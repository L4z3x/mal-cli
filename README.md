<div align="center">

![Build](https://img.shields.io/github/actions/workflow/status/L4z3x/mal-tui/rust.yml) ![Crates.io](https://img.shields.io/crates/v/mal-cli-rs) ![License](https://img.shields.io/github/license/L4z3x/mal-tui) ![Stars](https://img.shields.io/github/stars/L4z3x/mal-tui?style=social)

</div>

# MAL-Cli
🎌 A fast, keyboard-driven terminal client for  [MyAnimeList](https://myanimelist.net/)  – built with Rust and Ratatui.


## Note:
  - for rendering images use a gpu-enhanced terminal like kitty, and for windows use windows terminal >1.22 
  - forked from [SaeedAnas/mal-cli](https://github.com/SaeedAnas/mal-cli) (last commit 5 years ago)

# Demo:
<div align="center">

![gif](./assets/demo.gif)

</div>

## Detail page

<div align="center">

![detail](./assets/mal-tui-manga-details-page.png)

</div>

# INSTALLATION:
## ArchLinux:
  ```
  yay -S mal-cli
  ```

## using cargo:
  ```
  cargo install mal-cli-rs
  ```

## Debian-based:
  download the package from last release and run:
  ```
  sudo dpkg -i <installed-packege>
  ```
  release section can be found here [here](https://github.com/L4z3x/mal-cli/releases/)

## windows/ macos / musl:
  download binaries from release section and run directly otherwise use cargo
##
# HOW TO GET CLIENT ID:
  visit [mal](https://myanimelist.net/apiconfig/create)
  and if you get an error, go to your profile -> profile settings -> api -> create
  ![image](./assets/mal-client-id-page.png)
  

## Main keys:
  - [s]: switching/opening popups
  - [r]: opening popups (when s does the switching)
  - [Ctrl+p]: forward navigation
  - [Esc]: backward navigation
  


# Debug:
in $HOME/.config/mal-tui/config.yml file:
   set show_logger to true
   set log_level to INFO

# TODO:
- [ ] add help section
- [ ] add delete entry endpoint
- [ ] fix double click on windows

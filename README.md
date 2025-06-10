# MAL-Cli

A terminal interface for the official [myanimelist](https://myanimelist.net/) api written in rust.

forked from [SaeedAnas/mal-cli](https://github.com/SaeedAnas/mal-cli) (last commit 5 years ago)
## Note:
for rendering images use a gpu-enhanced terminal like kitty, and for windows use windows terminal >1.22
# HOW IT LOOKS
## Detail page
![detail](./assets/mal-tui-manga-details-page.png)
## Home page
![home](./assets/mal-tui-home-screenshot-01.png)
## Profile page
![profile](./assets/mal-tui-profile-screenshot-03.png)
## Search page
![search](./assets/mal-tui-search-screenshot-02.png)

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
  release section can be found here ![here](https://github.com/L4z3x/mal-cli/releases/)

## windows/ macos / musl:
  download binaries from release section and run directly otherwise use cargo
##
# HOW TO GET CLIENT ID:
  visit ![mal](https://myanimelist.net/apiconfig/create)
  and if you get an error, go to your profile -> profile settings -> api -> create
  ![image](./assets/mal-client-id-page.png)
  

# Debug:
in $HOME/.config/mal-tui/config.yml file:
   set show_logger to true
   set log_level to INFO

# TODO:
- [ ] add help section

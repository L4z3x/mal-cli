# MAL-Cli

A terminal interface for the official [myanimelist](https://myanimelist.net/) api written in rust.

forked from [SaeedAnas/mal-cli](https://github.com/SaeedAnas/mal-cli)

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

## windows/ macos:
  download binaries from release section and run directly otherwise use cargo

# HOW TO GET CLIENT ID:
  visit ![mal](https://myanimelist.net/apiconfig/create)
  and if you get an error, go to your profile -> profile settings -> api -> create
  ![image](./assets/mal-client-id-page.png)
  

# TODO:
- [ ] add help section

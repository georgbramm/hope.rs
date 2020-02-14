# hope.rs

## A hope web application completely written in Rust

extended hOPE library, cli and server written in Rust.

``` console
HTTP Server actix-web   <- websocket ->     Hope 
                                              |
                                            MongoDB Server
```
### Website

1. [hope](https://github.com/georgbramm/hope.rs).

## Build

The following build dependencies needs to be fulfilled to support the full
feature set of this application:

- [cargo-web](https://github.com/koute/cargo-web)
- [TODO]

The app consist of a library, a frontend and a backend. For getting started with hacking,
the backend can be tested via `make run-backend`, whereas the frontend can be
tested with `make run-frontend`. You can adapt the application configuration
within `Config.toml` if needed.

This installs build requirements, rust and cargo-web, on Ubuntu or Debian.
``` console
wget https://sh.rustup.rs -O rustup-init
sudo sh rustup-init -y
sudo apt-get install -y pkg-config libssl-dev
sudo cargo install cargo-web
```
This builds the project.
``` console
git clone https://github.com/georgbramm/hope.rs.git
cd webapp.rs
make
```
## Run

This runs the project.
``` console
cd webapp.rs
make server
```

## Deploy

This deploys as docker containers:
This runs the project.
``` console
cd webapp.rs
make deploy
```
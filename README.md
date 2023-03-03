# Rusttery

Rust :crab: + Battery :battery: = Rusttery

Program to control battery charge threshold for Linux.

## Requirements

- `systemd`
- `dbus` session running 
- `make`
- `cargo`
- `git`

## :warning: IMPORTANT :warning:

Due to battery model diversity is possible that this program doesn´t work on your hardware.

At the moment of writing this, `Rusttery` can set threshold to batteries that have name syntax like `BAT[number]`. 

For example: `BAT0` (it seems the most common).

To check this you can run:

```shell
ls /sys/class/power_supply/
```

It should return something like this:

```shell
ls /sys/class/power_supply/
AC0  BAT0
```

## Install

First clone this repo and then cd into it.

```shell
git clone https://github.com/claaj/rusttery && cd rusttery
```

Now build the binaries (remember that `make` and `cargo` are required):

```shell
make build
```

With the binaries already built, now is possible to install.

```shell
sudo make install
```

Finally to run the app, you can type:

```shell
rusttery
```
![clip](https://user-images.githubusercontent.com/102485147/222772106-1bfe555a-785d-45bd-aa03-b893feff17ef.gif)

## Uninstall

To uninstall cd into the repo, wherever is located:

```shell
cd path/to/rusttery-repo
```

Then run:

```shell
sudo make uninstall
```



> This program is being tested on ASUS X515 laptop running Fedora 37 and Sway.

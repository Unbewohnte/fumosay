# fumosay
## Like cowsay, but with soft friends (ᗜˬᗜ)
```
    __        __       <⑨ There are no buses in Gensokyo ⑨>
   |  \      /  |    ()
   |   \____/   | ()
   |__// V  \\__|
    /|| ᗜ ᗜ || \
    \_\\_︶_//_/
       jj⑨lll
   ()_/@V@V@V\_()
     /@V@V@V@V\
     (_)-----(_)
```
---

# Installation

## "Compile it yourself" way
install Rust
- [installation instructions](https://www.rust-lang.org/tools/install)

clone this repository
- `git clone https://github.com/Unbewohnte/fumosay`

cd into cloned repo
- `cd fumosay/`

compile for your OS && Architecture
- `cargo build --release`

move compiled executable to the current directory
- `mv target/release/fumosay .` or `mv target/[target_name]/release/fumosay .`

**or**

## "Download a pre-compiled version (Linux amd64 only)" way
- [Download a version of your choice](https://github.com/Unbewohnte/fumosay/releases)

unzip
- `unzip [zip_archive_name]` or `7z x [7z_archive_name]`

cd into unzipped directory
- `cd fumosay/`

## After compilation|downloading

run installation script
- `chmod +x install.sh && sudo ./install.sh`
**binary file and fumofiles must be in the same directory as install.sh !**

**Now you have fumosay installed !**

---

# Use
```
fumosay message_here
```
prints a message with a default fumo.fumo template

```
fumosay -f mini.fumo message_here
```
prints a message with a mini.fumo template. You can add your own fumofiles
in /usr/share/fumosay/fumofiles/ and use them with -f flag

```
fumosay -d path/to/your/fumofiles/ -f your_fumo.fumo fumofumo
```
uses a non-default path to fumofiles and a custom fumofile

---

# TODO list

- Add more fumos
- ~~Make it possible to use other fumos~~
- Improve message {box|bubble}
- ~~Embed fumofiles or take care of ways of locating them~~
- ~~Make an `install.sh` script~~

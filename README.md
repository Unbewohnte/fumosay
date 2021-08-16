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

## Compile it yourself
install Rust
- [installation instructions](https://www.rust-lang.org/tools/install)

clone this repository
- `git clone https://github.com/Unbewohnte/fumosay`

cd into cloned repo
- `cd fumosay/`

compile for your OS && Architecture
- `cargo build --release`


**or**

## Download a pre-compiled version (only amd64 Linux and Windows)
- [Download a version of your choice](https://github.com/Unbewohnte/fumosay/releases)

## After compilation|downloading

create a directory where the program will 'sit' 
- `mkdir $HOME/fumosay`

retrieve the executable and fumofiles and move them there  
- `mv fumosay/target/release/fumo $HOME/fumosay/ && mv fumofiles $HOME/fumosay`


**Now you have fumosay installed !**

The next possible step would probably be to add this directory to the $PATH environment variable.


---

## Use
```
./fumosay [message]
```

---

## TODO list

- ❌ Add more fumos
- ❌ Make it possible to use other fumos
- ❌ Improve message {box|bubble}
- somewhat ✅ Embed fumofiles or take care of ways of locating them
- ❌ Create a {deb|rpm} package
- ❌ Make an `install.sh` script

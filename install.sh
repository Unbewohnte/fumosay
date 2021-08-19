#!/bin/bash
# fumosay installation script

FUMOSAY_BIN_DIR_PATH=/usr/bin/
FUMOFILES_DIR_PATH=/usr/share/fumosay/fumofiles/

# create directories
mkdir -p $FUMOSAY_BIN_DIR_PATH && mkdir -p $FUMOFILES_DIR_PATH

# copy fumosay binary
if [ -e fumosay ] 
then
    cp fumosay $FUMOSAY_BIN_DIR_PATH && echo "Copied binary file"
elif [ -e target/x86_64-unknown-linux-musl/release/fumosay ] # For debug purposes. The binary file should be in the same directory as the script !!!
then
    cp target/x86_64-unknown-linux-musl/release/fumosay $FUMOSAY_BIN_DIR_PATH && echo "Copied binary from linux-musl directory"
else 
    echo "Could not find a fumosay binary file"
    exit 1
fi

# copy fumofiles
if [ -d fumofiles ]
then
    cp -r fumofiles/* $FUMOFILES_DIR_PATH && echo "Copied fumofiles"
else 
    echo "Could not find fumofiles directory"
    exit 1
fi


echo "Installation completed"



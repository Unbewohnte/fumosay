#!/bin/bash
# fumosay installation script

FUMOSAY_BIN_DIR_PATH=/usr/local/bin/
FUMOFILES_DIR_PATH=/usr/local/share/fumosay/fumofiles/

# create directories
mkdir -p $FUMOSAY_BIN_DIR_PATH && mkdir -p $FUMOFILES_DIR_PATH

# copy fumosay binary
if [ -e fumosay ]
then
    cp fumosay $FUMOSAY_BIN_DIR_PATH
else 
    echo "Could not find fumosay in current directory"
    exit 1
fi

# copy fumofiles
if [ -d fumofiles ]
then
    cp -r fumofiles/* $FUMOFILES_DIR_PATH
else 
    echo "Could not find fumofiles directory"
    exit 1
fi
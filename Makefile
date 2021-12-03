SRC_DIR := src
FUMOSAY_BIN_NAME := fumosay
FUMOFILES := fumofiles
FUMOSAY_INSTALLATION_DIR := /usr/local/bin
FUMOFILES_DST_DIR := /usr/local/share/fumosay

all:
	g++ -O2 -Wall -Werror -static src/fumosay.cpp -o fumosay

install: all
	mkdir -p $(FUMOSAY_INSTALLATION_DIR) && \
	mkdir -p $(FUMOSAY_INSTALLATION_DIR) && \
	cp -r $(FUMOFILES) $(FUMOFILES_DST_DIR) && \
	cp $(FUMOSAY_BIN_NAME) $(FUMOSAY_INSTALLATION_DIR)

clean:
	rm fumosay
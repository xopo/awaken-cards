.PHONY: dev build install clean

BINARY := hacker-news 
INSTALL_DIR := $(HOME)/.cargo/bin

dev: 
	bacon run

build: 
	cargo build --release

install: 
	cargo install --path .

clean: 
	cargo clean

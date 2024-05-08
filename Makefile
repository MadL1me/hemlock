build-all: build-linux build-x86_64-apple-darwin build-x86_64-pc-windows-gnu

build-linux: 
	cross build --release --target=x86_64-unknown-linux-gnu
	cd target/x86_64-pc-windows-gnu && tar -czf hemlock-linux.tar.gz hemlock
	cd target/x86_64-pc-windows-gnu && shasum -a 256 hemlock-linux.tar.gz

build-linux-amd64: 
	cross build --release --target=x86_64-unknown-linux-gnu
	cd target/x86_64-pc-windows-gnu && tar -czf hemlock-linux.tar.gz hemlock
	cd target/x86_64-pc-windows-gnu && shasum -a 256 hemlock-linux.tar.gz


build-mac:
	cargo build --release --target=x86_64-apple-darwin
	cd target/x86_64-apple-darwin/release && tar -czf hemlock-mac.tar.gz hemlock
	cd target/x86_64-apple-darwin/release && shasum -a 256 hemlock-mac.tar.gz

build-windows:
	cargo build --release --target=x86_64-pc-windows-gnu
	cd target/x86_64-pc-windows-gnu && tar -czf hemlock-windows.tar.gz hemlock
	cd target/x86_64-pc-windows-gnu && shasum -a 256 hemlock-windows.tar.gz

it-linux:
	docker run -it -v ./target/x86_64-unknown-linux-gnu/release:/HEMLOCK --platform linux/amd64 ubuntu

it-windows:
	docker run -it -v ./target/x86_64-pc-windows-gnu/release:/HEMLOCK --platform amd64 mcr.microsoft.com/windows:ltsc2019

download-deps:
	rustup target add x86_64-pc-windows-gnu

docker-pull-mac:
	docker pull --platform linux/amd64 ghcr.io/cross-rs/x86_64-unknown-linux-gnu:0.2.5

.PHONY: build-linux \


// https://github.com/MadL1me/hemlock/releases/download/v0.0.1/hemlock-mac.tar.gz
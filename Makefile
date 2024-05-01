build-all: build-linux build-x86_64-apple-darwin build-x86_64-pc-windows-gnu

build-linux: 
	cargo build --release --target=x86_64-unknown-linux-gnu

build-x86_64-apple-darwin:
	cargo build --release --target=x86_64-apple-darwin
	cd target/x86_64-apple-darwin/release && tar -czf hemlock-mac.tar.gz hemlock
	cd target/x86_64-apple-darwin/release && shasum -a 256 hemlock-mac.tar.gz

build-x86_64-pc-windows-gnu:
	cargo build --release --target=x86_64-pc-windows-gnu

calc-sha: 
	shasum -a 256 getfilesize.tar.gz

.PHONY: build-linux \

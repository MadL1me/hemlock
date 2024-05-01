build-add: build-linux build-mac build-x86_64_windows

build-linux: 
	cargo build --release --target=xy

build-mac:
	cargo build --release --target=x86_64-pc-windows-gnu

build-x86_64_windows:
	cargo build --release --target=x86_64-pc-windows-gnu

calc-sha: 
	shasum -a 256 getfilesize.tar.gz

.PHONY: build-linux \

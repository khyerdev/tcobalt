tcobalt:
	export RUSTUP_TOOLCHAIN=stable
	cargo fetch --locked --target "$$(rustc -vV | sed -n 's/host: //p')"

	export CARGO_TARGET_DIR=target
	cargo build --frozen --release --all-features

install:
	install -Dm0755 -t "/usr/bin" "target/release/tcobalt"
	ln -sf "/usr/bin/tcobalt" "/usr/bin/tcb"


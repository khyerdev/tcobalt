tcobalt:
	export RUSTUP_TOOLCHAIN=stable
	export CARGO_TARGET_DIR=target

	cargo fetch --locked --target "$$(rustc -vV | sed -n 's/host: //p')"
	cargo build --frozen --release --all-features

debug:
	export RUSTUP_TOOLCHAIN=stable
	export CARGO_TARGET_DIR=target

	cargo fetch --locked --target "$$(rustc -vV | sed -n 's/host: //p')"
	cargo build --frozen --all-features

release: tcobalt

install:
	install -Dm0755 -t "/usr/bin" "target/release/tcobalt"
	ln -sf "/usr/bin/tcobalt" "/usr/bin/tcb"


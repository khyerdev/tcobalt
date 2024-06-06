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

arm:
	export RUSTUP_TOOLCHAIN=stable
	export CARGO_TARGET_DIR=target

	cargo fetch --locked --target aarch64-unknown-linux-gnu
	CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc \
	cargo build --frozen --all-features --target aarch64-unknown-linux-gnu --release


release: tcobalt

install:
	install -Dm0755 -t "/usr/bin" "target/release/tcobalt"
	ln -sf "/usr/bin/tcobalt" "/usr/bin/tcb"


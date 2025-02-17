.PHONY: all clap clean

all: clap

clap: 
	cargo +nightly run --manifest-path nih/xtask/Cargo.toml --bin xtask bundle cyclegripper_nih --release

clean: 
	cargo clean

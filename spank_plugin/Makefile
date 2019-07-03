all: src/*.rs
	cargo build --release
	cp target/release/libspank_plugin_bank.so spank_bank.so
clean:
	rm -rf target
	rm -rf *.so
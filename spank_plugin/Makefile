all: src/*.rs
	cargo build
	cp target/debug/libspank_plugin_bank.so spank_slurm_banking.so
clean:
	rm -rf target
	rm -rf *.so

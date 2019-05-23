all: src/*.rs
	cargo build --release
	cp target/release/libjobcomp_bank.so jobcomp_bank.so
clean:
	rm -rf target
	rm -rf *.so
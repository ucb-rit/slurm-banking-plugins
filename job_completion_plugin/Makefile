all: src/*.rs
	cargo build --debug
	cp target/release/libjobcomp_bank.so jobcomp_slurm_banking.so
clean:
	rm -rf target
	rm -rf *.so

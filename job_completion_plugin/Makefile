all: src/*.rs
	cargo build
	cp target/debug/libjobcomp_bank.so jobcomp_slurm_banking.so
clean:
	rm -rf target
	rm -rf *.so

all: src/*.rs
	cargo build
	cp target/debug/libjob_submit_bank.so job_submit_slurm_banking.so
clean:
	rm -rf target
	rm -rf *.so

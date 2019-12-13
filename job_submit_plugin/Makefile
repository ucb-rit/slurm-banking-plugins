all: src/*.rs
	cargo build --debug
	cp target/release/libjob_submit_bank.so job_submit_slurm_banking.so
clean:
	rm -rf target
	rm -rf *.so

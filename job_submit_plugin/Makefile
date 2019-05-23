all: src/*.rs
	cargo build --release
	cp target/release/libjob_submit_bank.so job_submit_bank.so
clean:
	rm -rf target
	rm -rf *.so
plugins: src/*.rs
	cargo build --release
	cp target/release/libjob_submit_bank.so job_submit_bank.so
docker: src/*.rs
	docker build -t slurm-banking-plugins .
	docker run -it -h ernie slurm-banking-plugins
clean:
	rm -rf target
	rm -rf *.so
	docker rmi -f slurm-banking-plugins
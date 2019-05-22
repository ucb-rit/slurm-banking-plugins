plugins: src/*.rs
	cargo build --release
	cp target/release/libjob_submit_bank.so job_submit_bank.so
docker: src/*.rs
	docker build -t slurm-banking-plugins .
	docker run -it -h ernie slurm-banking-plugins
docker-dev:
	docker build -t slurm-banking-plugins-dev .
	docker run -v $(shell pwd)/src:/slurm-banking-plugins/src -it -h ernie slurm-banking-plugins-dev
install:
	cp *.so /usr/lib64/slurm/.
clean:
	rm -rf target
	rm -rf *.so
	docker rmi -f slurm-banking-plugins slurm-banking-plugins-dev
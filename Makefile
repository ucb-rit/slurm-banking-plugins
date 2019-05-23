SLURM_SOURCE_CODE_DIR=$(pwd)/slurm
PLUGIN_INSTALL_PREFIX=/usr/lib64/slurm/

all: jobcomp_bank.so job_submit_bank.so 

jobcomp_bank.so: job_completion_plugin/**/*
	SLURM_SOURCE_CODE_DIR=$(SLURM_SOURCE_CODE_DIR) $(MAKE) -C job_completion_plugin all
	cp job_completion_plugin/*.so .
job_submit_bank.so: job_submit_plugin/**/*
	SLURM_SOURCE_CODE_DIR=$(SLURM_SOURCE_CODE_DIR) $(MAKE) -C job_submit_plugin all
	cp job_submit_plugin/*.so .

.PHONY: docker
docker: docker/**/* **/*
	docker build -f docker/build/Dockerfile -t slurm-banking-plugins .
	docker run -it -h ernie slurm-banking-plugins

docker-dev: docker/**/* **/*
	docker build -f docker/dev/Dockerfile -t slurm-banking-plugins-dev .
	docker run \
		-v $(shell pwd)/job_submit_plugin/src:/slurm-banking-plugins/job_submit_plugin/src \
		-v $(shell pwd)/job_completion_plugin/src:/slurm-banking-plugins/job_completion_plugin/src \
		-v $(shell pwd)/slurm_banking/src:/slurm-banking-plugins/slurm_banking/src \
		-it -h ernie slurm-banking-plugins-dev
	
install: 
	cp job_completion_plugin/jobcomp_bank.so $(PLUGIN_INSTALL_PREFIX)/.
	cp job_submit_plugin/job_submit_bank.so $(PLUGIN_INSTALL_PREFIX)/.

uninstall:
	rm -f $(PLUGIN_INSTALL_PREFIX)/jobcomp_bank.so
	rm -f $(PLUGIN_INSTALL_PREFIX)/job_submit_bank.so

clean:
	$(MAKE) -C job_completion_plugin clean
	$(MAKE) -C job_submit_plugin clean
	rm -rf *.so slurm_banking/wrappers/src
	docker rmi -f slurm-banking-plugins slurm-banking-plugins-dev
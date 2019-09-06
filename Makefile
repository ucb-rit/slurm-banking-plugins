SLURM_SOURCE_CODE_DIR=$(shell pwd)/slurm
PLUGIN_INSTALL_PREFIX=/usr/lib64/slurm
SPANK_PLUGIN_INSTALL_PREFIX=/etc/slurm/spank

all: jobcomp_slurm_banking.so job_submit_slurm_banking.so spank_slurm_banking.so

slurm/slurm/slurm.h:
	cd $(SLURM_SOURCE_CODE_DIR) && ./configure
jobcomp_slurm_banking.so: slurm/slurm/slurm.h job_completion_plugin/**/*
	CPATH=$(CPATH):$(SLURM_SOURCE_CODE_DIR) SLURM_SOURCE_CODE_DIR=$(SLURM_SOURCE_CODE_DIR) $(MAKE) -C job_completion_plugin all
	cp job_completion_plugin/*.so .
job_submit_slurm_banking.so: slurm/slurm/slurm.h job_submit_plugin/**/*
	CPATH=$(CPATH):$(SLURM_SOURCE_CODE_DIR) SLURM_SOURCE_CODE_DIR=$(SLURM_SOURCE_CODE_DIR) $(MAKE) -C job_submit_plugin all
	cp job_submit_plugin/*.so .
spank_slurm_banking.so: slurm/slurm/slurm.h spank_plugin/**/*
	CPATH=$(CPATH):$(SLURM_SOURCE_CODE_DIR) SLURM_SOURCE_CODE_DIR=$(SLURM_SOURCE_CODE_DIR) $(MAKE) -C spank_plugin all
	cp spank_plugin/*.so .
mybrc_rest_client: spec/swagger.json
	docker run --rm -v $(shell pwd):/local swaggerapi/swagger-codegen-cli generate \
		-i /local/spec/swagger.json \
		-l rust \
		-o /local/mybrc_rest_client

.PHONY: test
test:
	cd job_completion_plugin && cargo fmt --all -- --check && cd ..
	cd job_submit_plugin && cargo fmt --all -- --check && cd ..
	cd spank_plugin && cargo fmt --all -- --check && cd ..
	cd slurm_banking && cargo fmt --all -- --check && cd ..

.PHONY: docker
docker: docker/**/* **/*
	docker build -f docker/build/Dockerfile -t slurm-banking-plugins .
	docker run -it -h ernie slurm-banking-plugins

.PHONY: docker-dev
docker-dev: docker/**/* **/*
	docker build -f docker/dev/Dockerfile -t slurm-banking-plugins-dev .
	docker run \
		-v $(shell pwd)/job_submit_plugin/src:/slurm-banking-plugins/job_submit_plugin/src \
		-v $(shell pwd)/job_completion_plugin/src:/slurm-banking-plugins/job_completion_plugin/src \
		-v $(shell pwd)/spank_plugin/src:/slurm-banking-plugins/spank_plugin/src \
		-v $(shell pwd)/slurm_banking/src:/slurm-banking-plugins/slurm_banking/src \
		-it -h ernie slurm-banking-plugins-dev

install:
	mkdir -p $(PLUGIN_INSTALL_PREFIX)
	mkdir -p $(SPANK_PLUGIN_INSTALL_PREFIX)
	cp job_completion_plugin/jobcomp_slurm_banking.so $(PLUGIN_INSTALL_PREFIX)/.
	cp job_submit_plugin/job_submit_slurm_banking.so $(PLUGIN_INSTALL_PREFIX)/.
	cp spank_plugin/spank_slurm_banking.so $(SPANK_PLUGIN_INSTALL_PREFIX)/.

uninstall:
	rm -f $(PLUGIN_INSTALL_PREFIX)/jobcomp_slurm_banking.so
	rm -f $(PLUGIN_INSTALL_PREFIX)/job_submit_slurm_banking.so
	rm -f $(SPANK_PLUGIN_INSTALL_PREFIX)/spank_slurm_banking.so

clean:
	$(MAKE) -C job_completion_plugin clean
	$(MAKE) -C job_submit_plugin clean
	$(MAKE) -C spank_plugin clean
	rm -rf *.so slurm_banking/wrappers/src

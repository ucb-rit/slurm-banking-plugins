FROM docker-centos6-slurm:latest
RUN yum install -y glibc clang # cargo
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
RUN echo "JobSubmitPlugins=job_submit/slurm_banking" >> /etc/slurm/slurm.conf \
    && echo "JobCompType=jobcomp/slurm_banking" >> /etc/slurm/slurm.conf
ADD . /slurm-banking-plugins
WORKDIR /slurm-banking-plugins
RUN git submodule update --init --recursive

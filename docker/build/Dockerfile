FROM giovtorres/docker-centos7-slurm:latest
RUN yum install -y glibc clang cargo
RUN echo "JobSubmitPlugins=job_submit/bank" >> /etc/slurm/slurm.conf \
    && echo "JobCompPlugins=jobcomp/bank" >> /etc/slurm/slurm.conf
ADD . /slurm-banking-plugins
RUN cd /slurm-banking-plugins \
    && git submodule update --init --recursive \
    && make all \
    && make install \
    && cp prices.toml /etc/slurm/.
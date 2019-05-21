FROM giovtorres/docker-centos7-slurm:latest
RUN echo "JobSubmitPlugins=job_submit/bank" >> /etc/slurm/slurm.conf
RUN yum install -y glibc clang cargo
ADD . /slurm-banking-plugins
RUN cd /slurm-banking-plugins \
    && make plugins \
    && cp *.so /usr/lib64/slurm/.
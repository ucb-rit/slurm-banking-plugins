FROM giovtorres/docker-centos7-slurm:18.08.7
RUN yum install -y glibc clang cargo
RUN git clone https://github.com/SchedMD/slurm.git \
    && cd slurm \
    && git checkout tags/slurm-18-08-7-1 \
    && ./configure
RUN echo "JobSubmitPlugins=job_submit/bank" >> /etc/slurm/slurm.conf \
    && echo "JobCompType=jobcomp/bank" >> /etc/slurm/slurm.conf
ADD . /slurm-banking-plugins
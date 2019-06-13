FROM giovtorres/docker-centos7-slurm:18.08.7
RUN yum install -y glibc clang cargo
RUN echo "JobSubmitPlugins=job_submit/bank" >> /etc/slurm/slurm.conf \
    && echo "JobCompType=jobcomp/bank" >> /etc/slurm/slurm.conf
ADD . /slurm-banking-plugins
RUN mv /slurm /slurm-banking-plugins/.
FROM giovtorres/docker-centos7-slurm:18.08.7
RUN yum install -y glibc clang cargo
RUN echo "JobSubmitPlugins=job_submit/slurm_banking" >> /etc/slurm/slurm.conf \
    && echo "JobCompType=jobcomp/slurm_banking" >> /etc/slurm/slurm.conf
ADD . /slurm-banking-plugins
WORKDIR /slurm-banking-plugins
RUN git submodule update --init --recursive
RUN pushd slurm && git checkout slurm-18.08 && popd
FROM giovtorres/docker-centos7-slurm:latest
RUN echo "JobSubmitPlugins=job_submit/bank" >> /etc/slurm/slurm.conf
RUN yum install -y glibc clang cargo
# RUN wget http://ftp.gnu.org/gnu/glibc/glibc-2.18.tar.gz \
#     && tar -xvf glibc-2.18.tar.gz \
#     && mkdir build \
#     && cd build \
#     && ../glibc-2.18/configure --prefix=/ \
#     && make -j4 \
#     && make install
# RUN ln -sf /lib/libc-2.18.so /lib64/libc.so.6
ADD . /slurm-banking-plugins
# RUN cp /slurm-banking-plugins/job_submit_bank.so /usr/lib64/slurm/.
# RUN cp /slurm-banking-plugins/*.so /usr/lib64/slurm/. \
#     && cp /slurm-banking-plugins/prices.toml /etc/slurm/.
# RUN cd /slurm-banking-plugins \
#     && make plugins \
#     && cp *.so /usr/lib64/slurm/. \
#     && cp prices.toml /etc/slurm/.
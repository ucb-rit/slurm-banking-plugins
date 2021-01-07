FROM centos:6
COPY ./CentOS-Base.repo /etc/yum.repos.d/CentOS-Base.repo
RUN yum clean all \
  && yum install -y epel-release \
  && yum install -y git clang openssl-devel \
  && yum groupinstall -y "Development Tools"
RUN curl --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh \
  && sh rustup.sh -y
ENV PATH /root/.cargo/bin:$PATH
WORKDIR /tmp
CMD [ "make" ]

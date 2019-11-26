FROM centos:7
RUN yum install -y epel-release \
  && yum install -y git clang openssl-devel \
  && yum groupinstall -y "Development Tools"
RUN curl --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh \
  && sh rustup.sh -y
ENV PATH /root/.cargo/bin:$PATH
WORKDIR /tmp
CMD [ "make" ]

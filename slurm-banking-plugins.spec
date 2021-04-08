Name:           slurm-banking-plugins
Version:        1
Release:        1%{?dist}
Summary:        Slurm banking plugins

License:        BSD-3
URL:            https://github.com/ucb-rit/slurm-banking-plugins
Source0:        %{name}-%{version}.tar.gz

BuildRequires: make clang gcc openssl-devel
Requires: glibc clang openssl-devel

%description


%prep
curl --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
sh rustup.sh -y
%setup -q


%build
source $HOME/.cargo/env
make


%install
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT/usr/lib64/slurm $RPM_BUILD_ROOT/etc/slurm/spank
%make_install

%clean
rm -rf $RPM_BUILD_ROOT

%files
%doc



%changelog

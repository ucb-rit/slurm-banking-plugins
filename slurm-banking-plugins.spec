Name:           slurm-banking-plugins
Version:        1
Release:        1%{?dist}
Summary:        Slurm banking plugins

License:        BSD-3
URL:            https://github.com/ucb-rit/slurm-banking-plugins
Source0:        %{name}-%{version}.tar.gz

BuildRequires: make clang
Requires: glibc clang  

%description


%prep
%setup -q


%build
make


%install
rm -rf $RPM_BUILD_ROOT
%make_install


%files
%doc



%changelog

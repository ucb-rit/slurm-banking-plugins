# Slurm Banking Plugins

__Currently in development - Not ready for use__

Slurm banking plugins provide allocation management to Slurm. The plugins deduct service units for completed and running jobs and prevent jobs from running if there are insufficient service units available. There are two plugins, one running on job submission and the other on job completion. The job completion plugin will reimburse service units if the job ran for less time than was expected based on its submission options. The plugins interact with an HTTP-based API to keep track of service units per account and user.

These plugins are written in [Rust](https://www.rust-lang.org), an efficient and memory-safe programming language. It uses [rust-bindgen](https://github.com/rust-lang/rust-bindgen) to automatically generate the Rust foreign function interface (FFI) bindings based on the Slurm C header files.

## Build Requirements
- [Rust](https://www.rust-lang.org/) (including [cargo](https://doc.rust-lang.org/cargo/))
- [Slurm](https://github.com/SchedMD/slurm) header files
- [Clang](http://clang.llvm.org/get_started.html) (dependency for [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html))
- [OpenSSL](https://www.openssl.org/) (dependency for [reqwest](https://docs.rs/reqwest/0.9.17/reqwest/))

## Building
Since the Slurm `jobcomp` plugins need access to the `"src/common/slurm_jobcomp.h"` header, we need access to the Slurm source code `src` directory in order to build (as well as the normal `<slurm/slurm.h>` headers on the `CPATH`). 

1. Edit the path to the path at the top of the Makefile to point to the Slurm source code directory
2. Once you have all the dependencies, just run `make` :)
3. After building, you will find the `.so` files in the same directory as the Makefile

### NixOS
`shell.nix` provides the environment for development on [NixOS](https://nixos.org). I run the following:

```bash
nix-shell 
make
```

## Developing
I use the [docker-centos7-slurm](https://github.com/giovtorres/docker-centos7-slurm) Docker container as a base, and build the plugins on top of it. 

`make docker-dev` builds the development container with Slurm plus all the other necessary dependencies for the plugins and drops you into a shell. The code is stored in `/slurm-banking-plugins` in the container. After making your changes, use `make && make install` to compile and install the plugins, then restart Slurm with `supervisorctl restart all`.


# Table of Contents

1.  [Introduction](#org4e542fd)
2.  [Limitations](#org6f71254)
3.  [Build Requirements](#orgceb3bac)
4.  [Building](#orgae7c48a)
    1.  [On Savio](#orgec5158d)
    2.  [NixOS](#org0ace64a)
    3.  [Help](#orgea425c0)
5.  [Usage](#org41b8357)
    1.  [Install, enable, and configure](#orgc92b0af)
        1.  [Install the `.so` files](#org10dda30)
        2.  [/etc/slurm/slurm.conf](#org9fac47e)
        3.  [/etc/slurm/plugstack.conf](#orgc3ef59b)
        4.  [/etc/slurm/bank-config.toml](#org4913398)
    2.  [Help/Debugging](#orgb4d1af2)
6.  [Developing](#org659c5dc)
    1.  [Project Structure](#orga96a35a)
    2.  [myBRC API Codegen](#org68855d2)
    3.  [Testing with myBRC](#org2f41b51)
    4.  [Creating a Release](#org286fe75)

<a href="https://travis-ci.org/ucb-rit/slurm-banking-plugins"><img src="https://travis-ci.org/ucb-rit/slurm-banking-plugins.svg?branch=master"></a> <a href="."><img src="https://img.shields.io/github/v/tag/ucb-rit/slurm-banking-plugins"></a> <a href="."><img src="https://img.shields.io/github/languages/top/ucb-rit/slurm-banking-plugins"></a> <a href="."><img src="https://img.shields.io/github/repo-size/ucb-rit/slurm-banking-plugins"></a>


<a id="org4e542fd"></a>

# Introduction

Slurm banking plugins provide allocation management to Slurm. The plugins deduct service units for completed and running jobs and prevent jobs from running if there are insufficient service units available. The plugins interact with a REST API (provided by myBRC), documented in the [spec/swagger.json](./spec/swagger.json). The following three plugins are used:

-   [job\_submit\_plugin](./job_submit_plugin) (job submission stage): Estimate maximum job cost based on submission parameters, and reject job if the API reports that the user/account has insufficient service units available.
-   [spank\_plugin](./spank_plugin) (job running stage): Report job and estimated cost to the API.
-   [job\_completion\_plugin](./job_completion_plugin) (job completing stage): Modify job in API to reflect actual usage.

These plugins are written in [Rust](https://www.rust-lang.org) to help with safety. It uses [rust-bindgen](https://github.com/rust-lang/rust-bindgen) to automatically generate the Rust foreign function interface (FFI) bindings based on the Slurm C header files.


<a id="org6f71254"></a>

# Limitations

-   Since the spank plugin cannot cancel a job, the user could overdraw their service unit allocation if they had enough service units at the time of submission, but not enough service units at the time the job starts running, since the units are only actually deducted from the balance when the job starts running.
-   If `--ntasks` or `--cpus-per-task` are unspecified, the job completion plugin will assume the value is 0 and will always allow the job, as long as the balance is non-negative. This can be improved in the future by checking whether the requested partition is exclusive and how many CPUs each node has, and then using that information to estimate the number of CPUs that will be used.
-   If the myBRC API is offline (or returns errors), the submit plugin will let the job go through.


<a id="orgceb3bac"></a>

# Build Requirements

-   [Rust](https://www.rust-lang.org/) (including [cargo](https://doc.rust-lang.org/cargo/))
-   [OpenSSL](https://openssl.org) (needed for making an HTTPS connection to the API)
-   [Slurm](https://github.com/SchedMD/slurm) header files and source code
-   [Clang](http://clang.llvm.org/get_started.html) (build dependency for [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html))


<a id="orgae7c48a"></a>

# Building

Since the Slurm `jobcomp` plugins need access to the `src/common/slurm_jobcomp.h` header, we need access to the Slurm source code `src` directory in order to build (as well as the normal `<slurm/slurm.h>` headers on the `CPATH`). 

You will have to first run `./configure` on the Slurm source code, otherwise `<slurm/slurm.h>` will not exist. If you don't run `./configure`, the Makefile will try to do it for you.

1.  Edit the path at the top of the Makefile to point to the Slurm source code directory, or symlink `./slurm` in this repository to point to it.
2.  Once you have all the dependencies, just run `make` :)
3.  After building, you will find the `.so` files in the same directory as the Makefile.


<a id="orgec5158d"></a>

## On Savio

You will need the Rust and `clang` dependencies. 
Rust can be installed following the instructions on [rustup.rs](https://rustup.rs), and is easiest if installed locally for each user. 
`clang` can be loaded as a module (or by setting the environment variables).

The plugins can be built as an unprivileged user, as long as that user can read the Slurm source code.

    # Install Rust locally for your user, and select default installation
    curl --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    
    # Clone the plugins repository
    git clone https://github.com/ucb-rit/slurm-banking-plugins.git && cd slurm-banking-plugins
    
    # Compile clang from source and load environment
    scripts/clang/build-clang.sh
    source scripts/clang/clang-env.sh
    
    # Point to slurm source code (OR you can make a copy)
    rmdir slurm && ln -s /path/to/slurm/source slurm # Point to slurm source
    rmdir slurm && cp -r /path/to/slurm/source slurm # OR make a copy
    
    # Compile plugins
    make

Then, follow the instructions in [Usage](#orga6caa71) to install, enable, and configure the plugins.

**When adding the .so binaries to the nodes with Warewulf, you must use "wwsh file import" instead of "wwsh file new". Make sure the format in "wwsh file print" is listed as binary.**


<a id="org0ace64a"></a>

## NixOS

`shell.nix` provides the environment for development on [NixOS](https://nixos.org). I run the following:

    nix-shell 
    make


<a id="orgea425c0"></a>

## Help

For additional reference on building, check [the build on travis-ci](https://travis-ci.org/ucb-rit/slurm-banking-plugins).


<a id="org41b8357"></a>

# <a id="orga6caa71"></a> Usage


<a id="orgc92b0af"></a>

## Install, enable, and configure


<a id="org10dda30"></a>

### Install the `.so` files

The `job_submit_slurm_banking.so` and `jobcomp_slurm_banking.so` should be installed in `/usr/lib64/slurm/`. The `spank_slurm_banking.so` plugin should be installed in `/etc/slurm/spank/`.

    make install


<a id="org9fac47e"></a>

### /etc/slurm/slurm.conf

Enable the submit and completion plugins:

    # other config options above...
    JobSubmitPlugins=job_submit/slurm_banking
    JobCompType=jobcomp/slurm_banking


<a id="orgc3ef59b"></a>

### /etc/slurm/plugstack.conf

Enable the spank plugin:

    optional /etc/slurm/spank/spank_slurm_banking.so


<a id="org4913398"></a>

### /etc/slurm/bank-config.toml

Configure the plugin settings. Options that **must** be set properly include the API URL, API token, and partition names. You can use the example provided as a template.

    cp bank-config.toml.example /etc/slurm/bank-config.toml


<a id="orgb4d1af2"></a>

## Help/Debugging

-   The plugins log errors to the slurmd (spank plugin) and slurmctld (job submit and job completion plugins) logs. You can filter for their output by grepping for `_bank`.
-   For a working example installation, refer to [the Docker files](./docker)


<a id="org659c5dc"></a>

# Developing

I use the [docker-centos7-slurm](https://github.com/giovtorres/docker-centos7-slurm) Docker container as a base, and build the plugins on top of it. 

`make docker-dev` builds the development container with Slurm plus all the other necessary dependencies for the plugins and drops you into a shell. The code is stored in `/slurm-banking-plugins` in the container. After making your changes, use `make && make install` to compile and install the plugins, copy the `plugstack.conf` and `bank-config.toml` config files to `/etc/slurm/`, and finally restart Slurm with `supervisorctl restart all`.


<a id="orga96a35a"></a>

## Project Structure

Each plugin is its own Rust project: [job\_completion\_plugin](./job_completion_plugin), [job\_submit\_plugin](./job_submit_plugin), and [spank\_plugin](./spank_plugin). Each of these uses the [slurm\_banking](./slurm_banking) project, which includes the job calculation functionality and helpers for calling the API. Communication with the myBRC API is done through [mybrc\_rest\_client](./mybrc_rest_client), described in the next section.


<a id="org68855d2"></a>

## myBRC API Codegen

I use [swagger-codegen](https://github.com/swagger-api/swagger-codegen) to generate a library to abstract away access to the API. The API is described by a schema file in [spec/swagger.json](./spec/swagger.json). This file is automatically generated by the myBRC API, and can be obtained at `/swagger.json` on the myBRC API.

If the API spec changes and you need to update this plugin, just regenerate the API client. First, put the new `swagger.json` in [spec/swagger.json](./spec/swagger.json). To generate the API client based on this new schema, I use the Dockerized version of [swagger-codegen](https://github.com/swagger-api/swagger-codegen) like so:

    docker run --rm -v $(pwd):/local swaggerapi/swagger-codegen-cli generate \
      -i /local/spec/swagger.json \
      -l rust \
      -o /local/mybrc_rest_client

You may find the generated files are not owned by your user, so just run `chown -R $USER mybrc_rest_client`.


<a id="org2f41b51"></a>

## Testing with myBRC

    # Build mybrc-rest Docker image from scgup
    docker build -f Dockerfile.mybrc-rest -t mybrc-rest
    
    # Build slurm-banking-plugins-dev image
    make docker-dev
    
    # Launch containers
    docker run --name=mybrc-rest -d -p 8181:8181 mybrc-rest
    docker run \
      -v $(pwd)/job_submit_plugin/src:/slurm-banking-plugins/job_submit_plugin/src \
      -v $(pwd)/job_completion_plugin/src:/slurm-banking-plugins/job_completion_plugin/src \
      -v $(pwd)/slurm_banking/src:/slurm-banking-plugins/slurm_banking/src \
      --link mybrc-rest -it -h ernie slurm-banking-plugins-dev


<a id="org286fe75"></a>

## Creating a Release

GitHub Actions is set up to automatically build [releases](https://github.com/ucb-rit/slurm-banking-plugins/releases) for tags starting with a `v`.
For example, if I push a tag `v0.1.0`, it will build releases for the code at that point.
There is a GitHub action to build using Docker for CentOS 6 and CentOS 7. In each of these,
you may specify the version of Slurm to compile against in the "Compile plugins" stage by changing
the tag to checkout of the Slurm source code. The GitHub Actions are in [.github/workflows](./.github/workflows). 
In this example, it's using `slurm-18-08-7-1` in the CentOS 6 build environment:

    - name: Compile plugins
      run: |
        scripts/build-with-docker.sh slurm-18-08-7-1 slurm-banking-plugins-centos6:latest


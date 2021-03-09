
# Table of Contents

1.  [Introduction](#orgf5ae0b7)
2.  [Limitations](#org55282f1)
3.  [Build Requirements](#orgca5c36e)
4.  [Building](#orgb2b531b)
    1.  [On Savio](#org99ffd08)
    2.  [NixOS](#org030ef2e)
    3.  [Help](#orgbd81377)
5.  [Usage](#org2c00e81)
    1.  [Install, enable, and configure](#org581000b)
        1.  [Install the `.so` files](#org84be7fd)
        2.  [/etc/slurm/slurm.conf](#orgbd345e5)
        3.  [/etc/slurm/plugstack.conf](#orgcfd3d1b)
        4.  [/etc/slurm/bank-config.toml](#orgc19269f)
    2.  [Help/Debugging](#org535c6d5)
6.  [Developing](#org90f9bce)
    1.  [Project Structure](#org205f1c9)
    2.  [myBRC API Codegen](#orgdf1f7ab)
    3.  [Testing with myBRC](#org50e850b)
    4.  [Creating a Release](#orge61bad3)

<a href="https://travis-ci.org/ucb-rit/slurm-banking-plugins"><img src="https://travis-ci.org/ucb-rit/slurm-banking-plugins.svg?branch=master"></a> <a href="."><img src="https://img.shields.io/github/v/tag/ucb-rit/slurm-banking-plugins"></a> <a href="."><img src="https://img.shields.io/github/languages/top/ucb-rit/slurm-banking-plugins"></a> <a href="."><img src="https://img.shields.io/github/repo-size/ucb-rit/slurm-banking-plugins"></a>


<a id="orgf5ae0b7"></a>

# Introduction

Slurm banking plugins provide allocation management to Slurm. The plugins deduct service units for completed and running jobs and prevent jobs from running if there are insufficient service units available. The plugins interact with a REST API (provided by myBRC), documented in the [spec/swagger.json](./spec/swagger.json). The following three plugins are used:

-   [job\_submit\_plugin](./job_submit_plugin) (job submission stage): Estimate maximum job cost based on submission parameters, and reject job if the API reports that the user/account has insufficient service units available.
-   [spank\_plugin](./spank_plugin) (job running stage): Report job and estimated cost to the API.
-   [job\_completion\_plugin](./job_completion_plugin) (job completing stage): Modify job in API to reflect actual usage.

These plugins are written in [Rust](https://www.rust-lang.org) to help with safety. It uses [rust-bindgen](https://github.com/rust-lang/rust-bindgen) to automatically generate the Rust foreign function interface (FFI) bindings based on the Slurm C header files.


<a id="org55282f1"></a>

# Limitations

-   Since the spank plugin cannot cancel a job, the user could overdraw their service unit allocation if they had enough service units at the time of submission, but not enough service units at the time the job starts running, since the units are only actually deducted from the balance when the job starts running.
-   If `--ntasks` or `--cpus-per-task` are unspecified, the job completion plugin will assume the value is 0 and will always allow the job, as long as the balance is non-negative. This can be improved in the future by checking whether the requested partition is exclusive and how many CPUs each node has, and then using that information to estimate the number of CPUs that will be used.
-   If the myBRC API is offline (or returns errors), the submit plugin will let the job go through.


<a id="orgca5c36e"></a>

# Build Requirements

-   [Rust](https://www.rust-lang.org/) (including [cargo](https://doc.rust-lang.org/cargo/))
-   [OpenSSL](https://openssl.org) (needed for making an HTTPS connection to the API)
-   [Slurm](https://github.com/SchedMD/slurm) header files and source code
-   [Clang](http://clang.llvm.org/get_started.html) (build dependency for [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html))


<a id="orgb2b531b"></a>

# Building

Since the Slurm `jobcomp` plugins need access to the `src/common/slurm_jobcomp.h` header, we need access to the Slurm source code `src` directory in order to build (as well as the normal `<slurm/slurm.h>` headers on the `CPATH`). 

You will have to first run `./configure` on the Slurm source code, otherwise `<slurm/slurm.h>` will not exist. If you don&rsquo;t run `./configure`, the Makefile will try to do it for you.

1.  Edit the path at the top of the Makefile to point to the Slurm source code directory, or symlink `./slurm` in this repository to point to it.
2.  Once you have all the dependencies, just run `make` :)
3.  After building, you will find the `.so` files in the same directory as the Makefile.


<a id="org99ffd08"></a>

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
    rmdir slurm && cp -r /path/to/slurm/source slurm
    
    # Compile plugins
    make

Then, follow the instructions in [Usage](#org5d2c816) to install, enable, and configure the plugins.

**When adding the .so binaries to the nodes with Warewulf, you must use &ldquo;wwsh file import&rdquo; instead of &ldquo;wwsh file new&rdquo;. Make sure the format in &ldquo;wwsh file print&rdquo; is listed as binary.**


<a id="org030ef2e"></a>

## NixOS

`shell.nix` provides the environment for development on [NixOS](https://nixos.org). I run the following:

    nix-shell 
    make


<a id="orgbd81377"></a>

## Help

For additional reference on building, check [the build on travis-ci](https://travis-ci.org/ucb-rit/slurm-banking-plugins).


<a id="org2c00e81"></a>

# <a id="org5d2c816"></a> Usage


<a id="org581000b"></a>

## Install, enable, and configure


<a id="org84be7fd"></a>

### Install the `.so` files

The `job_submit_slurm_banking.so` and `jobcomp_slurm_banking.so` should be installed in `/usr/lib64/slurm/`. The `spank_slurm_banking.so` plugin should be installed in `/etc/slurm/spank/`.

    make install


<a id="orgbd345e5"></a>

### /etc/slurm/slurm.conf

Enable the submit and completion plugins:

    # other config options above...
    JobSubmitPlugins=job_submit/slurm_banking
    JobCompType=jobcomp/slurm_banking


<a id="orgcfd3d1b"></a>

### /etc/slurm/plugstack.conf

Enable the spank plugin:

    optional /etc/slurm/spank/spank_slurm_banking.so


<a id="orgc19269f"></a>

### /etc/slurm/bank-config.toml

Configure the plugin settings. Options that **must** be set properly include the API URL, API token, and partition names. You can use the example provided as a template.

    cp bank-config.toml.example /etc/slurm/bank-config.toml


<a id="org535c6d5"></a>

## Help/Debugging

-   The plugins log errors to the slurmd (spank plugin) and slurmctld (job submit and job completion plugins) logs. You can filter for their output by grepping for `_bank`.
-   For a working example installation, refer to [the Docker files](./docker)


<a id="org90f9bce"></a>

# Developing

I use the [docker-centos7-slurm](https://github.com/giovtorres/docker-centos7-slurm) Docker container as a base, and build the plugins on top of it. For newer versions of Slurm, we use our own fork at [docker-centos7-slurm](https://github.com/ucb-rit/docker-centos7-slurm). For CentOS 6 testing we also have [docker-centos6-slurm](https://github.com/ucb-rit/docker-centos6-slurm).

`make docker-dev` builds the development container with Slurm (CentOS 7) plus all the other necessary dependencies for the plugins and drops you into a shell. The code is stored in `/slurm-banking-plugins` in the container.

Once in the container, check the Slurm version with `scontrol -V` and checkout the corresponding Slurm version in `/slurm-banking-plugins/slurm` so that the plugins are compiled against the correct Slurm version:

    pushd /slurm-banking-plugins/slurm
    git checkout tags/slurm-20-02-6-1 # for example
    popd

After making your changes, use `make && make install` to compile and install the plugins, copy the `plugstack.conf` and `bank-config.toml` config files to `/etc/slurm/`, make configuration changes as desired, and finally restart Slurm with `supervisorctl restart all`.

If the services do not start correctly, try starting them one-by-one with:

    supervisorctl status # inspect status
    supervisorctl start slurmctld

There is also the CentOS 6 equivalent with `make docker-centos6-dev`.


<a id="org205f1c9"></a>

## Project Structure

Each plugin is its own Rust project: [job\_completion\_plugin](./job_completion_plugin), [job\_submit\_plugin](./job_submit_plugin), and [spank\_plugin](./spank_plugin). Each of these uses the [slurm\_banking](./slurm_banking) project, which includes the job calculation functionality and helpers for calling the API. Communication with the myBRC API is done through [mybrc\_rest\_client](./mybrc_rest_client), described in the next section.


<a id="orgdf1f7ab"></a>

## myBRC API Codegen

I use [openapi-generator](https://github.com/OpenAPITools/openapi-generator) to generate a library to abstract away access to the API. The API is described by a schema file in [spec/swagger.json](./spec/swagger.json). This file is automatically generated by the myBRC API, and can be obtained at `/swagger.json` on the myBRC API.

If the API spec changes and you need to update this plugin, just regenerate the API client. First, put the new `swagger.json` in [spec/swagger.json](./spec/swagger.json). To generate the API client based on this new schema, I use the Dockerized version of [swagger-codegen](https://github.com/swagger-api/swagger-codegen) like so:

    docker run --rm -v $(shell pwd):/local openapitools/openapi-generator-cli generate \
    	-i /local/spec/swagger.json \
    	-g rust \
    	-o /local/mybrc_rest_client \
    	--library=reqwest

You may find the generated files are not owned by your user, so just run `chown -R $USER mybrc_rest_client`.


<a id="org50e850b"></a>

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


<a id="orge61bad3"></a>

## Creating a Release

GitHub Actions is set up to automatically build [releases](https://github.com/ucb-rit/slurm-banking-plugins/releases) for tags starting with a `v`.
For example, if I push a tag `v0.1.0`, it will build releases for the code at that point.
There is a GitHub action to build using Docker for CentOS 6 and CentOS 7. In each of these,
you may specify the version of Slurm to compile against in the &ldquo;Compile plugins&rdquo; stage by changing
the tag to checkout of the Slurm source code. The GitHub Actions are in [.github/workflows](./.github/workflows). 
In this example, it&rsquo;s using `slurm-18-08-7-1` in the CentOS 6 build environment:

    - name: Compile plugins
      run: |
        scripts/build-with-docker.sh slurm-18-08-7-1 slurm-banking-plugins-centos6:latest


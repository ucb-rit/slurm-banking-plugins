                        _______________________

                         SLURM BANKING PLUGINS
                        _______________________


Table of Contents
_________________

1. Limitations
2. Build Requirements
3. Building
.. 1. On Savio
.. 2. NixOS
.. 3. Help
4. Usage
.. 1. /etc/slurm/slurm.conf
.. 2. Help/Debugging
5. Developing
.. 1. Project Structure
.. 2. myBRC API Codegen
.. 3. Testing with myBRC


Slurm banking plugins provide allocation management to Slurm. The
plugins deduct service units for completed and running jobs and prevent
jobs from running if there are insufficient service units available. The
plugins interact with a REST API (provided by myBRC), documented in the
[spec/swagger.json]. The following three plugins are used:

- [job_submit_plugin] (job submission stage): Estimate maximum job cost
  based on submission parameters, and reject job if the API reports that
  the user/account has insufficient service units available.
- [spank_plugin] (job running stage): Report job and estimated cost to
  the API.
- [job_completion_plugin] (job completing stage): Modify job in API to
  reflect actual usage.

These plugins are written in [Rust] to help with safety. It uses
[rust-bindgen] to automatically generate the Rust foreign function
interface (FFI) bindings based on the Slurm C header files.


[spec/swagger.json] <./spec/swagger.json>

[job_submit_plugin] <./job_submit_plugin>

[spank_plugin] <./spank_plugin>

[job_completion_plugin] <./job_completion_plugin>

[Rust] <https://www.rust-lang.org>

[rust-bindgen] <https://github.com/rust-lang/rust-bindgen>


1 Limitations
=============

  - Since the spank plugin cannot cancel a job, the user could overdraw
    their service unit allocation if they had enough service units at
    the time of submission, but not enough service units at the time the
    job starts running, since the units are only actually deducted from
    the balance when the job starts running.
  - If `--ntasks' or `--cpus-per-task' are unspecified, the job
    completion plugin will assume the value is 0 and will always allow
    the job, as long as the balance is non-negative. This can be
    improved in the future by checking whether the requested partition
    is exclusive and how many CPUs each node has, and then using that
    information to estimate the number of CPUs that will be used.
  - If the myBRC API is offline (or returns errors), the submit plugin
    will let the job go through.


2 Build Requirements
====================

  - [Rust] (including [cargo])
  - [Slurm] header files and source code
  - [Clang] (dependency for [rust-bindgen])


[Rust] <https://www.rust-lang.org/>

[cargo] <https://doc.rust-lang.org/cargo/>

[Slurm] <https://github.com/SchedMD/slurm>

[Clang] <http://clang.llvm.org/get_started.html>

[rust-bindgen]
<https://rust-lang.github.io/rust-bindgen/requirements.html>


3 Building
==========

  Since the Slurm `jobcomp' plugins need access to the
  `"src/common/slurm_jobcomp.h"' header, we need access to the Slurm
  source code `src' directory in order to build (as well as the normal
  `<slurm/slurm.h>' headers on the `CPATH').

  You will have to first run `./configure' on the Slurm source code,
  otherwise `<slurm/slurm.h>' will not exist. If you don't run
  `./configure', the Makefile will try to do it for you.

  1. Edit the path at the top of the Makefile to point to the Slurm
     source code directory, or symlink `./slurm' in this repository to
     point to it.
  2. Once you have all the dependencies, just run `make' :)
  3. After building, you will find the `.so' files in the same directory
     as the Makefile.


3.1 On Savio
~~~~~~~~~~~~

  ,----
  | # After installing Rust (using rustup)...
  | module load clang
  | git clone https://github.com/ucb-rit/slurm-banking-plugins.git && cd slurm-banking-plugins
  | rmdir slurm && ln -s /path/to/slurm/source slurm # Point to slurm source
  | make
  | make install
  | vim /etc/slurm/slurm.conf # Edit slurm.conf to enable the job submit and job completion plugins
  | vim /etc/slurm/plugstack.conf # Edit plugstack.conf to enable the spank plugin
  | cp bank-config.toml /etc/slurm/bank-config.toml
  `----

  *When adding the .so binaries to the nodes with Warewulf, you must use
   `wwsh file import' instead of `wwsh file new'. Make sure the format
   in "wwsh file print" is listed as binary.*


3.2 NixOS
~~~~~~~~~

  `shell.nix' provides the environment for development on [NixOS]. I run
  the following:

  ,----
  | nix-shell 
  | make
  `----


[NixOS] <https://nixos.org>


3.3 Help
~~~~~~~~

  For additional reference on building, check [the build on
  travis-ci](<https://travis-ci.org/ucb-rit/slurm-banking-plugins>).


4 Usage
=======

  1. Move the `.so' files to `/usr/lib64/slurm' and `/etc/slurm/spank':
  ,----
  | make install
  `----

  1. Move `bank-config.toml' to `/etc/slurm/bank-config.toml' and update
     the partitions/prices accordingly.
  ,----
  | cp bank-config.toml /etc/slurm/bank-config.toml
  `----

  1. Include the spank plugin in `/etc/slurm/plugstack.conf' and the
     others in `/etc/slurm/slurm.conf':
  ,----
  | cp plugstack.conf /etc/slurm/.
  `----


4.1 /etc/slurm/slurm.conf
~~~~~~~~~~~~~~~~~~~~~~~~~

  ,----
  | # other config options above...
  | JobSubmitPlugins=job_submit/bank
  | JobCompType=jobcomp/bank
  `----


4.2 Help/Debugging
~~~~~~~~~~~~~~~~~~

  - The plugins log errors to the slurmd (spank plugin) and slurmctld
    (job submit and job completion plugins) logs. You can filter for
    their output by grepping for `_bank'.
  - For a working example installation, refer to [the Docker files]


[the Docker files] <./docker>


5 Developing
============

  I use the [docker-centos7-slurm] Docker container as a base, and build
  the plugins on top of it.

  `make docker-dev' builds the development container with Slurm plus all
  the other necessary dependencies for the plugins and drops you into a
  shell. The code is stored in `/slurm-banking-plugins' in the
  container. After making your changes, use `make && make install' to
  compile and install the plugins, copy the `plugstack.conf' and
  `bank-config.toml' config files to `/etc/slurm/', and finally restart
  Slurm with `supervisorctl restart all'.


[docker-centos7-slurm]
<https://github.com/giovtorres/docker-centos7-slurm>

5.1 Project Structure
~~~~~~~~~~~~~~~~~~~~~

  Each plugin is its own Rust project: [job_completion_plugin],
  [job_submit_plugin], and [spank_plugin]. Each of these uses the
  [slurm_banking] project, which includes the job calculation
  functionality and helpers for calling the API. Communication with the
  myBRC API is done through [mybrc_rest_client], described in the next
  section.


[job_completion_plugin] <./job_completion_plugin>

[job_submit_plugin] <./job_submit_plugin>

[spank_plugin] <./spank_plugin>

[slurm_banking] <./slurm_banking>

[mybrc_rest_client] <./mybrc_rest_client>


5.2 myBRC API Codegen
~~~~~~~~~~~~~~~~~~~~~

  I use [swagger-codegen] to generate a library to abstract away access
  to the API. The API is described by a schema file in
  [spec/swagger.json]. This file is automatically generated by the myBRC
  API, and can be obtained at `/swagger.json' on the myBRC API.

  If the API spec changes and you need to update this plugin, just
  regenerate the API client. First, put the new `swagger.json' in
  [spec/swagger.json]. To generate the API client based on this new
  schema, I use the Dockerized version of [swagger-codegen] like so:

  ,----
  | docker run --rm -v $(pwd):/local swaggerapi/swagger-codegen-cli generate \
  |   -i /local/spec/swagger.json \
  |   -l rust \
  |   -o /local/mybrc_rest_client
  `----

  You may find the generated files are not owned by your user, so just
  run `chown -R $USER mybrc_rest_client'.


[swagger-codegen] <https://github.com/swagger-api/swagger-codegen>

[spec/swagger.json] <./spec/swagger.json>


5.3 Testing with myBRC
~~~~~~~~~~~~~~~~~~~~~~

  ,----
  | # Build mybrc-rest Docker image from scgup
  | docker build -f Dockerfile.mybrc-rest -t mybrc-rest
  | 
  | # Build slurm-banking-plugins-dev image
  | make docker-dev
  | 
  | # Launch containers
  | docker run --name=mybrc-rest -d -p 8181:8181 mybrc-rest
  | docker run \
  |   -v $(pwd)/job_submit_plugin/src:/slurm-banking-plugins/job_submit_plugin/src \
  |   -v $(pwd)/job_completion_plugin/src:/slurm-banking-plugins/job_completion_plugin/src \
  |   -v $(pwd)/slurm_banking/src:/slurm-banking-plugins/slurm_banking/src \
  |   --link mybrc-rest -it -h ernie slurm-banking-plugins-dev
  `----

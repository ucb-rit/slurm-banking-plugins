# Slurm Banking Plugins

__Currently in development - Not ready for use__

Slurm banking plugins provide allocation management to Slurm. The plugins deduct service units for completed and running jobs and prevent jobs from running if there are insufficient service units available. There are two plugins, one running on job submission and the other on job completion. The job completion plugin will reimburse service units if the job ran for less time than was expected based on its submission options.

## Design

The plugins interact with an API to keep track of service units per account and user. [slurm-banking-dummy-api](https://github.com/ucb-rit/slurm-banking-dummy-api) is a dummy API for testing the functionality.

## Usage

### Requirements
- [rust](https://www.rust-lang.org/)
- openssl (for a dependency)

### Setup
After building, you will find the `.so` files in `target/{debug,release}`.

```bash
cargo build
```

#### NixOS
On [NixOS](https://nixos.org), I have to run the following to build in order to satisfy the openssl dependency:

```bash
nix-shell -p openssl pkgconfig
cargo build
```

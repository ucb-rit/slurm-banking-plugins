#!/bin/bash
# Job name:
#SBATCH --job-name=test
#
# Account:
#SBATCH --account=account_name
#
# Partition:
#SBATCH --partition=normal
#
# Quality of Service:
#SBATCH --qos=normal
#
# Wall clock limit:
#SBATCH --time=00:00:30
#
## Command(s) to run:
echo "hello world"

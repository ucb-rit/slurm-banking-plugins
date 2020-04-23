#!/bin/bash

BASEDIR=$(dirname $0)

# Activate conda environment
source /home/nicolaschan/.bashrc
conda activate dashboard

# Collect job data
$BASEDIR/load-jobs-today-rest.sh

# Collect CPU data
$BASEDIR/cpu2rest.py

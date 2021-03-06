#!/bin/sh
SLURM_TAG=$1
DOCKER_IMAGE=$2
cd slurm && git checkout "$SLURM_TAG" && cd ..
sudo make clean
sudo rm -f slurm/slurm/slurm.h
docker run -v $(pwd):/tmp "$DOCKER_IMAGE"

PLUGIN_DIR="plugins-$SLURM_TAG"
mkdir -p "$PLUGIN_DIR"
mv *.so "$PLUGIN_DIR"

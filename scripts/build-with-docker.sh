#!/bin/sh
SLURM_TAG=$1
DOCKER_IMAGE=$2
pushd slurm && git checkout "$SLURM_TAG" && popd
docker run -v $(pwd):/tmp "$DOCKER_IMAGE"

PLUGIN_DIR="plugins-$SLURM_TAG"
mkdir -p "$PLUGIN_DIR"
mv *.so "$PLUGIN_DIR"
tar -czvf "${PLUGIN_DIR}.tar.gz" "$PLUGIN_DIR"

#!/usr/bin/env bash
export DISTR=${1:-stretch}
#mkdir tmp
#cp Dockerfile tmp/
#docker rmi rust:${DISTR}-nightly
#docker build --no-cache --force-rm --rm --tag rust:${DISTR}-nightly -f tmp/Dockerfile tmp/
#rm -rf tmp

docker run \
    -ti \
    --rm \
    -v $PWD:/app \
    -w /app \
    rust:${DISTR}-nightly \
    /bin/sh -c 'cargo deb'
    
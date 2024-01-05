#!/bin/bash
docker run -it --rm -h playground \
    -e USER=$(id -u -n) \
    -e GROUP=$(id -g -n) \
    -e UID=$(id -u) \
    -e GID=$(id -g) \
    -v `pwd`:/home/$(id -u -n) \
    -w /home/$(id -u -n) \
    -p 50051:8000 \
    cartesi/playground:0.6.0 "/opt/cartesi/bin/remote-cartesi-machine --server-address=localhost:8000"
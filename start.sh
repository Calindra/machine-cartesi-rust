#!/bin/bash
docker run -it --rm -h playground -e USER=sandhilt -e GROUP=sandhilt -e UID=1000 -e GID=1000 -v /home/sandhilt/workspace/learn-lua:/home/sandhilt -w /home/sandhilt cartesi/playground:0.6.0 /bin/bash

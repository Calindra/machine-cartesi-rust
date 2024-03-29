#!/bin/bash
if [ -f "Dockerfile" ]; then
    echo "Dockerfile exists"
else
    echo "Dockerfile does not exist"
    exit 1
fi

docker build -t machine-cartesi-rust:latest .

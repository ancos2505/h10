#!/bin/sh

docker run --rm -it --network=host ghcr.io/hatoo/oha:latest http://localhost:8080 -n1m -c 1000

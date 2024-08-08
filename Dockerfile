FROM ubuntu:latest
LABEL authors="mdrea"

ENTRYPOINT ["top", "-b"]
FROM --platform=linux/amd64 ubuntu:latest

ENV DEBIAN_FRONTEND=non-interactive

RUN apt-get update && \
    apt-get install -y nasm gcc binutils && \
    apt-get clean

WORKDIR /workspace

CMD [ "bash" ]
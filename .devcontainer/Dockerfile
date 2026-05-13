FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive

ENV TZ=UTC
ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8
ENV LC_TIME=C.UTF-8

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      software-properties-common \
      ca-certificates \
      curl \
      gnupg && \
    add-apt-repository --yes universe && \
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
      | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main" \
      > /etc/apt/sources.list.d/nodesource.list

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      bash \
      build-essential \
      ca-certificates \
      clang \
      cmake \
      curl \
      file \
      g++ \
      gcc \
      git \
      jq \
      less \
      libcap-dev \
      libsqlite3-dev \
      libssl-dev \
      make \
      musl-tools \
      nodejs \
      openssh-client \
      pkg-config \
      protobuf-compiler \
      sudo \
      tzdata \
      unzip \
      vim \
      zlib1g-dev \
      just && \
    ln -snf /usr/share/zoneinfo/UTC /etc/localtime && \
    echo UTC > /etc/timezone && \
    rm -rf /var/lib/apt/lists/*

RUN echo "ubuntu ALL=(root) NOPASSWD:ALL" > /etc/sudoers.d/ubuntu && \
    chmod 0440 /etc/sudoers.d/ubuntu

USER ubuntu

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal && \
    /home/ubuntu/.cargo/bin/rustup component add clippy rustfmt

ENV PATH="/home/ubuntu/.cargo/bin:${PATH}"

WORKDIR /workspace

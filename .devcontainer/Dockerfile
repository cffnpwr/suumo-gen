FROM ubuntu:22.04

ARG UID=1000
ARG GID=1000
ARG USERNAME=developer

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

WORKDIR /workspace

RUN  apt-get update && \
  apt-get install -y --no-install-recommends sudo ca-certificates curl git gnupg unzip build-essential && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  groupadd --gid $GID $USERNAME && \
  useradd --uid $UID --gid $GID -m -s /bin/bash $USERNAME && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y && \
  chmod -R 777 $RUSTUP_HOME $CARGO_HOME && \
  echo "${USERNAME}   ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers 

USER ${USERNAME}

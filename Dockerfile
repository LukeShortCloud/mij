ARG ARCH=
FROM ${ARCH}debian:12.1-slim
ENV CMD_APT_INSTALL="apt-get install -y --no-install-recommends"
RUN apt-get update && ${CMD_APT_INSTALL} ca-certificates curl libasound2-dev openssl && curl --version && curl -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
VOLUME /workdir
CMD sleep infinity

FROM p4lang/third-party:latest
LABEL maintainer="P4 API Working Group <p4-dev@lists.p4.org>"
LABEL description="Dockerfile used for CI testing of p4lang/p4runtime"

# No questions asked during package installation.
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends software-properties-common git curl build-essential

ARG GO_VERSION=1.20.5

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
         amd64) arch='amd64' ;; \
	 arm64) arch='arm64' ;; \
         *) arch=''; echo >&2; echo >&2 "unsupported architecture '$dpkgArch'"; echo >&2 ; exit 1 ;; \
    esac; \
    curl -L -o go.tar.gz https://dl.google.com/go/go${GO_VERSION}.linux-${arch}.tar.gz; \
    tar -C /usr/local -xzf go.tar.gz; \
    rm -f go.tar.gz

ENV PATH="${PATH}:/usr/local/go/bin:/root/go/bin"

RUN go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.31
RUN go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.3

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.81.0 --profile minimal; \
    . $HOME/.cargo/env; \
    cargo install --locked protoc-gen-prost@0.4.0 protoc-gen-prost-crate@0.4.1 protoc-gen-tonic@0.4.1

ENV PATH="${PATH}:/root/.cargo/bin"

COPY . /p4runtime/
WORKDIR /p4runtime/

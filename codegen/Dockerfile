FROM p4lang/third-party:latest
LABEL maintainer="P4 API Working Group <p4-dev@lists.p4.org>"
LABEL description="Dockerfile used for CI testing of p4lang/p4runtime"

# No questions asked during package installation.
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends software-properties-common git curl

ARG GO_VERSION=1.14.2

RUN curl -o go.tar.gz https://dl.google.com/go/go$GO_VERSION.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go.tar.gz && \
    rm go.tar.gz

ENV PATH="${PATH}:/usr/local/go/bin:/root/go/bin"

# latest version of protoc-gen-go does not support GRPC
# https://github.com/protocolbuffers/protobuf-go/releases/tag/v1.20.0
ARG PROTOC_GEN_GO_VERSION=v1.3.2

ENV GO111MODULE=on

# required to generate gRPC bindings
RUN go get google.golang.org/grpc
RUN go get github.com/golang/protobuf/protoc-gen-go@$PROTOC_GEN_GO_VERSION

COPY . /p4runtime/
WORKDIR /p4runtime/


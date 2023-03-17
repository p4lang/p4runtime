"""Load dependencies needed to compile p4runtime as a 3rd-party consumer."""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def p4runtime_deps():
    """Loads dependencies needed to compile p4runtime."""
    # A recent absl version is required for gRPC, and the one pulled in by
    # com_google_protobuf is not recent enough.
    if "com_google_absl" not in native.existing_rules():
        http_archive(
            name = "com_google_absl",
            sha256 = "4208129b49006089ba1d6710845a45e31c59b0ab6bff9e5788a87f55c5abd602",
            strip_prefix = "abseil-cpp-20220623.0",
            urls = [
                "https://storage.googleapis.com/grpc-bazel-mirror/github.com/abseil/abseil-cpp/archive/20220623.0.tar.gz",
                "https://github.com/abseil/abseil-cpp/archive/20220623.0.tar.gz",
            ],
        )
    # Also required by the current gRPC version.
    # Can probably be removed when Protobuf dependencies are updated.
    if "upb" not in native.existing_rules():
        http_archive(
            name = "upb",
            sha256 = "017a7e8e4e842d01dba5dc8aa316323eee080cd1b75986a7d1f94d87220e6502",
            strip_prefix = "upb-e4635f223e7d36dfbea3b722a4ca4807a7e882e2",
            urls = [
                "https://storage.googleapis.com/grpc-bazel-mirror/github.com/protocolbuffers/upb/archive/e4635f223e7d36dfbea3b722a4ca4807a7e882e2.tar.gz",
                "https://github.com/protocolbuffers/upb/archive/e4635f223e7d36dfbea3b722a4ca4807a7e882e2.tar.gz",
            ],
        )
    if not native.existing_rule("com_google_protobuf"):
        http_archive(
            name = "com_google_protobuf",
            url = "https://github.com/protocolbuffers/protobuf/releases/download/v21.10/protobuf-all-21.10.tar.gz",
            strip_prefix = "protobuf-21.10",
            sha256 = "6fc9b6efc18acb2fd5fb3bcf981572539c3432600042b662a162c1226b362426",
        )
    if not native.existing_rule("rules_proto"):
        http_archive(
            name = "rules_proto",
            urls = [
                "https://mirror.bazel.build/github.com/bazelbuild/rules_proto/archive/9e4c622ba8c2178b71420ed3d14fb8874beee3a5.tar.gz",
                "https://github.com/bazelbuild/rules_proto/archive/9e4c622ba8c2178b71420ed3d14fb8874beee3a5.tar.gz",
            ],
            strip_prefix = "rules_proto-9e4c622ba8c2178b71420ed3d14fb8874beee3a5",
            sha256 = "c2182b2d8894b43dc30afbdc2ce44a216e7c6c933ed34e216bfbf86e2f4e1e48",
        )
    if not native.existing_rule("io_bazel_rules_go"):
        http_archive(
            name = "io_bazel_rules_go",
            urls = [
                "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.36.0/rules_go-v0.36.0.zip",
                "https://github.com/bazelbuild/rules_go/releases/download/v0.36.0/rules_go-v0.36.0.zip",
            ],
            sha256 = "ae013bf35bd23234d1dea46b079f1e05ba74ac0321423830119d3e787ec73483",
        )
    if not native.existing_rule("com_google_googleapis"):
        git_repository(
            name = "com_google_googleapis",
            remote = "https://github.com/googleapis/googleapis",
            commit = "9fe00a1330817b5ce00919bf2861cd8a9cea1a00",
            shallow_since = "1642638275 -0800",
        )
    if not native.existing_rule("com_github_grpc_grpc"):
        http_archive(
            name = "com_github_grpc_grpc",
            url = "https://github.com/grpc/grpc/archive/v1.51.1.tar.gz",
            strip_prefix = "grpc-1.51.1",
            sha256 = "b55696fb249669744de3e71acc54a9382bea0dce7cd5ba379b356b12b82d4229",
        )

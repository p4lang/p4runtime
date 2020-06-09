"""Load dependencies needed to compile p4runtime as a 3rd-party consumer."""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def p4runtime_deps():
    """Loads dependencies need to compile p4runtime."""
    if not native.existing_rule("com_google_protobuf"):
        http_archive(
            name = "com_google_protobuf",
            url = "https://github.com/protocolbuffers/protobuf/releases/download/v3.12.3/protobuf-all-3.12.3.tar.gz",
            strip_prefix = "protobuf-3.12.3",
            sha256 = "1a83f0525e5c8096b7b812181865da3c8637de88f9777056cefbf51a1eb0b83f",
        )
    if not native.existing_rule("rules_proto"):
        http_archive(
            name = "rules_proto",
            urls = [
                "https://mirror.bazel.build/github.com/bazelbuild/rules_proto/archive/97d8af4dc474595af3900dd85cb3a29ad28cc313.tar.gz",
                "https://github.com/bazelbuild/rules_proto/archive/97d8af4dc474595af3900dd85cb3a29ad28cc313.tar.gz",
            ],
            strip_prefix = "rules_proto-97d8af4dc474595af3900dd85cb3a29ad28cc313",
            sha256 = "602e7161d9195e50246177e7c55b2f39950a9cf7366f74ed5f22fd45750cd208",
        )
    if not native.existing_rule("com_google_googleapis"):
        git_repository(
            name = "com_google_googleapis",
            remote = "https://github.com/googleapis/googleapis",
            commit = "dd244bb3a5023a4a9290b21dae6b99020c026123",
            shallow_since = "1591402163 -0700",
        )
    if not native.existing_rule("com_github_grpc_grpc"):
        http_archive(
            name = "com_github_grpc_grpc",
            url = "https://github.com/grpc/grpc/archive/v1.29.1.tar.gz",
            strip_prefix = "grpc-1.29.1",
            sha256 = "0343e6dbde66e9a31c691f2f61e98d79f3584e03a11511fad3f10e3667832a45",
        )

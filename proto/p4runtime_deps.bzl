"""Load dependencies needed to compile p4runtime as a 3rd-party consumer."""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def p4runtime_deps():
    """Loads dependencies needed to compile p4runtime."""
    if not native.existing_rule("com_google_protobuf"):
        http_archive(
            name = "com_google_protobuf",
            url = "https://github.com/protocolbuffers/protobuf/releases/download/v3.18.1/protobuf-all-3.18.1.tar.gz",
            strip_prefix = "protobuf-3.18.1",
            sha256 = "b8ab9bbdf0c6968cf20060794bc61e231fae82aaf69d6e3577c154181991f576",
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
                "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.29.0/rules_go-v0.29.0.zip",
                "https://github.com/bazelbuild/rules_go/releases/download/v0.29.0/rules_go-v0.29.0.zip",
            ],
            sha256 = "2b1641428dff9018f9e85c0384f03ec6c10660d935b750e3fa1492a281a53b0f",
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
            url = "https://github.com/grpc/grpc/archive/v1.44.0-pre1.tar.gz",
            strip_prefix = "grpc-1.44.0-pre1",
            sha256 = "477f9760fcef65660319831251adac02d20b1220f3af0aff19b5400707de445f",
        )

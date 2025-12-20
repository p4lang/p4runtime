"""Load dependencies needed to compile p4runtime as a 3rd-party consumer."""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def p4runtime_deps():
    """Loads dependencies needed to compile p4runtime."""
    if not native.existing_rule("com_google_protobuf"):
        http_archive(
            name = "com_google_protobuf",
            url = "https://github.com/protocolbuffers/protobuf/archive/refs/tags/v28.2.tar.gz",
            strip_prefix = "protobuf-28.2",
            sha256 = "b2340aa47faf7ef10a0328190319d3f3bee1b24f426d4ce8f4253b6f27ce16db",
        )
    if not native.existing_rule("rules_proto"):
        http_archive(
            name = "rules_proto",
            sha256 = "6fb6767d1bef535310547e03247f7518b03487740c11b6c6adb7952033fe1295",
            strip_prefix = "rules_proto-6.0.2",
            url = "https://github.com/bazelbuild/rules_proto/releases/download/6.0.2/rules_proto-6.0.2.tar.gz",
        )
    if not native.existing_rule("io_bazel_rules_go"):
        http_archive(
            name = "io_bazel_rules_go",
            sha256 = "f4a9314518ca6acfa16cc4ab43b0b8ce1e4ea64b81c38d8a3772883f153346b8",
            urls = [
                "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.50.1/rules_go-v0.50.1.zip",
                "https://github.com/bazelbuild/rules_go/releases/download/v0.50.1/rules_go-v0.50.1.zip",
            ],
        )
    if not native.existing_rule("com_google_googleapis"):
        git_repository(
            name = "com_google_googleapis",
            remote = "https://github.com/googleapis/googleapis",
            commit = "de509e38d37a2a9d8b95e1ce78831189f4f3c0f4",
        )
    if not native.existing_rule("com_github_grpc_grpc"):
        http_archive(
            name = "com_github_grpc_grpc",
            url = "https://github.com/grpc/grpc/archive/refs/tags/v1.67.0.tar.gz",
            strip_prefix = "grpc-1.67.0",
            sha256 = "af0638f73e4452e22e295f8b3f452518234254104713a08497f3d3aaa76733ad",
        )
    if not native.existing_rule("bazel_skylib"):
        http_archive(
            name = "bazel_skylib",
            sha256 = "bc283cdfcd526a52c3201279cda4bc298652efa898b10b4db0837dc51652756f",
            urls = [
                "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.7.1/bazel-skylib-1.7.1.tar.gz",
                "https://github.com/bazelbuild/bazel-skylib/releases/download/1.7.1/bazel-skylib-1.7.1.tar.gz",
            ],
        )
    if not native.existing_rule("rules_license"):
        http_archive(
            name = "rules_license",
            urls = [
                "https://mirror.bazel.build/github.com/bazelbuild/rules_license/releases/download/1.0.0/rules_license-1.0.0.tar.gz",
                "https://github.com/bazelbuild/rules_license/releases/download/1.0.0/rules_license-1.0.0.tar.gz",
            ],
            sha256 = "26d4021f6898e23b82ef953078389dd49ac2b5618ac564ade4ef87cced147b38",
        )

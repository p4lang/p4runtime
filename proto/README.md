# P4Runtime protobuf definition files

## Using Bazel rules

Supported Bazel versions: 0.13, 0.14

1. Import this project, for example using `git_repository`.
2. Import dependencies in your `WORKSPACE`:
```
load("@com_github_p4lang_p4runtime//bazel:deps.bzl", "p4runtime_deps")
p4runtime_deps()
load("@com_github_p4lang_p4runtime//bazel:rules.bzl", "p4runtime_proto_repositories")
p4runtime_proto_repositories()
```
3. Use the provided targets:
    1. `p4types_proto_cc`, `p4info_proto_cc`, `p4data_proto_cc` and
    `p4runtime_proto_cc` for Protobuf C++ libraries.
    2. `p4runtime_grpc_cc` for P4Runtime with gRPC.

If you want to use different versions of Protbuf, gRPC and
rules_protobuf, you will be on your own.

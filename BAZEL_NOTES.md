### Bazel version?

0.23.2

### Why use [stackb/rules_proto](https://github.com/stackb/rules_proto) instead of native rules or [grpc/grpc](https://github.com/grpc/grpc)?

The only Bazel native rules I am aware of are `proto_library` and
`cc_proto_library`. These work well despite an issue with `proto_source_root` in
the `proto_library` rule. However, there is no native support for Python and
gRPC (any language?) at this time (Bazel 0.23.2), which is why we use
`stackb/rules_proto`. The `grpc/grpc` that ships with gRPC is also buggy.

### What is the issue with official grpc rules?

The official `grpc/grpc` repository provides Bazel rules to generate gRPC libraries for
both C++ and Python, but it doesn't work well with some external library such
as [googleapis](https://github.com/googleapis/googleapis/).

Also, the `py_proto_library` is not a public rule now, and the maintainer
[said](https://github.com/grpc/grpc/pull/19183#issuecomment-497480869) that
this rule is a temporary solution.

Moreover, `py_proto_library` and `py_grpc_library` rules are
[requested](https://github.com/grpc/grpc/issues/19255), but there is no
guarantee that when will it be done. We could switch to official rules
if it becomes stable.

### What is the `proto_source_root` issue?

I originally had the BUILD file in the proto package, and I was setting
`proto_source_root` to "proto" in the proto_library rules. However, it wasn't
working because of a Bazel issue.
See https://github.com/bazelbuild/bazel/issues/4544 for the issue.
See
https://github.com/bazelbuild/bazel/commit/e8956648d1c94a3a51e1aba5d229d1f27bdf8e35
for the fix.
The `proto_source_root` fix may be available in Bazel 0.15.x.

After upgrading Bazel to a newer version (0.23.2), the `proto_source_root` works for `proto_library`,
but still not working because the `cc_proto_library` does not handle the `proto_source_root`
and does not put correct include path while compiling the target.

So instead I moved the rules to the root BUILD file, and I am borrowing
`internal_copied_filegroup` from protobuf.bzl to "remove" the "proto" prefix from
my paths.

### Which versions of Protobuf and gRPC you are using?

Most of the dependencies come from `stackb/rules_proto`.
The `stackb/rules_proto` uses gRPC v1.20.1 and protobuf v3.7.1.

### Wouldn't it be better to invoke `protoc` manually in the BUILD file?

It would probably be less dependent on the versions of Bazel, Protobuf, and
gRPC. And not much more verbose. We could switch to invoking `protoc` directly if
the current rules are too much of a headache to maintain.

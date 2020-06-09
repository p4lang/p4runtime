# Bazel Notes

This project can be built using [Bazel](https://bazel.build/).
The Bazel WORKSPACE and BUILD files are located in the [proto folder][proto/].

To build, run
```sh
cd proto && bazel build //...
```

We run continuous integration to ensure this works with the latest version of
Bazel.


## Why not put the WORSKPACE file at the root of the project?

This would require stripping the `proto/` prefix when building the protobuf
libraries, for example using the `strip_import_prefix` option of the
`proto_library` rule. However, this currently brakes the `cc_grpc_library` rule.
Moving the WORSKPACE file to the proto directory is a simple workaround.

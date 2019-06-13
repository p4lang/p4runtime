"""Load dependencies needed to compile p4runtime as a 3rd-party consumer."""

load("//bazel:workspace_rule.bzl", "remote_workspace")

def p4runtime_deps():
    """Loads dependencies need to compile p4runtime."""

    if "build_stack_rules_proto" not in native.existing_rules():
        remote_workspace(
            name = "build_stack_rules_proto",
            remote = "https://github.com/stackb/rules_proto",
            commit = "2f4e4f62a3d7a43654d69533faa0652e1c4f5082",
        )

    if "com_google_googleapis" not in native.existing_rules():
        remote_workspace(
            name = "com_google_googleapis",
            remote = "https://github.com/googleapis/googleapis",
            commit = "1079c999f0683196d857795ae6951ced9e15ce72",
        )

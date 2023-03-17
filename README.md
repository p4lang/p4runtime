# P4Runtime Specification

This directory contains protobuf files, specifications and related artifacts for
all versions of the P4Runtime API. Documentation and protobuf definitions are
placed into two distinct top-level directories. In each of these directories,
files are organized based on the P4Runtime major version number (e.g. v1) as
follows:
```
.
├── docs
│   └── v1  # documentation for P4Runtime v1
├── proto
│   └── p4
│       ├── config
│       │   └── v1  # p4.config.v1 protobuf package (P4Info message definition)
│       └── v1  # p4.v1 protobuf package (P4Runtime service definition)
```

Git tags are used to mark minor and patch release versions.

## Reading the latest version of the documentation

The latest version of the P4Runtime v1 specification is available:
* [here](https://p4.org/p4-spec/docs/p4runtime-spec-working-draft-html-version.html)
  in **HTML** format
* [here](https://p4.org/p4-spec/docs/p4runtime-spec-working-draft-pdf-version.html)
  in **PDF** format

It is updated every time a new commit is pushed to the main branch.

## Overview

P4 is a language for programming the data plane of network devices. The
P4Runtime API is a control plane specification for controlling the data plane
elements of a device or program defined by a P4 program. This repository
provides a precise definition of the P4Runtime API via protobuf (.proto) files
and accompanying documentation. The target audience for this includes system
architects and developers who want to write controller applications for P4
devices or switches.

# Community

 * **Meetings**: the P4.org API Working Group meets every other Friday at
   9:30AM (Pacific Time). Please see the [P4 Working Groups Calendar](https://calendar.google.com/calendar/u/0/embed?src=j4to42rsjqtfks0qb7iah8gous@group.calendar.google.com&ctz=America/Los_Angeles)
   for meeting details.
 * **Email**: join our [mailing
   list](https://lists.p4.org/mailman/listinfo/p4-dev_lists.p4.org) to receive
   announcements and stay up-to-date with Working Group activities.
 * **Slack**: ask to join the [P4 Slack Workspace] to get (or provide!)
   interactive help.

# Compiling P4Runtime Protobuf files

## Build Using Docker

You can use Docker to run the protoc compiler on the P4Runtime Protobuf files
and generate the Protobuf & gRPC bindings for C++, Python and Go:

```
docker build -t p4runtime -f codegen/Dockerfile .
docker run -v <OUT>:/out/ -t p4runtime /p4runtime/codegen/compile_protos.sh /out/
```

This will generate the bindings in the local `<OUT>` directory. **You need to
provide the absolute path for `<OUT>`.** The default Docker user is root, so you
may need to change the permissions manually for the generated files after the
`docker run` command exits.

These commands are the ones used by our CI system to ensure that the Protobuf
files stay semantically valid.

## Build Using Bazel ![build protobufs](https://github.com/p4lang/p4runtime/workflows/build%20protobufs/badge.svg)

The protobufs can also be built using [Bazel](https://bazel.build/).
The Bazel WORKSPACE and BUILD files are located in the [proto folder](proto/).

To build, run
```sh
cd proto && bazel build //...
```

We run [continuous integration](.github/workflows/ci-build-proto.yml) to ensure
this works with the latest version of Bazel.

For an example of how to include P4Runtime in your own Bazel project, see
[bazel/example](bazel/example).

# Modification Policy

We use the following processes when making changes to the P4Runtime
specification and associated documents. These processes are designed to be
lightweight, to encourage active participation by members of the P4.org
community, while also ensuring that all proposed changes are properly vetted
before they are incorporated into the repository and released to the community.

## Core Processes

* Only members of the P4.org community may propose changes to the P4Runtime
  specification, and all contributed changes will be governed by the
  Apache-style license specified in the P4.org membership agreement.

* We will use [semantic versioning](http://semver.org/) to track changes to the
  P4Runtime specification: major version numbers track API-incompatible changes;
  minor version numbers track backward-compatible changes; and patch versions
  make backward-compatible bug fixes. Generally speaking, the P4Runtime working
  group co-chairs will typically batch together multiple changes into a single
  release, as appropriate.

## Detailed Processes

We now identify detailed processes for three classes of changes. The text below
refers to [key
committers](https://github.com/orgs/p4lang/teams/p4lang-key-committers), a
GitHub team that is authorized to modify the specification according to these
processes.

1. **Non-Technical Changes:** Changes that do not affect the definition of the
   API can be incorporated via a simple, lightweight review process: the author
   creates a pull request against the specification that a key committer must
   review and approve. The P4Runtime Working Group does not need to be
   explicitly notified. Such changes include improvements to the wording of the
   specification document, the addition of examples or figures, typo fixes, and
   so on.

2. **Technical Bug Fixes:** Any changes that repair an ambiguity or flaw in the
   current API specification can also be incorporated via the same lightweight
   review process: the author creates a GitHub issue as well as a pull request
   against the specification that a key committer must review and approve. The
   key committer should use their judgment in deciding if the fix should be
   incorporated without broader discussion or if it should be escalated to the
   P4Runtime Working Group. In any event, the Working Group should be notified
   by email.

3. **API Changes** Any change that substantially modifies the definition of the
   API, or extends it with new features, must be reviewed by the P4Runtime
   Working Group, either in an email discussion or a meeting. We imagine that
   such proposals would go through three stages: (i) a preliminary proposal with
   text that gives the motivation for the change and examples; (ii) a more
   detailed proposal with a discussion of relevant issues including the impact
   on existing programs; (iii) a final proposal accompanied by a design
   document, a pull request against the specification, and prototype
   implementation on a branch of `p4runtime`, and example(s) that illustrate the
   change. After approval, the author would create a GitHub issue as well as a
   pull request against the specification that a key committer must review and
   approve.

When updating the Protobuf files in a pull request, you will also need to update
the generated Go and Python files, which are hosted in this repository under
[go/](go/) and [py/](py/). This can be done easily by running `./codegen/update.sh`,
provided docker is installed and your user is part of the "docker" group
(which means that the `docker` command can be executed without `sudo`).

## Use generated P4Runtime library

### Go

To include the P4Runtime Go library to your project, you can add this repository url
to your `go.mod` file, for example:

```
module your_module_name

go 1.13

require (
  github.com/p4lang/p4runtime v1.3.0
)
```

### Python

To install P4Runtime Python library, use the `pip3` command:

```bash
pip3 install p4runtime
# Or specify the version
pip3 install p4runtime==1.3.0
```

## Guidelines for using Protocol Buffers (protobuf) in backwards-compatible ways

P4Runtime generally follows "Live at Head" development principles - new
development happens on the `main` branch and there are no support branches.
New releases are periodically created from the head of `main`.

P4Runtime follows [semantic versioning](https://semver.org/) for release
numbering, which means changes to the P4Runtime protobuf definitions have
implications on the next release number. The team has tried its best so
far to avoid a major version number bump, but recognizes that one may be
necessary in the future.

Whenever possible, it is best to introduce new functionality in backward
compatible ways. For example when role config was introduced, an unset
(empty) role configuration implies full pipeline access, which was the
default behavior before the feature was introduced.

There are no strict rules here for updating P4Runtime protobuf message
definitions, only advice written by those with experience in using
protobuf for applications while they have been extended over
time.  They are here for learning and reference:

* [Updating Proto Definitions Without Updating Code](https://developers.google.com/protocol-buffers/docs/overview#updating-defs)
* [Updating A Message Type](https://developers.google.com/protocol-buffers/docs/proto3#updating)
* [Backwards-compatibility issues in `oneof` fields](https://developers.google.com/protocol-buffers/docs/proto3#backwards-compatibility_issues)
* [API design guide](https://cloud.google.com/apis/design)

Some brief points, but not the full story:

* Do not change or reuse field numbers.
* Be careful when changing types.
* You can deprecate fields, but do not remove them (and make sure that
  you continue to support them) until you are sure that all clients
  and servers are updated.


[P4 Slack Workspace]: https://p4-lang.slack.com/join/shared_invite/enQtODA0NzY4Mjc5MTExLTRlMWVmN2I5ZTY4MTAzMDI3MGQ1OTZjM2ZmM2Q1MWE2YzZjYTQ2ZWMyMGUyYjQ2ZmIxMjFjZDE4ZThiN2ZkZWI

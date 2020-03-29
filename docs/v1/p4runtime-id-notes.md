# P4Runtime numeric id notes

There are many kinds of numeric identifiers used in the P4Runtime API
specification.  While they are all described in the specification and
accompanying Protobuf `.proto` files, this article attempts to collect
in one place notes on those identifiers, including:

+ Which software entity (or person) chooses their values
+ When the values are chosen
+ What is their scope in "space" and time in which they must be unique

This document was written while referring to the P4Runtime API
specification as of this commit of the Github repository
https://github.com/p4lang/p4runtime

```
commit c9cd4af17c2d562d6e4ec760237f16fea9558fcb
Author: Chris Sommers <31145757+chrispsommers@users.noreply.github.com>
Date:   Fri Mar 27 13:43:31 2020 -0700
```

This is after version 1.1.0 of the specification was released, and it
is expected that version 1.2.0 will be released soon after this time,
with very few (if any) changes from the version above.

Here is a complete list of Google Protobuf `.proto` files in this
version of the p4lang/p4runtime repository:

```bash
$ find . -name '*.proto'
./proto/p4/v1/p4runtime.proto
./proto/p4/v1/p4data.proto
./proto/p4/config/v1/p4info.proto
./proto/p4/config/v1/p4types.proto
```

And here are the number of occurrences of `id`, either as a word by
itself, or with `_` before it, in each of the files above.  These
restrictions are made to avoid matching against words like `valid` or
`bidirectional` in the comments.  As of the referenced version of the
files, it appears that all occurrences of numeric id values are either
a word by themselves, or have an underscore character just before
`id`.

```bash
$ find . -name '*.proto' | xargs egrep -c '(\bid\b|_id\b)'
./proto/p4/v1/p4runtime.proto:56
./proto/p4/v1/p4data.proto:0
./proto/p4/config/v1/p4info.proto:22
./proto/p4/config/v1/p4types.proto:0
```


# Numeric ids in the P4Info message

These are all defined in the file `proto/p4/config/v1/p4info.proto`.


## Numeric ids that are unique within an entire P4Info message

The P4Info messages listed below contain a `Preamble` message.  Each
`Preamble` message contains an `id` field, as well as several others
like `name`, `alias`, and `annotations`.  See the `Preamble` message
definition for the full list.

Section 6.3 "ID Allocation for P4Info Objects" of the P4Runtime API
specification says:

    The most significant 8 bits of the ID encodes the object type (as
    per Table 1).  [ ... ]  The remaining 24 bits must be generated in
    such a way that the resulting IDs must be globally unique in the
    scope of the P4Info message.

So all of these id values must be globally unique for a single P4Info
message, i.e. for a single compiled P4 program.  All of the values are
selected during compilation of the P4 program, either by the P4
compiler, or by the P4 developer using an `@id` annotation on the P4
object.

They are listed in alphabetical order by message name, except for
`DirectCounter` and `DirectMeter`, which are grouped together with
`Counter` and `Meter`, respectively.

+ `Action` - prefix 0x01
+ `ActionProfile` - prefix 0x11
+ `ControllerPacketMetadata` - prefix 0x04.  There can be at most two
  of these messages for a single P4 program, one named `packet_in` and
  the other `packet_out`.  Their id values must be different from each
  other, and have the prefix 0x04, but other than that they can be any
  numeric values.
+ `Counter` - prefix 0x12
  + `DirectCounter` - prefix 0x13
+ `Digest` - prefix 0x17
+ `ExternInstance` - Table 1 of specification has a reserved range of
  prefix values for vendor-specific extern types.  Each
  `ExternInstance` message must be grouped within an `Extern` message.
  The `Extern` message contains a field `extern_type_id` that
  specifies the id prefix of all `ExternInstance` objects of the same
  type, and those id prefix values must be unique within the context
  of the same P4Info message.
+ `Meter` - prefix 0x14
  + `DirectMeter` - prefix 0x15
+ `Register` - prefix 0x16
+ `Table` - prefix 0x02
+ `ValueSet` - prefix 0x03


## Numeric ids that are unique within a restricted scope of a P4Info message

The next group of id values below are in the `p4info.proto` file, but
not inside of `Preamble` messages.  The ids are unique, but only
within a scope that is smaller than the entire P4Info message,
described separately for each below.  Thus within a single P4Info
message, it is possible for there to be multiple messages of these
types with the same numeric id value.

| Field name | Message type | Scope | Notes |
| ---------- | ------------ | ----- | ----- |
| `id` | `MatchField` | `Table` or `ValueSet` object that contains the match field | Each `MatchField` describes one match field of the `Table` or `ValueSet`. |
| `id` | `Param` sub-message of `Action` | `Action` object | Each `Param` describes one (directionless) parameter of a P4 action. |
| `id` | `Metadata` sub-message of `ControllerPacketMetadata` | `ControllerPacketMetadata` object. | A P4 program can have at most two `ControllerPacketMetadata` messages, one named `packet_in`, the other `packet_out`. |


## Numeric ids used as references to P4 objects within a single P4Info message

The next group below are in the `p4info.proto` file, but not inside of
`Preamble` messages.  They are "references", meaning that they refer
to P4 objects by means of their numeric id value.  The value of these
fields must be equal to the id value of a P4 object of the appropriate
type, within the same P4Info message, with the exception that some of
them may be 0, described in the notes.

| Field name | Message type | Type of P4 object referred to | Notes |
| ---------- | ------------ | ----------------------------- | ----- |
| `const_default_action_id` | `Table` | `Action` | 0 means "the table's `default_action` is not declared `const` |
| `implementation_id` | `Table` | `ActionProfile` | 0 means "this table has no `implementation` table property. TBD: double check the meaning of 0 here. |
| `direct_resource_ids` | `Table` | `DirectCounter` or `DirectMeter` | repeated so that a single table can refer to up to one of each |
| `id` | sub-message `ActionRef` inside `Table` | `Action` | `ActionRef` messages are repeated, with a separate one for each action the table might invoke.  `ActionRef` messages contain optional annotation and `Scope` values that are specific to how a particular table is allowed to use that `Action` |
| `table_ids` | `ActionProfile` | `Table` | repeated once for each `Table` that uses this `ActionProfile` object.  This data is redundant with the `implementation_id` field above, but has been included for the convenience of consumers of P4Info messages. |
| `direct_table_id` | `DirectCounter` | `Table` | This data is redundant with the `direct_resource_ids` field above, but has been included for the convenience of consumers of P4Info messages. |
| `direct_table_id` | `DirectMeter` | `Table` | This data is redundant with the `direct_resource_ids` field above, but has been included for the convenience of consumers of P4Info messages. |


# Numeric ids that appear outside of a P4Info message

These are all defined in the file `proto/p4/v1/p4runtime.proto`.


## Numeric ids that are references to P4 objects in the relevant P4Info message

+ message `TableEntry`, field `table_id`
  + sub-message `FieldMatch`, field `field_id`
+ message `ValueSetEntry`, field `value_set_id`
  + sub-message (repeated) `ValueSetMember`
    + sub-message (repeated) `FieldMatch`, field `field_id`
+ message `TableAction`
  + sub-message `Action`, field `action_id`
    + sub-message `Param`, field `param_id`
+ message `CounterEntry`, field `counter_id` - TBD: I believe this
  should always be the id of a `Counter` object, never a
  `DirectCounter`
+ message `MeterEntry`, field `meter_id` - TBD: I believe this should
  always be the id of a `Meter` object, never a `DirectMeter`.
+ message `RegisterEntry`, field `register_id`
+ message `DigestEntry`, field `digest_id` - used to configure a
  `Digest` object as a whole, not to deal with digest messages.
+ message `DigestList`, field `digest_id` - such messages are sent by
  a server to a client, to report a list of digest messages generated
  by a P4 device.
+ message `DigestListAck`, field `digest_id` - such mssages are sent
  by a client to acknowledge an earlier `DigestList` message that it
  received.
+ message `ExternEntry`, fields `extern_type_id` and `extern_id`.

```
// action_profile_id is a reference to an ActionProfile object in the
// P4Info message.  That message type is used to define both action
// profiles and action selectors in a P4 program.

message ActionProfileMember {
  uint32 action_profile_id = 1;
}

// TBD: Is there some context within which a PacketMetadata message
// appears that distinguishes whether the metadata_id field values
// refer to fields of a "packet_in" vs. "packet_out" header?
message PacketMetadata {
  // This refers to Metadata.id coming from P4Info ControllerPacketMetadata.
  uint32 metadata_id = 1;
  bytes value = 2;
}
```


## Numeric ids that are NOT references to P4 objects

The ids in this section have nothing to do with any id values of P4
objects in a P4 program.


### Device ids

`device_id` field values appear in the message types listed below.
Their values are selected by means not specified in the P4Runtime
specification.

Scope: Unique within a single P4Runtime server.

+ `MasterArbitrationUpdate`
+ `SetForwardingPipelineConfigRequest`
+ `GetForwardingPipelineConfigRequest`
+ `WriteRequest`
+ `ReadRequest`

Here are two examples of how a P4Runtime server implementation might
select `device_id` values:

+ The values could be compiled into the P4Runtime server's code at
  compile time.
+ The P4Runtime server could do some kind of device discovery on a
  hardware board to determine how many devices there are, and assign
  then numbers in some predictable way, e.g. from 1 up to the number
  of devices.

Other ways are possible.  These examples are only mentioned to give
some idea of how they might be allocated.


### Role ids

`role_id` field values appear in the message types listed below.
Their values are selected by means not specified in the P4Runtime
specification.

Scope: TBD

+ `Role`, where the field is called `id` instead of `role_id`
  + `MasterArbitrationUpdate` messages contain `Role` sub-messages
+ `SetForwardingPipelineConfigRequest`
+ `WriteRequest`


### Election ids

`election_id` field values appear in the message types listed below.
Their values are selected by P4Runtime clients.

Scope: I believe this value is scoped to a (P4Runtime server,
device_id) pair.

+ `MasterArbitrationUpdate`
+ `SetForwardingPipelineConfigRequest`
+ `WriteRequest`


### Multicast group ids

`multicast_group_id` field values appear in the message types listed
below.  The allowed set of values is determined by the P4Runtime
server, typically in a contiguous range `[1, M]` for some maximum
number of multicast groups `M` supported by the device, which can
differ from one device to another.

Scope: A single P4Runtime device.  For example, multicast group id 7
could have a different replication list for every device.

+ `MulticastGroupEntry`


### Clone session ids

`session_id` field values appear in the message types listed below.
The allowed set of values is determined by the P4Runtime server,
typically in a contiguous range `[0, C-1]` for some maximum number of
multicast groups `C` supported by the device, which can differ from
one device to another.

Scope: A single P4Runtime device.  For example, clone session id 5
could have a different configuration for every device.

+ `CloneSessionEntry`


### Digest list ids

`list_id` field values appear in the message types listed below.  They
are selected by the P4Runtime server by some means not specified in
the P4Runtime API specification, but it is implies that the value
should always be a `list_id` value that is not currently in use by an
unacknowledged `DigestList` message in the scope of a single P4Runtime
`device_id`.  The same `list_id` value, e.g. 203, could be
simultaneously in use by multiple devices managed by the same
P4Runtime server.  For a single device, a `list_id` value could be
used later in time, after it has been acknowledged via a
`DigestListAck` message.

Scope: A single P4Runtime device.

+ `DigestList`
+ `DigestListAck`


### ids of action profiles, and their members

TBD

```
// Note: All id values directly in a TableAction message (i.e. not
// within an Action sub-message) are dynamically chosen by the client
// at run time, and do not refer to any id values within a P4Info
// message.
// table_actions ::= action_specification | action_profile_specification
message TableAction {
  oneof type {
    Action action = 1;
    uint32 action_profile_member_id = 2;
    uint32 action_profile_group_id = 3;
    ActionProfileActionSet action_profile_action_set = 4;
  }
}

message ActionProfileMember {
  uint32 action_profile_id = 1;
  uint32 member_id = 2;
  Action action = 3;
}

message ActionProfileGroup {
  uint32 action_profile_id = 1;
  uint32 group_id = 2;
  message Member {
    uint32 member_id = 1;
    int32 weight = 2;
    uint32 watch = 3;
  }
  repeated Member members = 3;
}
```


### ids of action selectors, their groups, and members within those groups

TBD

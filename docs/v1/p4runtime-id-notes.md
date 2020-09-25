# P4Runtime numeric id notes

There are many kinds of numeric identifiers used in the P4Runtime API
specification.  While they are all described in the specification and
accompanying Protobuf `.proto` files, this article attempts to collect
in one place notes on those identifiers, including:

+ Which software entity (or person) chooses their values
+ When the values are chosen
+ What is their scope in "space" and time in which they must be unique

This document was written at the time of the release of P4Runtime
v1.2.0. If you refer to other P4Runtime specification versions, you may
find some small inconsistencies with respect to this document.

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

Note that a numeric id value of 0 is not a valid id for any P4 object
described inside of a P4Info message.  In a few cases, an id that is
used as a reference may be 0 to mean "not a reference to any object".
A few special cases of 0 are called out in this article.

See the next major section for uses of id 0 outside of a P4Info
message.


## Numeric ids that are unique within the P4Info top level scope

The messages listed below are all of those that are sub-messages of
the `P4Info` message, and they contain a `Preamble` sub-message.  Each
`Preamble` message contains an `id` field, as well as several others
like `name`, `alias`, and `annotations`.  See the `Preamble` message
definition for the full list.

Section 6.3 "ID Allocation for P4Info Objects" of the P4Runtime API
specification says:

    The most significant 8 bits of the ID encodes the object type (as
    per Table 1).  [ ... ]  The remaining 24 bits must be generated in
    such a way that the resulting IDs must be globally unique in the
    scope of the P4Info message.

These ids are not required to be globally unique for a single P4Info
message, because they are allowed to be the same as the ids described
in the next section.  If we call the scope that contains the ids
listed in this section the P4Info top level scope, then all of these
ids must be unique within that scope.

All of these id values in this section and the next are selected
during compilation of the P4 program, either by the P4 compiler, or by
the P4 developer using an `@id` annotation on the P4 object.

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
| `implementation_id` | `Table` | `ActionProfile` | 0 means "this table has no `implementation` table property. |
| `direct_resource_ids` | `Table` | `DirectCounter` or `DirectMeter` | repeated so that a single table can refer to up to one of each |
| `id` | sub-message `ActionRef` inside `Table` | `Action` | `ActionRef` messages are repeated, with a separate one for each action the table might invoke.  `ActionRef` messages contain optional annotation and `Scope` values that are specific to how a particular table is allowed to use that `Action` |
| `table_ids` | `ActionProfile` | `Table` | repeated once for each `Table` that uses this `ActionProfile` object.  This data is redundant with the `implementation_id` field above, but has been included for the convenience of consumers of P4Info messages. |
| `direct_table_id` | `DirectCounter` | `Table` | This data is redundant with the `direct_resource_ids` field above, but has been included for the convenience of consumers of P4Info messages. |
| `direct_table_id` | `DirectMeter` | `Table` | This data is redundant with the `direct_resource_ids` field above, but has been included for the convenience of consumers of P4Info messages. |


# Numeric ids that appear outside of a P4Info message

These are all defined in the file `proto/p4/v1/p4runtime.proto`.

For those id values that can appear in a `ReadRequest` message, an id
of 0 is usually used to mean "read all objects of some kind,
regardless of their id".  Some additional notes on any special
considerations for ids with value 0 are described below.


## Numeric ids that are references to P4 objects in the relevant P4Info message

The fields in the table below are inside of messages defined in the
`p4runtime.proto` file.  The id values reference objects in the
relevant P4info message.  The table below gives the type of the P4
object referred to using the name of the message type as defined in
the `p4info.proto` file.

Note that some message names are common in both files, but their
fields differ from each other.  These message types have full names
that are qualified by which file they are in, so these identical
message type names do not conflict with each other.

| Field name | Message type (in `p4runtime.proto`) | Type of P4 object referred to (message type from `p4info.proto`) | Notes |
| ---------- | ------------ | ----------------------------- | ----- |
| `table_id` | `TableEntry` | `Table` | |
| `value_set_id` | `ValueSetEntry` | `ValueSet` | |
| `field_id` | `FieldMatch` sub-message of `TableEntry` (or `ValueSetMember`) | `MatchField` of `Table` (or `ValueSet`) | |
| `action_id` | `Action` sub-message of `TableAction` | `Action` (in P4Info message) | Note that there are two different `Action` message types. |
| `param_id` | `Param` sub-message of `Action` | `Param` sub-message of `Action` | Note there are two different `Param` message types, and each is a sub-message of a different `Action` message type. |
| `counter_id` | `CounterEntry` | `Counter` | an indexed counter (aka indirect) |
| `meter_id` | `MeterEntry` | `Meter` | an indexed meter (aka indirect) |
| `register_id` | `RegisterEntry` | `Register` | |
| `digest_id` | `DigestEntry` | `Digest` | `DigestEntry` messages are used to configure a `Digest` object as a whole, not to deal with digest messages. |
| `digest_id` | `DigestList` | `Digest` | `DigestList` messages are sent by a server to a client, to report a list of digest messages generated by a P4 device. |
| `digest_id` | `DigestListAck` | `Digest` | `DigestListAck` mssages are sent by a client to acknowledge an earlier `DigestList` message that it received. |
| `extern_id` | `ExternEntry` | `ExternInstance` | |
| `metadata_id` | `PacketMetadata` sub-message of `PacketIn` (or `PacketOut`) | `Metadata` sub-message of `ControllerPacketMetadata` | Refers to a `Metadata` message in the `ControllerPacketMetadata` object whose name is `packet_in` (or `packet_out`). |
| `action_profile_id` | `ActionProfileMember` | `ActionProfile` | `ActionProfileMember` messages may reference an `ActionProfile` object with `with_selector` equal to true or false, i.e. either an action selector or an action profile object |
| `action_profile_id` | `ActionProfileGroup` | `ActionProfile` | `ActionProfileMember` messages must reference an `ActionProfile` object with `with_selector` equal to true, indicating it is an action selector, not an action profile. |


## Numeric ids that are NOT references to P4 objects

The ids in this section have nothing to do with any id values of P4
objects in a P4 program.


### Device ids

Scope: Unique within a single P4Runtime server.

`device_id` field values appear in the message types listed below.
Their values are selected by means not specified in the P4Runtime
specification.

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

As of P4Runtime API version 1.x, `device_id` 0 is used by some servers
and clients as a normal `device_id` value, i.e. to name a P4 target
device.  An issue has been created to consider changing this for
P4Runtime API version 2.0:
https://github.com/p4lang/p4runtime/issues/291.


### Role ids

Scope: TBD

`role_id` field values appear in the message types listed below.
Their values are selected by clients.  The P4Runtime API specification
does not define the meaning of any numeric values of role id, with the
exception of 0.  A role id of 0 is always a role that can modify
anything that is modifiable via the P4Runtime API.

+ `Role`, where the field is called `id` instead of `role_id`
  + `MasterArbitrationUpdate` messages contain `Role` sub-messages
+ `SetForwardingPipelineConfigRequest`
+ `WriteRequest`

There is no known use case for a P4 data plane to know role id values.


### Election ids

Scope: A single P4Runtime device.

`election_id` field values appear in the message types listed below.
Their values are selected by P4Runtime clients.

+ `MasterArbitrationUpdate`
+ `SetForwardingPipelineConfigRequest`
+ `WriteRequest`

A client may send a message with an `election_id` of 0, and this is
always interpreted by the server to be lower than any other
`election_id` value, and indicates that, when sending that message,
the client is not attempting to become the primary client.

There is no known use case for a P4 data plane to know election id
values.


### Multicast group ids

Scope: A single P4Runtime device.

`multicast_group_id` field values appear in the message types listed
below.  The allowed set of values is determined by the P4Runtime
server, typically in a contiguous range `[1, M]` for some maximum
number of multicast groups `M` supported by the device, which can
differ from one device to another.

A particular multicast group id value X (e.g. X=7) could have a
different replication list for every device.

+ `MulticastGroupEntry`

Multicast group id values must be used by the P4 data plane to direct
packets to be replicated using the replication list configured for
that multicast group id.  Thus it is likely that a P4Runtime client
would first configure a multicast group id, then write that id into a
place readable by the P4 program, e.g. as a parameter of some action
of a table.

(It is also possible that a P4 program might have some multicast group
id values compiled into it for use for particular purposes, in which
case it is up to some agreement between the developer of the P4
program and the controller software.)


### Clone session ids

Scope: A single P4Runtime device.

`session_id` field values appear in the message types listed below.
The allowed set of values is determined by the P4Runtime server,
typically in a contiguous range `[1, C]` for some maximum number of
clone sessions `C` supported by the device, which can differ from one
device to another.

A particular clone session id value X (e.g. X=5) could have a
different clone session configuration for every device.

+ `CloneSessionEntry`

Like multicast group ids, clone session id values must be used by the
P4 data plane to direct packets to be cloned using the session with
that clone session id.  Thus it is likely that a P4Runtime client
would first configure a clone session id, then write that id into a
place readable by the P4 program, e.g. as a parameter of some action
of a table.

(It is also possible that a P4 program might have some clone session
id values compiled into it for use for particular purposes, in which
case it is up to some agreement between the developer of the P4
program and the controller software.)


### Digest list ids

Scope: A single P4Runtime device.

`list_id` field values appear in the message types listed below.  They
are selected by the P4Runtime server by some means not specified in
the P4Runtime API specification, but it is implied that when a server
sends a new `DigestList` message, it must contain a `list_id` value
that is not in the set of unacknowledged `list_id` values for the
device.  For a single device, a `list_id` value could be used later in
time, after it has been acknowledged via a `DigestListAck` message.

The same `list_id` value, e.g. 203, could be simultaneously in use by
multiple devices managed by the same P4Runtime server.

+ `DigestList`
+ `DigestListAck`

It is still TBD whether a digest list id of 0 has any special meaning
in the P4Runtime API.

There is no known use case for a P4 data plane to know digest list id
values.


### ids of action profiles, and their members

Scope: The field values named `action_profile_member_id` and
`member_id` have a scope of a single action profile extern object
instance within a single P4Runtime device.

Consider a P4 program with an ActionProfile extern object named `ap1`.
This program has 4 tables named `t1` through `t4` that all have a
table property `implementation = ap1;` in their definitions.  Several
P4_16 architectures, including v1model and PSA, enable any number of
tables to have the same ActionProfile object used in this way.

For every ActionProfile object, including `ap1`, the P4Runtime server
maintains a set of members at run time.  These members are identified
using non-0 integer values with type `uint32`.  The P4Runtime clients
choose which integer values to use.  The set of integers need not be
restricted to a consecutive range of values starting at 1.

The effect is that logically there is a mapping `M` (`M` for
"members") for `ap1` that, given one of these `uint32` integer values,
maps it to an action name, and some parameter values, all selected by
the client.  The client directs the server to update the contents of
`M` by sending `ActionProfileMember` messages.  Each message either
inserts, deletes, or modifies one integer-to-action element of `M`.

Suppose that at some point in time, the client has successfully added
to `ap1` a member with id 27, and the action `my_act5`.

Suppose also that the client has added an entry to table `t2` with
some set of key match conditions, using a `TableAction` message with
field `action_profile_member_id` equal to 27.  Note that adding such a
table entry can only succeed _after_ an entry has been added to `ap1`
with id 27.

After that state has been established in the data plane, when a packet
is being processed and `t2.apply()` is called, and the key matches
this table entry, then action `my_act5` will be executed by the P4
data plane, as the result of calling `apply` on that table.

If there are multiple ActionProfile extern object instances, the
server maintains state independently for each one.

There is no known use case for a P4 data plane to know P4Runtime API
action profile member id values.  It is likely that some P4 data plane
implementations will use "data plane member id" values as a technique
for implementing action profiles, but these values are not visible to
the P4 program.  If such values are in use, it is likely that they
will be much smaller than 32 bits wide, and if this is the case, the
P4Runtime server is responsible for managing them and performing any
needed translation between the P4Runtime API 32-bit values and the
data plane values.


### ids of action selectors, their groups, and members within those groups

Scope:

+ The field values named `action_profile_group_id` and `group_id` have
  a scope of a single action selector extern object instance within a
  single P4Runtime device.
+ The field values named `action_profile_member_id` and `member_id`
  have a scope of a single action selector extern object instance
  within a single P4Runtime device.

Action selectors are more general than action profiles.  They add
another optional level of indirection, but it is most common to use
them when you want that extra level of indirection -- there is not
much purpose to use an action selector if all you want are the
features of action profiles.

Like action profiles, one must still create members with ids that
refer to an "action plus its control plane parameters".  For tables
using an action selector, the client can add entries to those tables
that refer to member ids directly, just as one can for tables with
action profiles.

The more common use case for action selectors is to create groups,
which are sets of members.

The basic idea is that for every entry in a table that has an action
selector implementation, it either refers to a member id directly, or
to a group id, where every group has been configured by the client as
a set of member ids (and a couple of other properties described
later).  If it refers to a group id, one of the eligible members of
that group is chosen by the data plane via some mechanism (examples
below), and this gives you a member id.  Once a member id has been
determined, the action plus parameters configured for that member id
are invoked.

Among the possible mechanisms for selecting one member among a group,
the most common is to calculate a hash function of a selected set of
fields from one or more packet headers (e.g. the IPv4 source and
destination address, IPv4 protocol, and if the packet is TCP or UDP,
the source and destination port numbers).  If the group has N members,
then take the hash function modulo N, and the result indicates which
member to select.  A common use case for action selectors is to select
one of several shortest paths calculated by a routing protocol to
packet's destination, called [Equal Cost
Multi-Path](https://en.wikipedia.org/wiki/Equal-cost_multi-path_routing)
routing.

Aside: The P4Runtime API provides two different ways to control the
state of an ActionSelector object.  The one described here uses
`ActionProfileMember` and `ActionProfileGroup` messages, and requires
the clients to choose member id and group id values to identify
members and groups.  The other way is called "one shot" action
selector programming, and eliminates the need of clients to choose and
manage these id values.  There is a tradeoff here -- using the API
described here provides more precise control if a client wishes to
reuse common members in multiple groups, whereas the one shot API does
not enable this.  See the P4Runtime API documentation, in the section
"One Shot Action Selector Programming", for more details.

In the P4Runtime API, the action selector groups are identified by
non-0 integer id values with type `uint32`.  The member id and group
id values for a single action selector object are in separate
"scopes", i.e. a single action selector can have a member with id 5 at
the same time as it has a group with id 5, and those are different
things.  The member ids are inserted, modified, and deleted using
`ActionProfileMember` messages, just as for action profiles.  The
group ids are inserted, modified, and/or deleted using
`ActionProfileGroup` messages.

Consider a P4 program with an ActionSelector extern object named
`as1`.  This program has 4 tables named `t5` through `t8` that all
have a table property `implementation = as1;` in their definitions.
As for action profiles, several P4_16 architectures enable any number
of tables to have the same ActionSelector object used in this way.

For every ActionSelector object, the P4Runtime server maintains a set
of members, and also a set of groups, at run time.

Just as for action profiles, there is a mapping `M` for `as1` that
maps each member id integer value to an action name and action
parameter values.  A client can modify this mapping using
`ActionProfileMember` messages.

Unique to ActionSelector objects is that there is also a mapping `G`
for `as1` that maps each group id integer value to a group, which is a
set of tuples of the form `(member_id, weight, watch)`.

Each `member_id` value must equal the id of some member already added
earlier to the action selector.

`weight` is a positive integer value that specifies the relative
frequency with which this member should be selected, within a single
group.

`watch` is an integer that identifies a port on the device.  This is a
32-bit value that is called an "SDN port" in the P4Runtime API
specification.  If this value is 0, this feature is not applicable to
the group member, and the member is always eligible for selection.  If
`watch` is equal to a port id of the device, then the member is
eligible only when the port is currently operational.

There is no known use case for a P4 data plane to know P4Runtime API
action selector group id or member id values.  It is likely that some
P4 data plane implementations will use "data plane group id" values as
a technique for implementing action selectors (and similar for member
ids), but these values are not visible to the P4 program.  If such
values are in use, it is likely that they will be much smaller than 32
bits wide, and if this is the case, the P4Runtime server is
responsible for managing them and performing any needed translation
between the P4Runtime API 32-bit values and the data plane values.

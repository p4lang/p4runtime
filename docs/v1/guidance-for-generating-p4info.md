# Guidance for generating P4Info messages

The P4Runtime v1.0 specification restricts the types that it supports
for the following kinds of things:

+ table search key fields, defined in the P4Info message in a
  `MatchField` message
+ fields of a `ValueSet`, also defined in the P4Info message in a
  `MatchField` message
+ parameters specified by the control plane for a table action,
  defined in the P4Info message in a `Param` message
+ metadata fields in a header sent from data plane to controller, or
  from controller to the data plane, defined in the P4Info message in
  a `Metadata` message, if a [recently proposed
  PR](https://github.com/p4lang/p4runtime/pull/188) is merged in.

Later in this section, we will use the term "constrained value" for
brevity, instead of repeating all of kinds of objects listed above.
For such values, the P4Runtime v1.0 supports all of the following
types, but currently no others:

+ `bit<W>`
+ an `enum` with an underlying type of `bit<W>`, also called a
  serializable `enum`
+ a `typedef` or `type` name that, when "followed back" to the lowest
  base type, is one of the above.  (As of the P4_16 language
  specification version 1.1, it is not required to support a `type`
  definition with a serializable `enum` as its base type.  See
  [p4runtime issue
  #192](https://github.com/p4lang/p4runtime/issues/192).)

P4Info `MatchField`, `Param`, and probably soon also `Metadata`
messages may optionally contain the following two fields:

+ `int32 bitwidth`
+ `P4NamedType type_name`

Below we will describe what values these fields should have.

Consider a single constrained value `x`.  Create a list of types
`type_list(x)` using the pseudocode shown below.

```
type_list(x) {
    tlist = [];    // tlist initialized to empty list
    T = declared type of object x in the P4 program;
    while (true) {
        if (T is declared as "type B T") {
            tlist = tlist + [T];   // append T to end of tlist
            T = B;
        } else if (T is declared as "typedef B T") {
            T = B;
        } else {
            tlist = tlist + [T];   // append T to end of tlist
            return tlist;
        }
    }
}
```

Note that `type_list(x)` always starts with zero or more `type` names,
and always ends with one type that is neither a `type` nor `typedef`
name, e.g. `bit<W>`, a header type, struct type, etc.  It never
contains the name of a type declared using `typedef`.  P4Runtime v1.0
only supports `p4runtime_translation` annotations on `type`
definitions.  If any occur on a `typedef` definition, they should be
ignored.

The p4c compiler signals an error if you attempt to create a cycle of
type names.  In order to create such a cycle, the first `type` or
`typedef` that appears in the program would have to refer to a later
type name, and this is not allowed.

If the last type is not `bit<W>` or `enum bit<W>`, that is an error
for P4Runtime v1.0.  The "base" type must always be one of those for
every constrained value.


### `type_name` field

Let `first_type` be the first element of the list `type_list(x)`.

If `first_type` is a `type` name (i.e. not `bit<W>` or `enum bit<W>`),
then the value of the P4Info `type_name` field should be `{name =
"first_type_name"}`, where `first_type_name` is the name of
`first_type`.

Otherwise, the `type_name` field should be unset in the P4info
message.


### `bitwidth` field

If `first_type` is a `type` name, _and_ if the `type` definition for
this type has a `p4runtime_translation(uri_string, n)` annotation in
the source code, then the P4Info `bitwidth` field should be assigned
the value `n` that is the second parameter of that
`p4runtime_translation` annotation.

Otherwise, `bitwidth` should be equal to `W` where `bit<W>` or `enum
bit<W>` is the last element of `type_list(x)`.

Note that all type names that are present as the value of a
`type_name` field anywhere in a P4Info message must contain an entry
somewhere in the `type_info` field of the P4Info message.  Thus this
`bitwidth` value could be considered redundant information, since it
can be derived from the description of the type contained within the
`type_info` field (see [this
comment](https://github.com/p4lang/p4runtime/issues/189#issuecomment-471240515)).


## Example 1: field with simple `bit<W>` type

```
bit<10> f1;

type_list(f1) -> [bit<10>]

type_name: left unset in P4Info message
bitwidth: 10
```

Based on the P4 code snippet above, there is no need to set any fields
inside of the type_info field of the P4Info message, because there are
no named types in that code.


## Example 2: field with user-defined `type` with no annotation

```
typedef bit<10> T1uint_t;
@p4runtime_translation("mycompany.com/psa/v1/T1_t", 32)
type T1uint_t T1_t;
type T1_t T2_t;
T2_t f2;
```

```
Execution trace for call to type_list(f2):
    T = declared type of object f2 in the P4 program = T2_t
    Evaluate condition (T2_t is declared as "type B T") -> true,
        because T2_t is declared as "type T1_t T2_t"
    tlist = tlist + [T] -> tlist=[T2_t]
    T = B = T1_t
    Evaluate condition (T1_t is declared as "type B T") -> true,
        because T1_t is declared as "type T1uint_t T1_t"
    tlist = tlist + [T] -> tlist=[T2_t, T1_t]
    T = B = T1uint_t
    Evaluate condition (T1uint_t is declared as "type B T") -> false
    Evaluate condition (T1uint_t is declared as "typedef B T") -> true,
        because T1uint_t is declared as "typedef bit<10> T1uint_t"
    T = B = bit<10>
    Evaluate condition (bit<10> is declared as "type B T") -> false
    Evaluate condition (bit<10> is declared as "typedef B T") -> false
    tlist = tlist + [T] -> tlist=[T2_t, T1_t, bit<10>]
    return tlist

type_list(f2) -> [T2_t, T1_t, bit<10>]

type_name: "T2_t"

    Reason: T2_t is the first type name in type_list(f2)

bitwidth: 10

    Reason: Type T2_t is the first type name in type_list(f2), but it
    has no p4runtime_translation on it, so even though T1_t does, that
    is ignored.  Use the width 10 from the last element of
    type_list(f2).
```

Based on the P4 code snippet above (copied below for easy reference),
the value below starting with `type_info {` should be in the P4Info
message describing the program, because of the `type` definitions.
There is never anything put into a P4Info message because of `typedef`
definitions in a P4 program.

Note that the bit width of 10 appears in the P4Info message for any
`type`s "built on top of" a `bit<10>`, _unless that type has its own
`p4runtime_translation` annotation_.

```
typedef bit<10> T1uint_t;
@p4runtime_translation("mycompany.com/psa/v1/T1_t", 32)
type T1uint_t T1_t;
type T1_t T2_t;
T2_t f2;
```

```
type_info {
  new_types {
    key: "T2_t"
    value {
      original_type {
        bitstring {
          bit {
            bitwidth: 10
          }
        }
      }
    }
  }
}
```

Type `T2_t` is described via an `original_type` message, because
`T2_t` does not have a `p4runtime_translation` annotation.

Based on the P4 code snippet above, there is no reason to include a
definition for the type `T1_t` in the P4Info message.  If there were
some other variable other than `f2` in the program declared with type
`T1_t`, then the following definition for type `T1_t` must be included
in the `type_info` field.

```
type_info {
  new_types {
    key: "T1_t"
    value {
      translated_type {
        uri: "mycompany.com/psa/v1/T1_t"
        sdn_bitwidth: 32
      }
    }
  }
}
```

Type `T1_t` is described via a `translated_type` message, because
`T1_t` has a `p4runtime_translation` annotation.


## Example 3: field with user-defined `type` with `p4runtime_translation` annotation

It is not clear whether there are strong use cases for declaring a
`type` based upon another `type` in a P4_16 program.

However, currently the P4_16 language and `p4c` compiler allow it, so
it seems to be a good idea to have predictable rules to follow for
what the P4Info message contents should be, and how the resulting
system should behave.

The basic design described here tries to keep things fairly
straightforward to explain and understand, if a P4_16 program does so.

If a constrained value is declared with a `type` that has a
`p4runtime_translation` on it, that one is used.

In the absence of such an annotation on that `type`, no P4runtime
translation is done for that type, _even if a later type in
`type_list(x)` does have such an annotation_.  The final type in
`type_list(x)` is used.

Below is an example of a P4 code snippet that demonstrates one
example, but I do _not_ claim that it is useful for any actual
production P4 program to be written this way (until and unless some
demonstrates a reason why it would be useful).

```
@p4runtime_translation("mycompany.com/psa/v1/T1_t", 32)
type bit<10> T1_t;
@p4runtime_translation("mycompany.com/psa/v1/T2_t", 18)
type T1_t T2_t;
T2_t f3;

type_list(f3) -> [T2_t, T1_t, bit<10>]

type_name: "T2_t"

    Reason: T2_t is the first type name in type_list(f3)

bitwidth: 18

    Reason: Type T2_t is the first type name in type_list(f3), and it
    has a p4runtime_translation annotation on it.  Use the width 18
    that is the second parameter of that annotation.
```

The contents of the `type_info` field of the P4Info message should be
as follows.  For the code snippet above, there is no reason to include
a description of the type `T1_t`.  It is shown below for the sake of
an example, but it only needs to be included in the P4Info message if
there is a reference directly to type `T1_t` somewhere in the P4Info
message.

```
type_info {
  new_types {
    key: "T1_t"
    value {
      translated_type {
        uri: "mycompany.com/psa/v1/T1_t"
        sdn_bitwidth: 32
      }
    }
  }
  new_types {
    key: "T2_t"
    value {
      translated_type {
        uri: "mycompany.com/psa/v1/T2_t"
        sdn_bitwidth: 18
      }
    }
  }
}
```

Note that both types `T1_t` and `T2_t` are described via
`translated_type` messages, because they both have a
`p4runtime_translation` annotation.


## Example 4: field with serializable `enum` type

See [p4runtime issue
#192](https://github.com/p4lang/p4runtime/issues/192).

```
enum bit<10> enum1_t {
    A = 1,
    B = 2
}

enum1_t f4;

type_list(f1) -> [enum1_t]

type_name: see discussion below
bitwidth: 10
```

It seems pretty clear that `bitwidth` should be 10 bits for field
`f4`.

However, what about `type_name`?

Should `type_name` be unset?  If so, there is the disadvantage that
the control plane software has no indication that field `f4` is of
type `enum1_t`.  As far as it could tell from the P4Info message, it
is declared as type `bit<10>`.  This is not necessarily a fatal flaw,
but it is a loss of potentially useful information in the P4Info
message.

Should `type_name` be `"enum1_t"`, and then `"enum_t"` should be
described within the `type_info` field of the message?

As of early March 2019, `p4c` does not support users defining their
own `type` definitions with a serializable `enum` like `enum1_t` as
the base type.  The disadvantage with this situation is that there is
no way to define a serializable `enum` type for a constrained value,
and also have `p4runtime_translation` annotated on a `type` with the
serializable `enum` as its underlying type.

If `p4c` ever does support defining a `type` with a serializable
`enum` as its underlying type, with an optional
`p4runtime_translation` annotation, then we should think about how
that should be represented in a P4Info message, and add an example of
it here.

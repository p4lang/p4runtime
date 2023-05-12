# GenericTable example p4info

Below is an example for realizing MulticastGroupEntry as a GenericTable.
The key (MatchFields) comprise of only the group_id. 2 different ways of
defining the data fields has been presented. Only 1 of them is required.

```
generic_tables {
  generic_table_type_id : 145
  generic_table_type_name : "MulticastGroup"
  preamble {
    id: 45332650
    name: "MulticastGroup"
    alias: "multicast_group"
  }
  generic_match_fields {
    id: 1
    name: "multicast_group_id"
    match_type: EXACT
    type {
      type : "bytes"
      width : 32
    }
  }
  union_refs {
    id: 23557840
  }
  size: 1024
}
```

In the below one, both `instance` and `port` are separate
repeated fields. So the check on `len(instance_array) == len(port_array)`
needs to be a runtime check. This however keeps implementation simpler
and faster since we avoid further recursive nesting.

`port` is a varbytes of max size 64 bits each. The field is repeated so it
is defined as list of varbits through p4info.

```
unions {
  preamble {
    id: 23557840
    name: "multicast_group_member_add"
    alias: "multicast_group_member_add"
  }
  params {
    id: 1
    name: "instance"
    repeated: true
    type {
      type : "bytes"
      width : 32
    }
  }
  params {
    id: 2
    name: "port"
    repeated: true
    type {
      type : "varbytes"
      max_bit_width : 64
    }
  }
}

```

The below one is similar to the above but both `instance` and `port` have
been converted to a list of structs.

```
unions {
  preamble {
    id: 23557841
    name: "multicast_group_member_add_2"
    alias: "multicast_group_member_add_2"
  }
  params {
    id: 1
    name: "replica"
    repeated: true
    type {
      type : "list"
    }
    params {
      param {
        id: 1
        name: "instance"
        repeated: true
        type {
          type : "bytes"
          width : 32
        }
      }
      param : {
        id: 2
        name: "port"
        repeated: true
        type {
          type : "varbytes"
          max_bit_width : 64
        }
      }
    }
  }
}
```

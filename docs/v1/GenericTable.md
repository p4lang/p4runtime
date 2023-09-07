# GenericTable example p4info

Below is an example for realizing MulticastGroupEntry as a GenericTable.
The key (MatchFields) comprise of only the group_id. 2 different ways of
defining the data fields has been presented. Only 1 of them is required.

```
generic_tables {
  type_id : 145
  type_name : "MulticastGroup"
  properties : {
    "indexed"
  }
  instances {
    preamble {
      id: 45332650
      name: "MulticastGroup"
      alias: "multicast_group"
    }
    generic_match_fields {
      id: 1
      name: "multicast_group_id"
      match_type: EXACT
      type_spec {
        bitstring {
          bit {
            bitwidth : 32
          }
        }
      }
    }
    union_refs {
      id: 23557840
    }
    size: 1024
  }
}
```

In the below one, both `instance` and `port` are separate
repeated fields. So the check on `len(instance_array) == len(port_array)`
needs to be a runtime check. This however keeps implementation simpler
and faster since we avoid further recursive nesting.

`port` is a varbytes of max size 64 bits each. The field is a list so
it is defined as list of varbits through p4info.

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
    type_spec {
      list {
        element_type_spec {
          bitstring {
            bit {
              bitwidth : 32
            }
          }
        }
        max_size : 10
      }
    }
  }
  params {
    id: 2
    name: "port"
    type_spec {
      list {
        type_spec {
          bitstring {
            varbit {
              max_bitwidth : 64
            }
          }
        }
        max_size : 10
      }
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
    type_spec {
      list {
        element_type_spec {
          struct {
            members {
              id : 1
              name : "instance"
              type_spec {
                bitstring {
                  bit {
                    bitwidth : 32
                  }
                }
              }
            }
            members {
              id : 2
              name : "port"
              type_spec {
                bitstring {
                  varbit {
                    max_bitwidth : 64
                  }
                }
              }
            }
          }
        }
        max_size : 10
      }
    }
  }
}
```

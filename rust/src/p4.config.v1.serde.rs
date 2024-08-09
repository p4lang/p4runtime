// @generated
impl serde::Serialize for Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if !self.params.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Action", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Params,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Action")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Action {
                    preamble: preamble__,
                    params: params__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Action", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for action::Param {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if self.bitwidth != 0 {
            len += 1;
        }
        if self.doc.is_some() {
            len += 1;
        }
        if self.type_name.is_some() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Action.Param", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if self.bitwidth != 0 {
            struct_ser.serialize_field("bitwidth", &self.bitwidth)?;
        }
        if let Some(v) = self.doc.as_ref() {
            struct_ser.serialize_field("doc", v)?;
        }
        if let Some(v) = self.type_name.as_ref() {
            struct_ser.serialize_field("typeName", v)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for action::Param {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "bitwidth",
            "doc",
            "type_name",
            "typeName",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Annotations,
            AnnotationLocations,
            Bitwidth,
            Doc,
            TypeName,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "bitwidth" => Ok(GeneratedField::Bitwidth),
                            "doc" => Ok(GeneratedField::Doc),
                            "typeName" | "type_name" => Ok(GeneratedField::TypeName),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = action::Param;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Action.Param")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<action::Param, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut bitwidth__ = None;
                let mut doc__ = None;
                let mut type_name__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bitwidth => {
                            if bitwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitwidth"));
                            }
                            bitwidth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Doc => {
                            if doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            doc__ = map_.next_value()?;
                        }
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = map_.next_value()?;
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(action::Param {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    bitwidth: bitwidth__.unwrap_or_default(),
                    doc: doc__,
                    type_name: type_name__,
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Action.Param", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActionProfile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if !self.table_ids.is_empty() {
            len += 1;
        }
        if self.with_selector {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if self.max_group_size != 0 {
            len += 1;
        }
        if self.selector_size_semantics.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ActionProfile", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if !self.table_ids.is_empty() {
            struct_ser.serialize_field("tableIds", &self.table_ids)?;
        }
        if self.with_selector {
            struct_ser.serialize_field("withSelector", &self.with_selector)?;
        }
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        if self.max_group_size != 0 {
            struct_ser.serialize_field("maxGroupSize", &self.max_group_size)?;
        }
        if let Some(v) = self.selector_size_semantics.as_ref() {
            match v {
                action_profile::SelectorSizeSemantics::SumOfWeights(v) => {
                    struct_ser.serialize_field("sumOfWeights", v)?;
                }
                action_profile::SelectorSizeSemantics::SumOfMembers(v) => {
                    struct_ser.serialize_field("sumOfMembers", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActionProfile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "table_ids",
            "tableIds",
            "with_selector",
            "withSelector",
            "size",
            "max_group_size",
            "maxGroupSize",
            "sum_of_weights",
            "sumOfWeights",
            "sum_of_members",
            "sumOfMembers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            TableIds,
            WithSelector,
            Size,
            MaxGroupSize,
            SumOfWeights,
            SumOfMembers,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "tableIds" | "table_ids" => Ok(GeneratedField::TableIds),
                            "withSelector" | "with_selector" => Ok(GeneratedField::WithSelector),
                            "size" => Ok(GeneratedField::Size),
                            "maxGroupSize" | "max_group_size" => Ok(GeneratedField::MaxGroupSize),
                            "sumOfWeights" | "sum_of_weights" => Ok(GeneratedField::SumOfWeights),
                            "sumOfMembers" | "sum_of_members" => Ok(GeneratedField::SumOfMembers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionProfile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ActionProfile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActionProfile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut table_ids__ = None;
                let mut with_selector__ = None;
                let mut size__ = None;
                let mut max_group_size__ = None;
                let mut selector_size_semantics__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::TableIds => {
                            if table_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableIds"));
                            }
                            table_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::WithSelector => {
                            if with_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withSelector"));
                            }
                            with_selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxGroupSize => {
                            if max_group_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGroupSize"));
                            }
                            max_group_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SumOfWeights => {
                            if selector_size_semantics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sumOfWeights"));
                            }
                            selector_size_semantics__ = map_.next_value::<::std::option::Option<_>>()?.map(action_profile::SelectorSizeSemantics::SumOfWeights)
;
                        }
                        GeneratedField::SumOfMembers => {
                            if selector_size_semantics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sumOfMembers"));
                            }
                            selector_size_semantics__ = map_.next_value::<::std::option::Option<_>>()?.map(action_profile::SelectorSizeSemantics::SumOfMembers)
;
                        }
                    }
                }
                Ok(ActionProfile {
                    preamble: preamble__,
                    table_ids: table_ids__.unwrap_or_default(),
                    with_selector: with_selector__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                    max_group_size: max_group_size__.unwrap_or_default(),
                    selector_size_semantics: selector_size_semantics__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ActionProfile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for action_profile::SumOfMembers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_member_weight.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ActionProfile.SumOfMembers", len)?;
        if let Some(v) = self.max_member_weight.as_ref() {
            struct_ser.serialize_field("maxMemberWeight", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for action_profile::SumOfMembers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_member_weight",
            "maxMemberWeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxMemberWeight,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxMemberWeight" | "max_member_weight" => Ok(GeneratedField::MaxMemberWeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = action_profile::SumOfMembers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ActionProfile.SumOfMembers")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<action_profile::SumOfMembers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_member_weight__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxMemberWeight => {
                            if max_member_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxMemberWeight"));
                            }
                            max_member_weight__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(action_profile::SumOfMembers {
                    max_member_weight: max_member_weight__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ActionProfile.SumOfMembers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for action_profile::SumOfWeights {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.config.v1.ActionProfile.SumOfWeights", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for action_profile::SumOfWeights {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = action_profile::SumOfWeights;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ActionProfile.SumOfWeights")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<action_profile::SumOfWeights, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(action_profile::SumOfWeights {
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ActionProfile.SumOfWeights", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActionRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.scope != 0 {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ActionRef", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.scope != 0 {
            let v = action_ref::Scope::try_from(self.scope)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.scope)))?;
            struct_ser.serialize_field("scope", &v)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActionRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "scope",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Scope,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "scope" => Ok(GeneratedField::Scope),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ActionRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActionRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut scope__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = Some(map_.next_value::<action_ref::Scope>()? as i32);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ActionRef {
                    id: id__.unwrap_or_default(),
                    scope: scope__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ActionRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for action_ref::Scope {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::TableAndDefault => "TABLE_AND_DEFAULT",
            Self::TableOnly => "TABLE_ONLY",
            Self::DefaultOnly => "DEFAULT_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for action_ref::Scope {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TABLE_AND_DEFAULT",
            "TABLE_ONLY",
            "DEFAULT_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = action_ref::Scope;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TABLE_AND_DEFAULT" => Ok(action_ref::Scope::TableAndDefault),
                    "TABLE_ONLY" => Ok(action_ref::Scope::TableOnly),
                    "DEFAULT_ONLY" => Ok(action_ref::Scope::DefaultOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ControllerPacketMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ControllerPacketMetadata", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControllerPacketMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControllerPacketMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ControllerPacketMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControllerPacketMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ControllerPacketMetadata {
                    preamble: preamble__,
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ControllerPacketMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for controller_packet_metadata::Metadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if self.bitwidth != 0 {
            len += 1;
        }
        if self.type_name.is_some() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ControllerPacketMetadata.Metadata", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if self.bitwidth != 0 {
            struct_ser.serialize_field("bitwidth", &self.bitwidth)?;
        }
        if let Some(v) = self.type_name.as_ref() {
            struct_ser.serialize_field("typeName", v)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for controller_packet_metadata::Metadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "bitwidth",
            "type_name",
            "typeName",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Annotations,
            AnnotationLocations,
            Bitwidth,
            TypeName,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "bitwidth" => Ok(GeneratedField::Bitwidth),
                            "typeName" | "type_name" => Ok(GeneratedField::TypeName),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = controller_packet_metadata::Metadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ControllerPacketMetadata.Metadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<controller_packet_metadata::Metadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut bitwidth__ = None;
                let mut type_name__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bitwidth => {
                            if bitwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitwidth"));
                            }
                            bitwidth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = map_.next_value()?;
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(controller_packet_metadata::Metadata {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    bitwidth: bitwidth__.unwrap_or_default(),
                    type_name: type_name__,
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ControllerPacketMetadata.Metadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Counter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if self.index_type_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Counter", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        if let Some(v) = self.index_type_name.as_ref() {
            struct_ser.serialize_field("indexTypeName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Counter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "spec",
            "size",
            "index_type_name",
            "indexTypeName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Spec,
            Size,
            IndexTypeName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "spec" => Ok(GeneratedField::Spec),
                            "size" => Ok(GeneratedField::Size),
                            "indexTypeName" | "index_type_name" => Ok(GeneratedField::IndexTypeName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Counter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Counter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Counter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut spec__ = None;
                let mut size__ = None;
                let mut index_type_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IndexTypeName => {
                            if index_type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexTypeName"));
                            }
                            index_type_name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Counter {
                    preamble: preamble__,
                    spec: spec__,
                    size: size__.unwrap_or_default(),
                    index_type_name: index_type_name__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Counter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CounterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.CounterSpec", len)?;
        if self.unit != 0 {
            let v = counter_spec::Unit::try_from(self.unit)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.unit)))?;
            struct_ser.serialize_field("unit", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CounterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Unit,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CounterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.CounterSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CounterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value::<counter_spec::Unit>()? as i32);
                        }
                    }
                }
                Ok(CounterSpec {
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.CounterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for counter_spec::Unit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Bytes => "BYTES",
            Self::Packets => "PACKETS",
            Self::Both => "BOTH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for counter_spec::Unit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "BYTES",
            "PACKETS",
            "BOTH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = counter_spec::Unit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(counter_spec::Unit::Unspecified),
                    "BYTES" => Ok(counter_spec::Unit::Bytes),
                    "PACKETS" => Ok(counter_spec::Unit::Packets),
                    "BOTH" => Ok(counter_spec::Unit::Both),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Digest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if self.type_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Digest", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if let Some(v) = self.type_spec.as_ref() {
            struct_ser.serialize_field("typeSpec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Digest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "type_spec",
            "typeSpec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            TypeSpec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "typeSpec" | "type_spec" => Ok(GeneratedField::TypeSpec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Digest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Digest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Digest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut type_spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::TypeSpec => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeSpec"));
                            }
                            type_spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Digest {
                    preamble: preamble__,
                    type_spec: type_spec__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Digest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectCounter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.direct_table_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.DirectCounter", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if self.direct_table_id != 0 {
            struct_ser.serialize_field("directTableId", &self.direct_table_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectCounter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "spec",
            "direct_table_id",
            "directTableId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Spec,
            DirectTableId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "spec" => Ok(GeneratedField::Spec),
                            "directTableId" | "direct_table_id" => Ok(GeneratedField::DirectTableId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectCounter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.DirectCounter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DirectCounter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut spec__ = None;
                let mut direct_table_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::DirectTableId => {
                            if direct_table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directTableId"));
                            }
                            direct_table_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DirectCounter {
                    preamble: preamble__,
                    spec: spec__,
                    direct_table_id: direct_table_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.DirectCounter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectMeter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.direct_table_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.DirectMeter", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if self.direct_table_id != 0 {
            struct_ser.serialize_field("directTableId", &self.direct_table_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectMeter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "spec",
            "direct_table_id",
            "directTableId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Spec,
            DirectTableId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "spec" => Ok(GeneratedField::Spec),
                            "directTableId" | "direct_table_id" => Ok(GeneratedField::DirectTableId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectMeter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.DirectMeter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DirectMeter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut spec__ = None;
                let mut direct_table_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::DirectTableId => {
                            if direct_table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directTableId"));
                            }
                            direct_table_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DirectMeter {
                    preamble: preamble__,
                    spec: spec__,
                    direct_table_id: direct_table_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.DirectMeter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Documentation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.brief.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Documentation", len)?;
        if !self.brief.is_empty() {
            struct_ser.serialize_field("brief", &self.brief)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Documentation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "brief",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Brief,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "brief" => Ok(GeneratedField::Brief),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Documentation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Documentation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Documentation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut brief__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Brief => {
                            if brief__.is_some() {
                                return Err(serde::de::Error::duplicate_field("brief"));
                            }
                            brief__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Documentation {
                    brief: brief__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Documentation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Expression {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Expression", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                expression::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                expression::Value::Int64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("int64Value", ToString::to_string(&v).as_str())?;
                }
                expression::Value::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Expression {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "string_value",
            "stringValue",
            "int64_value",
            "int64Value",
            "bool_value",
            "boolValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StringValue,
            Int64Value,
            BoolValue,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "int64Value" | "int64_value" => Ok(GeneratedField::Int64Value),
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Expression;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Expression")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Expression, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(expression::Value::StringValue);
                        }
                        GeneratedField::Int64Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Value"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| expression::Value::Int64Value(x.0));
                        }
                        GeneratedField::BoolValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(expression::Value::BoolValue);
                        }
                    }
                }
                Ok(Expression {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Expression", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expressions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ExpressionList", len)?;
        if !self.expressions.is_empty() {
            struct_ser.serialize_field("expressions", &self.expressions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expressions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expressions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "expressions" => Ok(GeneratedField::Expressions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpressionList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ExpressionList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpressionList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expressions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expressions => {
                            if expressions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expressions"));
                            }
                            expressions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExpressionList {
                    expressions: expressions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ExpressionList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Extern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extern_type_id != 0 {
            len += 1;
        }
        if !self.extern_type_name.is_empty() {
            len += 1;
        }
        if !self.instances.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Extern", len)?;
        if self.extern_type_id != 0 {
            struct_ser.serialize_field("externTypeId", &self.extern_type_id)?;
        }
        if !self.extern_type_name.is_empty() {
            struct_ser.serialize_field("externTypeName", &self.extern_type_name)?;
        }
        if !self.instances.is_empty() {
            struct_ser.serialize_field("instances", &self.instances)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Extern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extern_type_id",
            "externTypeId",
            "extern_type_name",
            "externTypeName",
            "instances",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternTypeId,
            ExternTypeName,
            Instances,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "externTypeId" | "extern_type_id" => Ok(GeneratedField::ExternTypeId),
                            "externTypeName" | "extern_type_name" => Ok(GeneratedField::ExternTypeName),
                            "instances" => Ok(GeneratedField::Instances),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Extern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Extern")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Extern, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extern_type_id__ = None;
                let mut extern_type_name__ = None;
                let mut instances__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExternTypeId => {
                            if extern_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externTypeId"));
                            }
                            extern_type_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExternTypeName => {
                            if extern_type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externTypeName"));
                            }
                            extern_type_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Instances => {
                            if instances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instances"));
                            }
                            instances__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Extern {
                    extern_type_id: extern_type_id__.unwrap_or_default(),
                    extern_type_name: extern_type_name__.unwrap_or_default(),
                    instances: instances__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Extern", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExternInstance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if self.info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ExternInstance", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if let Some(v) = self.info.as_ref() {
            struct_ser.serialize_field("info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternInstance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Info,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternInstance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ExternInstance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExternInstance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExternInstance {
                    preamble: preamble__,
                    info: info__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ExternInstance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyValuePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.KeyValuePair", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.KeyValuePair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(KeyValuePair {
                    key: key__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.KeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyValuePairList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kv_pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.KeyValuePairList", len)?;
        if !self.kv_pairs.is_empty() {
            struct_ser.serialize_field("kvPairs", &self.kv_pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyValuePairList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kv_pairs",
            "kvPairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KvPairs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kvPairs" | "kv_pairs" => Ok(GeneratedField::KvPairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyValuePairList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.KeyValuePairList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeyValuePairList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kv_pairs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KvPairs => {
                            if kv_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kvPairs"));
                            }
                            kv_pairs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(KeyValuePairList {
                    kv_pairs: kv_pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.KeyValuePairList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MatchField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if self.bitwidth != 0 {
            len += 1;
        }
        if self.doc.is_some() {
            len += 1;
        }
        if self.type_name.is_some() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        if self.r#match.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.MatchField", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if self.bitwidth != 0 {
            struct_ser.serialize_field("bitwidth", &self.bitwidth)?;
        }
        if let Some(v) = self.doc.as_ref() {
            struct_ser.serialize_field("doc", v)?;
        }
        if let Some(v) = self.type_name.as_ref() {
            struct_ser.serialize_field("typeName", v)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        if let Some(v) = self.r#match.as_ref() {
            match v {
                match_field::Match::MatchType(v) => {
                    let v = match_field::MatchType::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("matchType", &v)?;
                }
                match_field::Match::OtherMatchType(v) => {
                    struct_ser.serialize_field("otherMatchType", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MatchField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "bitwidth",
            "doc",
            "type_name",
            "typeName",
            "structured_annotations",
            "structuredAnnotations",
            "match_type",
            "matchType",
            "other_match_type",
            "otherMatchType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Annotations,
            AnnotationLocations,
            Bitwidth,
            Doc,
            TypeName,
            StructuredAnnotations,
            MatchType,
            OtherMatchType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "bitwidth" => Ok(GeneratedField::Bitwidth),
                            "doc" => Ok(GeneratedField::Doc),
                            "typeName" | "type_name" => Ok(GeneratedField::TypeName),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            "matchType" | "match_type" => Ok(GeneratedField::MatchType),
                            "otherMatchType" | "other_match_type" => Ok(GeneratedField::OtherMatchType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MatchField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.MatchField")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MatchField, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut bitwidth__ = None;
                let mut doc__ = None;
                let mut type_name__ = None;
                let mut structured_annotations__ = None;
                let mut r#match__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bitwidth => {
                            if bitwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitwidth"));
                            }
                            bitwidth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Doc => {
                            if doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            doc__ = map_.next_value()?;
                        }
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = map_.next_value()?;
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MatchType => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchType"));
                            }
                            r#match__ = map_.next_value::<::std::option::Option<match_field::MatchType>>()?.map(|x| match_field::Match::MatchType(x as i32));
                        }
                        GeneratedField::OtherMatchType => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherMatchType"));
                            }
                            r#match__ = map_.next_value::<::std::option::Option<_>>()?.map(match_field::Match::OtherMatchType);
                        }
                    }
                }
                Ok(MatchField {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    bitwidth: bitwidth__.unwrap_or_default(),
                    doc: doc__,
                    type_name: type_name__,
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                    r#match: r#match__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.MatchField", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for match_field::MatchType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Exact => "EXACT",
            Self::Lpm => "LPM",
            Self::Ternary => "TERNARY",
            Self::Range => "RANGE",
            Self::Optional => "OPTIONAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for match_field::MatchType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "EXACT",
            "LPM",
            "TERNARY",
            "RANGE",
            "OPTIONAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = match_field::MatchType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(match_field::MatchType::Unspecified),
                    "EXACT" => Ok(match_field::MatchType::Exact),
                    "LPM" => Ok(match_field::MatchType::Lpm),
                    "TERNARY" => Ok(match_field::MatchType::Ternary),
                    "RANGE" => Ok(match_field::MatchType::Range),
                    "OPTIONAL" => Ok(match_field::MatchType::Optional),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Meter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if self.index_type_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Meter", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        if let Some(v) = self.index_type_name.as_ref() {
            struct_ser.serialize_field("indexTypeName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Meter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "spec",
            "size",
            "index_type_name",
            "indexTypeName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Spec,
            Size,
            IndexTypeName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "spec" => Ok(GeneratedField::Spec),
                            "size" => Ok(GeneratedField::Size),
                            "indexTypeName" | "index_type_name" => Ok(GeneratedField::IndexTypeName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Meter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Meter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Meter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut spec__ = None;
                let mut size__ = None;
                let mut index_type_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IndexTypeName => {
                            if index_type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexTypeName"));
                            }
                            index_type_name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Meter {
                    preamble: preamble__,
                    spec: spec__,
                    size: size__.unwrap_or_default(),
                    index_type_name: index_type_name__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Meter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.MeterSpec", len)?;
        if self.unit != 0 {
            let v = meter_spec::Unit::try_from(self.unit)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.unit)))?;
            struct_ser.serialize_field("unit", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Unit,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.MeterSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MeterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value::<meter_spec::Unit>()? as i32);
                        }
                    }
                }
                Ok(MeterSpec {
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.MeterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for meter_spec::Unit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Bytes => "BYTES",
            Self::Packets => "PACKETS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for meter_spec::Unit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "BYTES",
            "PACKETS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = meter_spec::Unit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(meter_spec::Unit::Unspecified),
                    "BYTES" => Ok(meter_spec::Unit::Bytes),
                    "PACKETS" => Ok(meter_spec::Unit::Packets),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for P4BitTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bitwidth != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4BitTypeSpec", len)?;
        if self.bitwidth != 0 {
            struct_ser.serialize_field("bitwidth", &self.bitwidth)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4BitTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bitwidth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bitwidth,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bitwidth" => Ok(GeneratedField::Bitwidth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4BitTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4BitTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4BitTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bitwidth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bitwidth => {
                            if bitwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitwidth"));
                            }
                            bitwidth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(P4BitTypeSpec {
                    bitwidth: bitwidth__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4BitTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4BitstringLikeTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        if self.type_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4BitstringLikeTypeSpec", len)?;
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        if let Some(v) = self.type_spec.as_ref() {
            match v {
                p4_bitstring_like_type_spec::TypeSpec::Bit(v) => {
                    struct_ser.serialize_field("bit", v)?;
                }
                p4_bitstring_like_type_spec::TypeSpec::Int(v) => {
                    struct_ser.serialize_field("int", v)?;
                }
                p4_bitstring_like_type_spec::TypeSpec::Varbit(v) => {
                    struct_ser.serialize_field("varbit", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4BitstringLikeTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
            "bit",
            "int",
            "varbit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
            Bit,
            Int,
            Varbit,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            "bit" => Ok(GeneratedField::Bit),
                            "int" => Ok(GeneratedField::Int),
                            "varbit" => Ok(GeneratedField::Varbit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4BitstringLikeTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4BitstringLikeTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4BitstringLikeTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                let mut type_spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bit => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bit"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_bitstring_like_type_spec::TypeSpec::Bit)
;
                        }
                        GeneratedField::Int => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_bitstring_like_type_spec::TypeSpec::Int)
;
                        }
                        GeneratedField::Varbit => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("varbit"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_bitstring_like_type_spec::TypeSpec::Varbit)
;
                        }
                    }
                }
                Ok(P4BitstringLikeTypeSpec {
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                    type_spec: type_spec__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4BitstringLikeTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4BoolType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.config.v1.P4BoolType", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4BoolType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4BoolType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4BoolType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4BoolType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(P4BoolType {
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4BoolType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4DataTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.type_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4DataTypeSpec", len)?;
        if let Some(v) = self.type_spec.as_ref() {
            match v {
                p4_data_type_spec::TypeSpec::Bitstring(v) => {
                    struct_ser.serialize_field("bitstring", v)?;
                }
                p4_data_type_spec::TypeSpec::Bool(v) => {
                    struct_ser.serialize_field("bool", v)?;
                }
                p4_data_type_spec::TypeSpec::Tuple(v) => {
                    struct_ser.serialize_field("tuple", v)?;
                }
                p4_data_type_spec::TypeSpec::Struct(v) => {
                    struct_ser.serialize_field("struct", v)?;
                }
                p4_data_type_spec::TypeSpec::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                p4_data_type_spec::TypeSpec::HeaderUnion(v) => {
                    struct_ser.serialize_field("headerUnion", v)?;
                }
                p4_data_type_spec::TypeSpec::HeaderStack(v) => {
                    struct_ser.serialize_field("headerStack", v)?;
                }
                p4_data_type_spec::TypeSpec::HeaderUnionStack(v) => {
                    struct_ser.serialize_field("headerUnionStack", v)?;
                }
                p4_data_type_spec::TypeSpec::Enum(v) => {
                    struct_ser.serialize_field("enum", v)?;
                }
                p4_data_type_spec::TypeSpec::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                p4_data_type_spec::TypeSpec::SerializableEnum(v) => {
                    struct_ser.serialize_field("serializableEnum", v)?;
                }
                p4_data_type_spec::TypeSpec::NewType(v) => {
                    struct_ser.serialize_field("newType", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4DataTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bitstring",
            "bool",
            "tuple",
            "struct",
            "header",
            "header_union",
            "headerUnion",
            "header_stack",
            "headerStack",
            "header_union_stack",
            "headerUnionStack",
            "enum",
            "error",
            "serializable_enum",
            "serializableEnum",
            "new_type",
            "newType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bitstring,
            Bool,
            Tuple,
            Struct,
            Header,
            HeaderUnion,
            HeaderStack,
            HeaderUnionStack,
            Enum,
            Error,
            SerializableEnum,
            NewType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bitstring" => Ok(GeneratedField::Bitstring),
                            "bool" => Ok(GeneratedField::Bool),
                            "tuple" => Ok(GeneratedField::Tuple),
                            "struct" => Ok(GeneratedField::Struct),
                            "header" => Ok(GeneratedField::Header),
                            "headerUnion" | "header_union" => Ok(GeneratedField::HeaderUnion),
                            "headerStack" | "header_stack" => Ok(GeneratedField::HeaderStack),
                            "headerUnionStack" | "header_union_stack" => Ok(GeneratedField::HeaderUnionStack),
                            "enum" => Ok(GeneratedField::Enum),
                            "error" => Ok(GeneratedField::Error),
                            "serializableEnum" | "serializable_enum" => Ok(GeneratedField::SerializableEnum),
                            "newType" | "new_type" => Ok(GeneratedField::NewType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4DataTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4DataTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4DataTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bitstring => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitstring"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::Bitstring)
;
                        }
                        GeneratedField::Bool => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bool"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::Bool)
;
                        }
                        GeneratedField::Tuple => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::Tuple)
;
                        }
                        GeneratedField::Struct => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("struct"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::Struct)
;
                        }
                        GeneratedField::Header => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::Header)
;
                        }
                        GeneratedField::HeaderUnion => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerUnion"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::HeaderUnion)
;
                        }
                        GeneratedField::HeaderStack => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerStack"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::HeaderStack)
;
                        }
                        GeneratedField::HeaderUnionStack => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerUnionStack"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::HeaderUnionStack)
;
                        }
                        GeneratedField::Enum => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enum"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::Enum)
;
                        }
                        GeneratedField::Error => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::Error)
;
                        }
                        GeneratedField::SerializableEnum => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serializableEnum"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::SerializableEnum)
;
                        }
                        GeneratedField::NewType => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newType"));
                            }
                            type_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data_type_spec::TypeSpec::NewType)
;
                        }
                    }
                }
                Ok(P4DataTypeSpec {
                    type_spec: type_spec__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4DataTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4EnumTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4EnumTypeSpec", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4EnumTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4EnumTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4EnumTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4EnumTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4EnumTypeSpec {
                    members: members__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4EnumTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for p4_enum_type_spec::Member {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4EnumTypeSpec.Member", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for p4_enum_type_spec::Member {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = p4_enum_type_spec::Member;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4EnumTypeSpec.Member")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<p4_enum_type_spec::Member, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(p4_enum_type_spec::Member {
                    name: name__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4EnumTypeSpec.Member", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4ErrorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.config.v1.P4ErrorType", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4ErrorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4ErrorType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4ErrorType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4ErrorType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(P4ErrorType {
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4ErrorType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4ErrorTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4ErrorTypeSpec", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4ErrorTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4ErrorTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4ErrorTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4ErrorTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4ErrorTypeSpec {
                    members: members__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4ErrorTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4HeaderStackTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4HeaderStackTypeSpec", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if self.size != 0 {
            struct_ser.serialize_field("size", &self.size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4HeaderStackTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Size,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "header" => Ok(GeneratedField::Header),
                            "size" => Ok(GeneratedField::Size),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4HeaderStackTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4HeaderStackTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4HeaderStackTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(P4HeaderStackTypeSpec {
                    header: header__,
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4HeaderStackTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4HeaderTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4HeaderTypeSpec", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4HeaderTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4HeaderTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4HeaderTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4HeaderTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4HeaderTypeSpec {
                    members: members__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4HeaderTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for p4_header_type_spec::Member {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.type_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4HeaderTypeSpec.Member", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.type_spec.as_ref() {
            struct_ser.serialize_field("typeSpec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for p4_header_type_spec::Member {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type_spec",
            "typeSpec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypeSpec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "typeSpec" | "type_spec" => Ok(GeneratedField::TypeSpec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = p4_header_type_spec::Member;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4HeaderTypeSpec.Member")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<p4_header_type_spec::Member, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut type_spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeSpec => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeSpec"));
                            }
                            type_spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(p4_header_type_spec::Member {
                    name: name__.unwrap_or_default(),
                    type_spec: type_spec__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4HeaderTypeSpec.Member", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4HeaderUnionStackTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header_union.is_some() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4HeaderUnionStackTypeSpec", len)?;
        if let Some(v) = self.header_union.as_ref() {
            struct_ser.serialize_field("headerUnion", v)?;
        }
        if self.size != 0 {
            struct_ser.serialize_field("size", &self.size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4HeaderUnionStackTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_union",
            "headerUnion",
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderUnion,
            Size,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "headerUnion" | "header_union" => Ok(GeneratedField::HeaderUnion),
                            "size" => Ok(GeneratedField::Size),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4HeaderUnionStackTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4HeaderUnionStackTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4HeaderUnionStackTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_union__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderUnion => {
                            if header_union__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerUnion"));
                            }
                            header_union__ = map_.next_value()?;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(P4HeaderUnionStackTypeSpec {
                    header_union: header_union__,
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4HeaderUnionStackTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4HeaderUnionTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4HeaderUnionTypeSpec", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4HeaderUnionTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4HeaderUnionTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4HeaderUnionTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4HeaderUnionTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4HeaderUnionTypeSpec {
                    members: members__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4HeaderUnionTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for p4_header_union_type_spec::Member {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4HeaderUnionTypeSpec.Member", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for p4_header_union_type_spec::Member {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Header,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "header" => Ok(GeneratedField::Header),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = p4_header_union_type_spec::Member;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4HeaderUnionTypeSpec.Member")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<p4_header_union_type_spec::Member, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut header__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                    }
                }
                Ok(p4_header_union_type_spec::Member {
                    name: name__.unwrap_or_default(),
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4HeaderUnionTypeSpec.Member", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4Ids {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.config.v1.P4Ids", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4Ids {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4Ids;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4Ids")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4Ids, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(P4Ids {
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4Ids", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for p4_ids::Prefix {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Action => "ACTION",
            Self::Table => "TABLE",
            Self::ValueSet => "VALUE_SET",
            Self::ControllerHeader => "CONTROLLER_HEADER",
            Self::PsaExternsStart => "PSA_EXTERNS_START",
            Self::ActionProfile => "ACTION_PROFILE",
            Self::Counter => "COUNTER",
            Self::DirectCounter => "DIRECT_COUNTER",
            Self::Meter => "METER",
            Self::DirectMeter => "DIRECT_METER",
            Self::Register => "REGISTER",
            Self::Digest => "DIGEST",
            Self::OtherExternsStart => "OTHER_EXTERNS_START",
            Self::Max => "MAX",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for p4_ids::Prefix {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "ACTION",
            "TABLE",
            "VALUE_SET",
            "CONTROLLER_HEADER",
            "PSA_EXTERNS_START",
            "ACTION_PROFILE",
            "COUNTER",
            "DIRECT_COUNTER",
            "METER",
            "DIRECT_METER",
            "REGISTER",
            "DIGEST",
            "OTHER_EXTERNS_START",
            "MAX",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = p4_ids::Prefix;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(p4_ids::Prefix::Unspecified),
                    "ACTION" => Ok(p4_ids::Prefix::Action),
                    "TABLE" => Ok(p4_ids::Prefix::Table),
                    "VALUE_SET" => Ok(p4_ids::Prefix::ValueSet),
                    "CONTROLLER_HEADER" => Ok(p4_ids::Prefix::ControllerHeader),
                    "PSA_EXTERNS_START" => Ok(p4_ids::Prefix::PsaExternsStart),
                    "ACTION_PROFILE" => Ok(p4_ids::Prefix::ActionProfile),
                    "COUNTER" => Ok(p4_ids::Prefix::Counter),
                    "DIRECT_COUNTER" => Ok(p4_ids::Prefix::DirectCounter),
                    "METER" => Ok(p4_ids::Prefix::Meter),
                    "DIRECT_METER" => Ok(p4_ids::Prefix::DirectMeter),
                    "REGISTER" => Ok(p4_ids::Prefix::Register),
                    "DIGEST" => Ok(p4_ids::Prefix::Digest),
                    "OTHER_EXTERNS_START" => Ok(p4_ids::Prefix::OtherExternsStart),
                    "MAX" => Ok(p4_ids::Prefix::Max),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for P4Info {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pkg_info.is_some() {
            len += 1;
        }
        if !self.tables.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if !self.action_profiles.is_empty() {
            len += 1;
        }
        if !self.counters.is_empty() {
            len += 1;
        }
        if !self.direct_counters.is_empty() {
            len += 1;
        }
        if !self.meters.is_empty() {
            len += 1;
        }
        if !self.direct_meters.is_empty() {
            len += 1;
        }
        if !self.controller_packet_metadata.is_empty() {
            len += 1;
        }
        if !self.value_sets.is_empty() {
            len += 1;
        }
        if !self.registers.is_empty() {
            len += 1;
        }
        if !self.digests.is_empty() {
            len += 1;
        }
        if !self.externs.is_empty() {
            len += 1;
        }
        if self.type_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4Info", len)?;
        if let Some(v) = self.pkg_info.as_ref() {
            struct_ser.serialize_field("pkgInfo", v)?;
        }
        if !self.tables.is_empty() {
            struct_ser.serialize_field("tables", &self.tables)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if !self.action_profiles.is_empty() {
            struct_ser.serialize_field("actionProfiles", &self.action_profiles)?;
        }
        if !self.counters.is_empty() {
            struct_ser.serialize_field("counters", &self.counters)?;
        }
        if !self.direct_counters.is_empty() {
            struct_ser.serialize_field("directCounters", &self.direct_counters)?;
        }
        if !self.meters.is_empty() {
            struct_ser.serialize_field("meters", &self.meters)?;
        }
        if !self.direct_meters.is_empty() {
            struct_ser.serialize_field("directMeters", &self.direct_meters)?;
        }
        if !self.controller_packet_metadata.is_empty() {
            struct_ser.serialize_field("controllerPacketMetadata", &self.controller_packet_metadata)?;
        }
        if !self.value_sets.is_empty() {
            struct_ser.serialize_field("valueSets", &self.value_sets)?;
        }
        if !self.registers.is_empty() {
            struct_ser.serialize_field("registers", &self.registers)?;
        }
        if !self.digests.is_empty() {
            struct_ser.serialize_field("digests", &self.digests)?;
        }
        if !self.externs.is_empty() {
            struct_ser.serialize_field("externs", &self.externs)?;
        }
        if let Some(v) = self.type_info.as_ref() {
            struct_ser.serialize_field("typeInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4Info {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pkg_info",
            "pkgInfo",
            "tables",
            "actions",
            "action_profiles",
            "actionProfiles",
            "counters",
            "direct_counters",
            "directCounters",
            "meters",
            "direct_meters",
            "directMeters",
            "controller_packet_metadata",
            "controllerPacketMetadata",
            "value_sets",
            "valueSets",
            "registers",
            "digests",
            "externs",
            "type_info",
            "typeInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PkgInfo,
            Tables,
            Actions,
            ActionProfiles,
            Counters,
            DirectCounters,
            Meters,
            DirectMeters,
            ControllerPacketMetadata,
            ValueSets,
            Registers,
            Digests,
            Externs,
            TypeInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pkgInfo" | "pkg_info" => Ok(GeneratedField::PkgInfo),
                            "tables" => Ok(GeneratedField::Tables),
                            "actions" => Ok(GeneratedField::Actions),
                            "actionProfiles" | "action_profiles" => Ok(GeneratedField::ActionProfiles),
                            "counters" => Ok(GeneratedField::Counters),
                            "directCounters" | "direct_counters" => Ok(GeneratedField::DirectCounters),
                            "meters" => Ok(GeneratedField::Meters),
                            "directMeters" | "direct_meters" => Ok(GeneratedField::DirectMeters),
                            "controllerPacketMetadata" | "controller_packet_metadata" => Ok(GeneratedField::ControllerPacketMetadata),
                            "valueSets" | "value_sets" => Ok(GeneratedField::ValueSets),
                            "registers" => Ok(GeneratedField::Registers),
                            "digests" => Ok(GeneratedField::Digests),
                            "externs" => Ok(GeneratedField::Externs),
                            "typeInfo" | "type_info" => Ok(GeneratedField::TypeInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4Info;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4Info")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4Info, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pkg_info__ = None;
                let mut tables__ = None;
                let mut actions__ = None;
                let mut action_profiles__ = None;
                let mut counters__ = None;
                let mut direct_counters__ = None;
                let mut meters__ = None;
                let mut direct_meters__ = None;
                let mut controller_packet_metadata__ = None;
                let mut value_sets__ = None;
                let mut registers__ = None;
                let mut digests__ = None;
                let mut externs__ = None;
                let mut type_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PkgInfo => {
                            if pkg_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkgInfo"));
                            }
                            pkg_info__ = map_.next_value()?;
                        }
                        GeneratedField::Tables => {
                            if tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tables"));
                            }
                            tables__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionProfiles => {
                            if action_profiles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfiles"));
                            }
                            action_profiles__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Counters => {
                            if counters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counters"));
                            }
                            counters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DirectCounters => {
                            if direct_counters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directCounters"));
                            }
                            direct_counters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Meters => {
                            if meters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meters"));
                            }
                            meters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DirectMeters => {
                            if direct_meters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directMeters"));
                            }
                            direct_meters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ControllerPacketMetadata => {
                            if controller_packet_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controllerPacketMetadata"));
                            }
                            controller_packet_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueSets => {
                            if value_sets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSets"));
                            }
                            value_sets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Registers => {
                            if registers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registers"));
                            }
                            registers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Digests => {
                            if digests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digests"));
                            }
                            digests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Externs => {
                            if externs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externs"));
                            }
                            externs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeInfo => {
                            if type_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeInfo"));
                            }
                            type_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(P4Info {
                    pkg_info: pkg_info__,
                    tables: tables__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    action_profiles: action_profiles__.unwrap_or_default(),
                    counters: counters__.unwrap_or_default(),
                    direct_counters: direct_counters__.unwrap_or_default(),
                    meters: meters__.unwrap_or_default(),
                    direct_meters: direct_meters__.unwrap_or_default(),
                    controller_packet_metadata: controller_packet_metadata__.unwrap_or_default(),
                    value_sets: value_sets__.unwrap_or_default(),
                    registers: registers__.unwrap_or_default(),
                    digests: digests__.unwrap_or_default(),
                    externs: externs__.unwrap_or_default(),
                    type_info: type_info__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4Info", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4IntTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bitwidth != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4IntTypeSpec", len)?;
        if self.bitwidth != 0 {
            struct_ser.serialize_field("bitwidth", &self.bitwidth)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4IntTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bitwidth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bitwidth,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bitwidth" => Ok(GeneratedField::Bitwidth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4IntTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4IntTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4IntTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bitwidth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bitwidth => {
                            if bitwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitwidth"));
                            }
                            bitwidth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(P4IntTypeSpec {
                    bitwidth: bitwidth__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4IntTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4NamedType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4NamedType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4NamedType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4NamedType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4NamedType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4NamedType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4NamedType {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4NamedType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4NewTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        if self.representation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4NewTypeSpec", len)?;
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        if let Some(v) = self.representation.as_ref() {
            match v {
                p4_new_type_spec::Representation::OriginalType(v) => {
                    struct_ser.serialize_field("originalType", v)?;
                }
                p4_new_type_spec::Representation::TranslatedType(v) => {
                    struct_ser.serialize_field("translatedType", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4NewTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
            "original_type",
            "originalType",
            "translated_type",
            "translatedType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
            OriginalType,
            TranslatedType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            "originalType" | "original_type" => Ok(GeneratedField::OriginalType),
                            "translatedType" | "translated_type" => Ok(GeneratedField::TranslatedType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4NewTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4NewTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4NewTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                let mut representation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OriginalType => {
                            if representation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalType"));
                            }
                            representation__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_new_type_spec::Representation::OriginalType)
;
                        }
                        GeneratedField::TranslatedType => {
                            if representation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translatedType"));
                            }
                            representation__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_new_type_spec::Representation::TranslatedType)
;
                        }
                    }
                }
                Ok(P4NewTypeSpec {
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                    representation: representation__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4NewTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4NewTypeTranslation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        if self.sdn_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4NewTypeTranslation", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        if let Some(v) = self.sdn_type.as_ref() {
            match v {
                p4_new_type_translation::SdnType::SdnBitwidth(v) => {
                    struct_ser.serialize_field("sdnBitwidth", v)?;
                }
                p4_new_type_translation::SdnType::SdnString(v) => {
                    struct_ser.serialize_field("sdnString", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4NewTypeTranslation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
            "sdn_bitwidth",
            "sdnBitwidth",
            "sdn_string",
            "sdnString",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
            SdnBitwidth,
            SdnString,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uri" => Ok(GeneratedField::Uri),
                            "sdnBitwidth" | "sdn_bitwidth" => Ok(GeneratedField::SdnBitwidth),
                            "sdnString" | "sdn_string" => Ok(GeneratedField::SdnString),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4NewTypeTranslation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4NewTypeTranslation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4NewTypeTranslation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                let mut sdn_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SdnBitwidth => {
                            if sdn_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sdnBitwidth"));
                            }
                            sdn_type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| p4_new_type_translation::SdnType::SdnBitwidth(x.0));
                        }
                        GeneratedField::SdnString => {
                            if sdn_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sdnString"));
                            }
                            sdn_type__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_new_type_translation::SdnType::SdnString)
;
                        }
                    }
                }
                Ok(P4NewTypeTranslation {
                    uri: uri__.unwrap_or_default(),
                    sdn_type: sdn_type__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4NewTypeTranslation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for p4_new_type_translation::SdnString {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.config.v1.P4NewTypeTranslation.SdnString", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for p4_new_type_translation::SdnString {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = p4_new_type_translation::SdnString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4NewTypeTranslation.SdnString")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<p4_new_type_translation::SdnString, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(p4_new_type_translation::SdnString {
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4NewTypeTranslation.SdnString", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4SerializableEnumTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.underlying_type.is_some() {
            len += 1;
        }
        if !self.members.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4SerializableEnumTypeSpec", len)?;
        if let Some(v) = self.underlying_type.as_ref() {
            struct_ser.serialize_field("underlyingType", v)?;
        }
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4SerializableEnumTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "underlying_type",
            "underlyingType",
            "members",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnderlyingType,
            Members,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "underlyingType" | "underlying_type" => Ok(GeneratedField::UnderlyingType),
                            "members" => Ok(GeneratedField::Members),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4SerializableEnumTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4SerializableEnumTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4SerializableEnumTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut underlying_type__ = None;
                let mut members__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UnderlyingType => {
                            if underlying_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("underlyingType"));
                            }
                            underlying_type__ = map_.next_value()?;
                        }
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4SerializableEnumTypeSpec {
                    underlying_type: underlying_type__,
                    members: members__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4SerializableEnumTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for p4_serializable_enum_type_spec::Member {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4SerializableEnumTypeSpec.Member", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for p4_serializable_enum_type_spec::Member {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "value" => Ok(GeneratedField::Value),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = p4_serializable_enum_type_spec::Member;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4SerializableEnumTypeSpec.Member")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<p4_serializable_enum_type_spec::Member, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(p4_serializable_enum_type_spec::Member {
                    name: name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4SerializableEnumTypeSpec.Member", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4StructTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4StructTypeSpec", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4StructTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
            Annotations,
            AnnotationLocations,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4StructTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4StructTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4StructTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4StructTypeSpec {
                    members: members__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4StructTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for p4_struct_type_spec::Member {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.type_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4StructTypeSpec.Member", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.type_spec.as_ref() {
            struct_ser.serialize_field("typeSpec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for p4_struct_type_spec::Member {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type_spec",
            "typeSpec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypeSpec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "typeSpec" | "type_spec" => Ok(GeneratedField::TypeSpec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = p4_struct_type_spec::Member;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4StructTypeSpec.Member")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<p4_struct_type_spec::Member, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut type_spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeSpec => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeSpec"));
                            }
                            type_spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(p4_struct_type_spec::Member {
                    name: name__.unwrap_or_default(),
                    type_spec: type_spec__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4StructTypeSpec.Member", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4TupleTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4TupleTypeSpec", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4TupleTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4TupleTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4TupleTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4TupleTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4TupleTypeSpec {
                    members: members__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4TupleTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4TypeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.structs.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.header_unions.is_empty() {
            len += 1;
        }
        if !self.enums.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if !self.serializable_enums.is_empty() {
            len += 1;
        }
        if !self.new_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4TypeInfo", len)?;
        if !self.structs.is_empty() {
            struct_ser.serialize_field("structs", &self.structs)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.header_unions.is_empty() {
            struct_ser.serialize_field("headerUnions", &self.header_unions)?;
        }
        if !self.enums.is_empty() {
            struct_ser.serialize_field("enums", &self.enums)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.serializable_enums.is_empty() {
            struct_ser.serialize_field("serializableEnums", &self.serializable_enums)?;
        }
        if !self.new_types.is_empty() {
            struct_ser.serialize_field("newTypes", &self.new_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4TypeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "structs",
            "headers",
            "header_unions",
            "headerUnions",
            "enums",
            "error",
            "serializable_enums",
            "serializableEnums",
            "new_types",
            "newTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Structs,
            Headers,
            HeaderUnions,
            Enums,
            Error,
            SerializableEnums,
            NewTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "structs" => Ok(GeneratedField::Structs),
                            "headers" => Ok(GeneratedField::Headers),
                            "headerUnions" | "header_unions" => Ok(GeneratedField::HeaderUnions),
                            "enums" => Ok(GeneratedField::Enums),
                            "error" => Ok(GeneratedField::Error),
                            "serializableEnums" | "serializable_enums" => Ok(GeneratedField::SerializableEnums),
                            "newTypes" | "new_types" => Ok(GeneratedField::NewTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4TypeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4TypeInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4TypeInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut structs__ = None;
                let mut headers__ = None;
                let mut header_unions__ = None;
                let mut enums__ = None;
                let mut error__ = None;
                let mut serializable_enums__ = None;
                let mut new_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Structs => {
                            if structs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structs"));
                            }
                            structs__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::HeaderUnions => {
                            if header_unions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerUnions"));
                            }
                            header_unions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Enums => {
                            if enums__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enums"));
                            }
                            enums__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::SerializableEnums => {
                            if serializable_enums__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serializableEnums"));
                            }
                            serializable_enums__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::NewTypes => {
                            if new_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newTypes"));
                            }
                            new_types__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(P4TypeInfo {
                    structs: structs__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    header_unions: header_unions__.unwrap_or_default(),
                    enums: enums__.unwrap_or_default(),
                    error: error__,
                    serializable_enums: serializable_enums__.unwrap_or_default(),
                    new_types: new_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4TypeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4VarbitTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_bitwidth != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.P4VarbitTypeSpec", len)?;
        if self.max_bitwidth != 0 {
            struct_ser.serialize_field("maxBitwidth", &self.max_bitwidth)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4VarbitTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_bitwidth",
            "maxBitwidth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxBitwidth,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxBitwidth" | "max_bitwidth" => Ok(GeneratedField::MaxBitwidth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4VarbitTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.P4VarbitTypeSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4VarbitTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_bitwidth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxBitwidth => {
                            if max_bitwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBitwidth"));
                            }
                            max_bitwidth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(P4VarbitTypeSpec {
                    max_bitwidth: max_bitwidth__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.P4VarbitTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PkgInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.doc.is_some() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if !self.arch.is_empty() {
            len += 1;
        }
        if !self.organization.is_empty() {
            len += 1;
        }
        if !self.contact.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        if self.platform_properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.PkgInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.doc.as_ref() {
            struct_ser.serialize_field("doc", v)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if !self.arch.is_empty() {
            struct_ser.serialize_field("arch", &self.arch)?;
        }
        if !self.organization.is_empty() {
            struct_ser.serialize_field("organization", &self.organization)?;
        }
        if !self.contact.is_empty() {
            struct_ser.serialize_field("contact", &self.contact)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        if let Some(v) = self.platform_properties.as_ref() {
            struct_ser.serialize_field("platformProperties", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PkgInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "doc",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "arch",
            "organization",
            "contact",
            "url",
            "structured_annotations",
            "structuredAnnotations",
            "platform_properties",
            "platformProperties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Doc,
            Annotations,
            AnnotationLocations,
            Arch,
            Organization,
            Contact,
            Url,
            StructuredAnnotations,
            PlatformProperties,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "version" => Ok(GeneratedField::Version),
                            "doc" => Ok(GeneratedField::Doc),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "arch" => Ok(GeneratedField::Arch),
                            "organization" => Ok(GeneratedField::Organization),
                            "contact" => Ok(GeneratedField::Contact),
                            "url" => Ok(GeneratedField::Url),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            "platformProperties" | "platform_properties" => Ok(GeneratedField::PlatformProperties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PkgInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.PkgInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PkgInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut doc__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut arch__ = None;
                let mut organization__ = None;
                let mut contact__ = None;
                let mut url__ = None;
                let mut structured_annotations__ = None;
                let mut platform_properties__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Doc => {
                            if doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            doc__ = map_.next_value()?;
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Arch => {
                            if arch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arch"));
                            }
                            arch__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contact => {
                            if contact__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            contact__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlatformProperties => {
                            if platform_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformProperties"));
                            }
                            platform_properties__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PkgInfo {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    doc: doc__,
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    arch: arch__.unwrap_or_default(),
                    organization: organization__.unwrap_or_default(),
                    contact: contact__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                    platform_properties: platform_properties__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.PkgInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlatformProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.multicast_group_table_size != 0 {
            len += 1;
        }
        if self.multicast_group_table_total_replicas != 0 {
            len += 1;
        }
        if self.multicast_group_table_max_replicas_per_entry != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.PlatformProperties", len)?;
        if self.multicast_group_table_size != 0 {
            struct_ser.serialize_field("multicastGroupTableSize", &self.multicast_group_table_size)?;
        }
        if self.multicast_group_table_total_replicas != 0 {
            struct_ser.serialize_field("multicastGroupTableTotalReplicas", &self.multicast_group_table_total_replicas)?;
        }
        if self.multicast_group_table_max_replicas_per_entry != 0 {
            struct_ser.serialize_field("multicastGroupTableMaxReplicasPerEntry", &self.multicast_group_table_max_replicas_per_entry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlatformProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "multicast_group_table_size",
            "multicastGroupTableSize",
            "multicast_group_table_total_replicas",
            "multicastGroupTableTotalReplicas",
            "multicast_group_table_max_replicas_per_entry",
            "multicastGroupTableMaxReplicasPerEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MulticastGroupTableSize,
            MulticastGroupTableTotalReplicas,
            MulticastGroupTableMaxReplicasPerEntry,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "multicastGroupTableSize" | "multicast_group_table_size" => Ok(GeneratedField::MulticastGroupTableSize),
                            "multicastGroupTableTotalReplicas" | "multicast_group_table_total_replicas" => Ok(GeneratedField::MulticastGroupTableTotalReplicas),
                            "multicastGroupTableMaxReplicasPerEntry" | "multicast_group_table_max_replicas_per_entry" => Ok(GeneratedField::MulticastGroupTableMaxReplicasPerEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlatformProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.PlatformProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlatformProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut multicast_group_table_size__ = None;
                let mut multicast_group_table_total_replicas__ = None;
                let mut multicast_group_table_max_replicas_per_entry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MulticastGroupTableSize => {
                            if multicast_group_table_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multicastGroupTableSize"));
                            }
                            multicast_group_table_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MulticastGroupTableTotalReplicas => {
                            if multicast_group_table_total_replicas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multicastGroupTableTotalReplicas"));
                            }
                            multicast_group_table_total_replicas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MulticastGroupTableMaxReplicasPerEntry => {
                            if multicast_group_table_max_replicas_per_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multicastGroupTableMaxReplicasPerEntry"));
                            }
                            multicast_group_table_max_replicas_per_entry__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PlatformProperties {
                    multicast_group_table_size: multicast_group_table_size__.unwrap_or_default(),
                    multicast_group_table_total_replicas: multicast_group_table_total_replicas__.unwrap_or_default(),
                    multicast_group_table_max_replicas_per_entry: multicast_group_table_max_replicas_per_entry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.PlatformProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Preamble {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.alias.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.annotation_locations.is_empty() {
            len += 1;
        }
        if self.doc.is_some() {
            len += 1;
        }
        if !self.structured_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Preamble", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.alias.is_empty() {
            struct_ser.serialize_field("alias", &self.alias)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.annotation_locations.is_empty() {
            struct_ser.serialize_field("annotationLocations", &self.annotation_locations)?;
        }
        if let Some(v) = self.doc.as_ref() {
            struct_ser.serialize_field("doc", v)?;
        }
        if !self.structured_annotations.is_empty() {
            struct_ser.serialize_field("structuredAnnotations", &self.structured_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Preamble {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "alias",
            "annotations",
            "annotation_locations",
            "annotationLocations",
            "doc",
            "structured_annotations",
            "structuredAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Alias,
            Annotations,
            AnnotationLocations,
            Doc,
            StructuredAnnotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "alias" => Ok(GeneratedField::Alias),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "annotationLocations" | "annotation_locations" => Ok(GeneratedField::AnnotationLocations),
                            "doc" => Ok(GeneratedField::Doc),
                            "structuredAnnotations" | "structured_annotations" => Ok(GeneratedField::StructuredAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Preamble;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Preamble")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Preamble, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut alias__ = None;
                let mut annotations__ = None;
                let mut annotation_locations__ = None;
                let mut doc__ = None;
                let mut structured_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Alias => {
                            if alias__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alias"));
                            }
                            alias__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLocations => {
                            if annotation_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLocations"));
                            }
                            annotation_locations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Doc => {
                            if doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            doc__ = map_.next_value()?;
                        }
                        GeneratedField::StructuredAnnotations => {
                            if structured_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structuredAnnotations"));
                            }
                            structured_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Preamble {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    alias: alias__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    annotation_locations: annotation_locations__.unwrap_or_default(),
                    doc: doc__,
                    structured_annotations: structured_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Preamble", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Register {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if self.type_spec.is_some() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if self.index_type_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Register", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if let Some(v) = self.type_spec.as_ref() {
            struct_ser.serialize_field("typeSpec", v)?;
        }
        if self.size != 0 {
            struct_ser.serialize_field("size", &self.size)?;
        }
        if let Some(v) = self.index_type_name.as_ref() {
            struct_ser.serialize_field("indexTypeName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Register {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "type_spec",
            "typeSpec",
            "size",
            "index_type_name",
            "indexTypeName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            TypeSpec,
            Size,
            IndexTypeName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "typeSpec" | "type_spec" => Ok(GeneratedField::TypeSpec),
                            "size" => Ok(GeneratedField::Size),
                            "indexTypeName" | "index_type_name" => Ok(GeneratedField::IndexTypeName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Register;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Register")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Register, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut type_spec__ = None;
                let mut size__ = None;
                let mut index_type_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::TypeSpec => {
                            if type_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeSpec"));
                            }
                            type_spec__ = map_.next_value()?;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IndexTypeName => {
                            if index_type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexTypeName"));
                            }
                            index_type_name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Register {
                    preamble: preamble__,
                    type_spec: type_spec__,
                    size: size__.unwrap_or_default(),
                    index_type_name: index_type_name__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Register", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceLocation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file.is_empty() {
            len += 1;
        }
        if self.line != 0 {
            len += 1;
        }
        if self.column != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.SourceLocation", len)?;
        if !self.file.is_empty() {
            struct_ser.serialize_field("file", &self.file)?;
        }
        if self.line != 0 {
            struct_ser.serialize_field("line", &self.line)?;
        }
        if self.column != 0 {
            struct_ser.serialize_field("column", &self.column)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceLocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file",
            "line",
            "column",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            File,
            Line,
            Column,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "file" => Ok(GeneratedField::File),
                            "line" => Ok(GeneratedField::Line),
                            "column" => Ok(GeneratedField::Column),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceLocation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.SourceLocation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourceLocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file__ = None;
                let mut line__ = None;
                let mut column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SourceLocation {
                    file: file__.unwrap_or_default(),
                    line: line__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.SourceLocation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StructuredAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.source_location.is_some() {
            len += 1;
        }
        if self.body.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.StructuredAnnotation", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.source_location.as_ref() {
            struct_ser.serialize_field("sourceLocation", v)?;
        }
        if let Some(v) = self.body.as_ref() {
            match v {
                structured_annotation::Body::ExpressionList(v) => {
                    struct_ser.serialize_field("expressionList", v)?;
                }
                structured_annotation::Body::KvPairList(v) => {
                    struct_ser.serialize_field("kvPairList", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StructuredAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "source_location",
            "sourceLocation",
            "expression_list",
            "expressionList",
            "kv_pair_list",
            "kvPairList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            SourceLocation,
            ExpressionList,
            KvPairList,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "sourceLocation" | "source_location" => Ok(GeneratedField::SourceLocation),
                            "expressionList" | "expression_list" => Ok(GeneratedField::ExpressionList),
                            "kvPairList" | "kv_pair_list" => Ok(GeneratedField::KvPairList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StructuredAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.StructuredAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StructuredAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut source_location__ = None;
                let mut body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceLocation => {
                            if source_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceLocation"));
                            }
                            source_location__ = map_.next_value()?;
                        }
                        GeneratedField::ExpressionList => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expressionList"));
                            }
                            body__ = map_.next_value::<::std::option::Option<_>>()?.map(structured_annotation::Body::ExpressionList)
;
                        }
                        GeneratedField::KvPairList => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kvPairList"));
                            }
                            body__ = map_.next_value::<::std::option::Option<_>>()?.map(structured_annotation::Body::KvPairList)
;
                        }
                    }
                }
                Ok(StructuredAnnotation {
                    name: name__.unwrap_or_default(),
                    source_location: source_location__,
                    body: body__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.StructuredAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Table {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if !self.match_fields.is_empty() {
            len += 1;
        }
        if !self.action_refs.is_empty() {
            len += 1;
        }
        if self.const_default_action_id != 0 {
            len += 1;
        }
        if self.implementation_id != 0 {
            len += 1;
        }
        if !self.direct_resource_ids.is_empty() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if self.idle_timeout_behavior != 0 {
            len += 1;
        }
        if self.is_const_table {
            len += 1;
        }
        if self.has_initial_entries {
            len += 1;
        }
        if self.other_properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.Table", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if !self.match_fields.is_empty() {
            struct_ser.serialize_field("matchFields", &self.match_fields)?;
        }
        if !self.action_refs.is_empty() {
            struct_ser.serialize_field("actionRefs", &self.action_refs)?;
        }
        if self.const_default_action_id != 0 {
            struct_ser.serialize_field("constDefaultActionId", &self.const_default_action_id)?;
        }
        if self.implementation_id != 0 {
            struct_ser.serialize_field("implementationId", &self.implementation_id)?;
        }
        if !self.direct_resource_ids.is_empty() {
            struct_ser.serialize_field("directResourceIds", &self.direct_resource_ids)?;
        }
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        if self.idle_timeout_behavior != 0 {
            let v = table::IdleTimeoutBehavior::try_from(self.idle_timeout_behavior)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.idle_timeout_behavior)))?;
            struct_ser.serialize_field("idleTimeoutBehavior", &v)?;
        }
        if self.is_const_table {
            struct_ser.serialize_field("isConstTable", &self.is_const_table)?;
        }
        if self.has_initial_entries {
            struct_ser.serialize_field("hasInitialEntries", &self.has_initial_entries)?;
        }
        if let Some(v) = self.other_properties.as_ref() {
            struct_ser.serialize_field("otherProperties", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Table {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "match_fields",
            "matchFields",
            "action_refs",
            "actionRefs",
            "const_default_action_id",
            "constDefaultActionId",
            "implementation_id",
            "implementationId",
            "direct_resource_ids",
            "directResourceIds",
            "size",
            "idle_timeout_behavior",
            "idleTimeoutBehavior",
            "is_const_table",
            "isConstTable",
            "has_initial_entries",
            "hasInitialEntries",
            "other_properties",
            "otherProperties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            MatchFields,
            ActionRefs,
            ConstDefaultActionId,
            ImplementationId,
            DirectResourceIds,
            Size,
            IdleTimeoutBehavior,
            IsConstTable,
            HasInitialEntries,
            OtherProperties,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "matchFields" | "match_fields" => Ok(GeneratedField::MatchFields),
                            "actionRefs" | "action_refs" => Ok(GeneratedField::ActionRefs),
                            "constDefaultActionId" | "const_default_action_id" => Ok(GeneratedField::ConstDefaultActionId),
                            "implementationId" | "implementation_id" => Ok(GeneratedField::ImplementationId),
                            "directResourceIds" | "direct_resource_ids" => Ok(GeneratedField::DirectResourceIds),
                            "size" => Ok(GeneratedField::Size),
                            "idleTimeoutBehavior" | "idle_timeout_behavior" => Ok(GeneratedField::IdleTimeoutBehavior),
                            "isConstTable" | "is_const_table" => Ok(GeneratedField::IsConstTable),
                            "hasInitialEntries" | "has_initial_entries" => Ok(GeneratedField::HasInitialEntries),
                            "otherProperties" | "other_properties" => Ok(GeneratedField::OtherProperties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.Table")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Table, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut match_fields__ = None;
                let mut action_refs__ = None;
                let mut const_default_action_id__ = None;
                let mut implementation_id__ = None;
                let mut direct_resource_ids__ = None;
                let mut size__ = None;
                let mut idle_timeout_behavior__ = None;
                let mut is_const_table__ = None;
                let mut has_initial_entries__ = None;
                let mut other_properties__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::MatchFields => {
                            if match_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchFields"));
                            }
                            match_fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionRefs => {
                            if action_refs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionRefs"));
                            }
                            action_refs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConstDefaultActionId => {
                            if const_default_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constDefaultActionId"));
                            }
                            const_default_action_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ImplementationId => {
                            if implementation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("implementationId"));
                            }
                            implementation_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DirectResourceIds => {
                            if direct_resource_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directResourceIds"));
                            }
                            direct_resource_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IdleTimeoutBehavior => {
                            if idle_timeout_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeoutBehavior"));
                            }
                            idle_timeout_behavior__ = Some(map_.next_value::<table::IdleTimeoutBehavior>()? as i32);
                        }
                        GeneratedField::IsConstTable => {
                            if is_const_table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isConstTable"));
                            }
                            is_const_table__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasInitialEntries => {
                            if has_initial_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasInitialEntries"));
                            }
                            has_initial_entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OtherProperties => {
                            if other_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherProperties"));
                            }
                            other_properties__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Table {
                    preamble: preamble__,
                    match_fields: match_fields__.unwrap_or_default(),
                    action_refs: action_refs__.unwrap_or_default(),
                    const_default_action_id: const_default_action_id__.unwrap_or_default(),
                    implementation_id: implementation_id__.unwrap_or_default(),
                    direct_resource_ids: direct_resource_ids__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                    idle_timeout_behavior: idle_timeout_behavior__.unwrap_or_default(),
                    is_const_table: is_const_table__.unwrap_or_default(),
                    has_initial_entries: has_initial_entries__.unwrap_or_default(),
                    other_properties: other_properties__,
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.Table", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for table::IdleTimeoutBehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoTimeout => "NO_TIMEOUT",
            Self::NotifyControl => "NOTIFY_CONTROL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for table::IdleTimeoutBehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NO_TIMEOUT",
            "NOTIFY_CONTROL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = table::IdleTimeoutBehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NO_TIMEOUT" => Ok(table::IdleTimeoutBehavior::NoTimeout),
                    "NOTIFY_CONTROL" => Ok(table::IdleTimeoutBehavior::NotifyControl),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ValueSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preamble.is_some() {
            len += 1;
        }
        if !self.r#match.is_empty() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.config.v1.ValueSet", len)?;
        if let Some(v) = self.preamble.as_ref() {
            struct_ser.serialize_field("preamble", v)?;
        }
        if !self.r#match.is_empty() {
            struct_ser.serialize_field("match", &self.r#match)?;
        }
        if self.size != 0 {
            struct_ser.serialize_field("size", &self.size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValueSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preamble",
            "match",
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preamble,
            Match,
            Size,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preamble" => Ok(GeneratedField::Preamble),
                            "match" => Ok(GeneratedField::Match),
                            "size" => Ok(GeneratedField::Size),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValueSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.config.v1.ValueSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValueSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preamble__ = None;
                let mut r#match__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preamble => {
                            if preamble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preamble"));
                            }
                            preamble__ = map_.next_value()?;
                        }
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ValueSet {
                    preamble: preamble__,
                    r#match: r#match__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.config.v1.ValueSet", FIELDS, GeneratedVisitor)
    }
}

// @generated
impl serde::Serialize for Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action_id != 0 {
            len += 1;
        }
        if !self.params.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Action", len)?;
        if self.action_id != 0 {
            struct_ser.serialize_field("actionId", &self.action_id)?;
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
            "action_id",
            "actionId",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActionId,
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
                            "actionId" | "action_id" => Ok(GeneratedField::ActionId),
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
                formatter.write_str("struct p4.v1.Action")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_id__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActionId => {
                            if action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionId"));
                            }
                            action_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
                    action_id: action_id__.unwrap_or_default(),
                    params: params__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Action", FIELDS, GeneratedVisitor)
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
        if self.param_id != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Action.Param", len)?;
        if self.param_id != 0 {
            struct_ser.serialize_field("paramId", &self.param_id)?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
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
            "param_id",
            "paramId",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParamId,
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
                            "paramId" | "param_id" => Ok(GeneratedField::ParamId),
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
            type Value = action::Param;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Action.Param")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<action::Param, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParamId => {
                            if param_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paramId"));
                            }
                            param_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(action::Param {
                    param_id: param_id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Action.Param", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActionProfileAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        if self.weight != 0 {
            len += 1;
        }
        if self.watch_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ActionProfileAction", len)?;
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if let Some(v) = self.watch_kind.as_ref() {
            match v {
                action_profile_action::WatchKind::Watch(v) => {
                    struct_ser.serialize_field("watch", v)?;
                }
                action_profile_action::WatchKind::WatchPort(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("watchPort", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActionProfileAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
            "weight",
            "watch",
            "watch_port",
            "watchPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
            Weight,
            Watch,
            WatchPort,
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
                            "action" => Ok(GeneratedField::Action),
                            "weight" => Ok(GeneratedField::Weight),
                            "watch" => Ok(GeneratedField::Watch),
                            "watchPort" | "watch_port" => Ok(GeneratedField::WatchPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionProfileAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ActionProfileAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActionProfileAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                let mut weight__ = None;
                let mut watch_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Watch => {
                            if watch_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watch"));
                            }
                            watch_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| action_profile_action::WatchKind::Watch(x.0));
                        }
                        GeneratedField::WatchPort => {
                            if watch_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchPort"));
                            }
                            watch_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| action_profile_action::WatchKind::WatchPort(x.0));
                        }
                    }
                }
                Ok(ActionProfileAction {
                    action: action__,
                    weight: weight__.unwrap_or_default(),
                    watch_kind: watch_kind__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ActionProfileAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActionProfileActionSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.action_profile_actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ActionProfileActionSet", len)?;
        if !self.action_profile_actions.is_empty() {
            struct_ser.serialize_field("actionProfileActions", &self.action_profile_actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActionProfileActionSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action_profile_actions",
            "actionProfileActions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActionProfileActions,
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
                            "actionProfileActions" | "action_profile_actions" => Ok(GeneratedField::ActionProfileActions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionProfileActionSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ActionProfileActionSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActionProfileActionSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_profile_actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActionProfileActions => {
                            if action_profile_actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileActions"));
                            }
                            action_profile_actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ActionProfileActionSet {
                    action_profile_actions: action_profile_actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ActionProfileActionSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActionProfileGroup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action_profile_id != 0 {
            len += 1;
        }
        if self.group_id != 0 {
            len += 1;
        }
        if !self.members.is_empty() {
            len += 1;
        }
        if self.max_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ActionProfileGroup", len)?;
        if self.action_profile_id != 0 {
            struct_ser.serialize_field("actionProfileId", &self.action_profile_id)?;
        }
        if self.group_id != 0 {
            struct_ser.serialize_field("groupId", &self.group_id)?;
        }
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if self.max_size != 0 {
            struct_ser.serialize_field("maxSize", &self.max_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActionProfileGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action_profile_id",
            "actionProfileId",
            "group_id",
            "groupId",
            "members",
            "max_size",
            "maxSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActionProfileId,
            GroupId,
            Members,
            MaxSize,
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
                            "actionProfileId" | "action_profile_id" => Ok(GeneratedField::ActionProfileId),
                            "groupId" | "group_id" => Ok(GeneratedField::GroupId),
                            "members" => Ok(GeneratedField::Members),
                            "maxSize" | "max_size" => Ok(GeneratedField::MaxSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionProfileGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ActionProfileGroup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActionProfileGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_profile_id__ = None;
                let mut group_id__ = None;
                let mut members__ = None;
                let mut max_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActionProfileId => {
                            if action_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileId"));
                            }
                            action_profile_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GroupId => {
                            if group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupId"));
                            }
                            group_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxSize => {
                            if max_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSize"));
                            }
                            max_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ActionProfileGroup {
                    action_profile_id: action_profile_id__.unwrap_or_default(),
                    group_id: group_id__.unwrap_or_default(),
                    members: members__.unwrap_or_default(),
                    max_size: max_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ActionProfileGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for action_profile_group::Member {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.member_id != 0 {
            len += 1;
        }
        if self.weight != 0 {
            len += 1;
        }
        if self.watch_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ActionProfileGroup.Member", len)?;
        if self.member_id != 0 {
            struct_ser.serialize_field("memberId", &self.member_id)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if let Some(v) = self.watch_kind.as_ref() {
            match v {
                action_profile_group::member::WatchKind::Watch(v) => {
                    struct_ser.serialize_field("watch", v)?;
                }
                action_profile_group::member::WatchKind::WatchPort(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("watchPort", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for action_profile_group::Member {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "member_id",
            "memberId",
            "weight",
            "watch",
            "watch_port",
            "watchPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MemberId,
            Weight,
            Watch,
            WatchPort,
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
                            "memberId" | "member_id" => Ok(GeneratedField::MemberId),
                            "weight" => Ok(GeneratedField::Weight),
                            "watch" => Ok(GeneratedField::Watch),
                            "watchPort" | "watch_port" => Ok(GeneratedField::WatchPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = action_profile_group::Member;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ActionProfileGroup.Member")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<action_profile_group::Member, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut member_id__ = None;
                let mut weight__ = None;
                let mut watch_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MemberId => {
                            if member_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memberId"));
                            }
                            member_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Watch => {
                            if watch_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watch"));
                            }
                            watch_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| action_profile_group::member::WatchKind::Watch(x.0));
                        }
                        GeneratedField::WatchPort => {
                            if watch_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchPort"));
                            }
                            watch_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| action_profile_group::member::WatchKind::WatchPort(x.0));
                        }
                    }
                }
                Ok(action_profile_group::Member {
                    member_id: member_id__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    watch_kind: watch_kind__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ActionProfileGroup.Member", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActionProfileMember {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action_profile_id != 0 {
            len += 1;
        }
        if self.member_id != 0 {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ActionProfileMember", len)?;
        if self.action_profile_id != 0 {
            struct_ser.serialize_field("actionProfileId", &self.action_profile_id)?;
        }
        if self.member_id != 0 {
            struct_ser.serialize_field("memberId", &self.member_id)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActionProfileMember {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action_profile_id",
            "actionProfileId",
            "member_id",
            "memberId",
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActionProfileId,
            MemberId,
            Action,
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
                            "actionProfileId" | "action_profile_id" => Ok(GeneratedField::ActionProfileId),
                            "memberId" | "member_id" => Ok(GeneratedField::MemberId),
                            "action" => Ok(GeneratedField::Action),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionProfileMember;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ActionProfileMember")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActionProfileMember, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_profile_id__ = None;
                let mut member_id__ = None;
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActionProfileId => {
                            if action_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileId"));
                            }
                            action_profile_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MemberId => {
                            if member_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memberId"));
                            }
                            member_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ActionProfileMember {
                    action_profile_id: action_profile_id__.unwrap_or_default(),
                    member_id: member_id__.unwrap_or_default(),
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ActionProfileMember", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CapabilitiesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.v1.CapabilitiesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesRequest {
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
            type Value = CapabilitiesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.CapabilitiesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CapabilitiesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CapabilitiesRequest {
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.CapabilitiesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CapabilitiesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.p4runtime_api_version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.CapabilitiesResponse", len)?;
        if !self.p4runtime_api_version.is_empty() {
            struct_ser.serialize_field("p4runtimeApiVersion", &self.p4runtime_api_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "p4runtime_api_version",
            "p4runtimeApiVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            P4runtimeApiVersion,
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
                            "p4runtimeApiVersion" | "p4runtime_api_version" => Ok(GeneratedField::P4runtimeApiVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CapabilitiesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.CapabilitiesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CapabilitiesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut p4runtime_api_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::P4runtimeApiVersion => {
                            if p4runtime_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("p4runtimeApiVersion"));
                            }
                            p4runtime_api_version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CapabilitiesResponse {
                    p4runtime_api_version: p4runtime_api_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.CapabilitiesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CloneSessionEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.session_id != 0 {
            len += 1;
        }
        if !self.replicas.is_empty() {
            len += 1;
        }
        if self.class_of_service != 0 {
            len += 1;
        }
        if self.packet_length_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.CloneSessionEntry", len)?;
        if self.session_id != 0 {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if !self.replicas.is_empty() {
            struct_ser.serialize_field("replicas", &self.replicas)?;
        }
        if self.class_of_service != 0 {
            struct_ser.serialize_field("classOfService", &self.class_of_service)?;
        }
        if self.packet_length_bytes != 0 {
            struct_ser.serialize_field("packetLengthBytes", &self.packet_length_bytes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CloneSessionEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "replicas",
            "class_of_service",
            "classOfService",
            "packet_length_bytes",
            "packetLengthBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            Replicas,
            ClassOfService,
            PacketLengthBytes,
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
                            "sessionId" | "session_id" => Ok(GeneratedField::SessionId),
                            "replicas" => Ok(GeneratedField::Replicas),
                            "classOfService" | "class_of_service" => Ok(GeneratedField::ClassOfService),
                            "packetLengthBytes" | "packet_length_bytes" => Ok(GeneratedField::PacketLengthBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CloneSessionEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.CloneSessionEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CloneSessionEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut replicas__ = None;
                let mut class_of_service__ = None;
                let mut packet_length_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Replicas => {
                            if replicas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replicas"));
                            }
                            replicas__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClassOfService => {
                            if class_of_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classOfService"));
                            }
                            class_of_service__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PacketLengthBytes => {
                            if packet_length_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetLengthBytes"));
                            }
                            packet_length_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CloneSessionEntry {
                    session_id: session_id__.unwrap_or_default(),
                    replicas: replicas__.unwrap_or_default(),
                    class_of_service: class_of_service__.unwrap_or_default(),
                    packet_length_bytes: packet_length_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.CloneSessionEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CounterData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.byte_count != 0 {
            len += 1;
        }
        if self.packet_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.CounterData", len)?;
        if self.byte_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("byteCount", ToString::to_string(&self.byte_count).as_str())?;
        }
        if self.packet_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("packetCount", ToString::to_string(&self.packet_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CounterData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "byte_count",
            "byteCount",
            "packet_count",
            "packetCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ByteCount,
            PacketCount,
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
                            "byteCount" | "byte_count" => Ok(GeneratedField::ByteCount),
                            "packetCount" | "packet_count" => Ok(GeneratedField::PacketCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CounterData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.CounterData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CounterData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut byte_count__ = None;
                let mut packet_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ByteCount => {
                            if byte_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("byteCount"));
                            }
                            byte_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PacketCount => {
                            if packet_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetCount"));
                            }
                            packet_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CounterData {
                    byte_count: byte_count__.unwrap_or_default(),
                    packet_count: packet_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.CounterData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CounterEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.counter_id != 0 {
            len += 1;
        }
        if self.index.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.CounterEntry", len)?;
        if self.counter_id != 0 {
            struct_ser.serialize_field("counterId", &self.counter_id)?;
        }
        if let Some(v) = self.index.as_ref() {
            struct_ser.serialize_field("index", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CounterEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "counter_id",
            "counterId",
            "index",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CounterId,
            Index,
            Data,
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
                            "counterId" | "counter_id" => Ok(GeneratedField::CounterId),
                            "index" => Ok(GeneratedField::Index),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CounterEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.CounterEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CounterEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut counter_id__ = None;
                let mut index__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CounterId => {
                            if counter_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterId"));
                            }
                            counter_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = map_.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CounterEntry {
                    counter_id: counter_id__.unwrap_or_default(),
                    index: index__,
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.CounterEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DigestEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.digest_id != 0 {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.DigestEntry", len)?;
        if self.digest_id != 0 {
            struct_ser.serialize_field("digestId", &self.digest_id)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DigestEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest_id",
            "digestId",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DigestId,
            Config,
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
                            "digestId" | "digest_id" => Ok(GeneratedField::DigestId),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DigestEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.DigestEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DigestEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest_id__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DigestId => {
                            if digest_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digestId"));
                            }
                            digest_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DigestEntry {
                    digest_id: digest_id__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.DigestEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for digest_entry::Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_timeout_ns != 0 {
            len += 1;
        }
        if self.max_list_size != 0 {
            len += 1;
        }
        if self.ack_timeout_ns != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.DigestEntry.Config", len)?;
        if self.max_timeout_ns != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxTimeoutNs", ToString::to_string(&self.max_timeout_ns).as_str())?;
        }
        if self.max_list_size != 0 {
            struct_ser.serialize_field("maxListSize", &self.max_list_size)?;
        }
        if self.ack_timeout_ns != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("ackTimeoutNs", ToString::to_string(&self.ack_timeout_ns).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for digest_entry::Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_timeout_ns",
            "maxTimeoutNs",
            "max_list_size",
            "maxListSize",
            "ack_timeout_ns",
            "ackTimeoutNs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxTimeoutNs,
            MaxListSize,
            AckTimeoutNs,
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
                            "maxTimeoutNs" | "max_timeout_ns" => Ok(GeneratedField::MaxTimeoutNs),
                            "maxListSize" | "max_list_size" => Ok(GeneratedField::MaxListSize),
                            "ackTimeoutNs" | "ack_timeout_ns" => Ok(GeneratedField::AckTimeoutNs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = digest_entry::Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.DigestEntry.Config")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<digest_entry::Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_timeout_ns__ = None;
                let mut max_list_size__ = None;
                let mut ack_timeout_ns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxTimeoutNs => {
                            if max_timeout_ns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTimeoutNs"));
                            }
                            max_timeout_ns__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxListSize => {
                            if max_list_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxListSize"));
                            }
                            max_list_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AckTimeoutNs => {
                            if ack_timeout_ns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ackTimeoutNs"));
                            }
                            ack_timeout_ns__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(digest_entry::Config {
                    max_timeout_ns: max_timeout_ns__.unwrap_or_default(),
                    max_list_size: max_list_size__.unwrap_or_default(),
                    ack_timeout_ns: ack_timeout_ns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.DigestEntry.Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DigestList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.digest_id != 0 {
            len += 1;
        }
        if self.list_id != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.DigestList", len)?;
        if self.digest_id != 0 {
            struct_ser.serialize_field("digestId", &self.digest_id)?;
        }
        if self.list_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("listId", ToString::to_string(&self.list_id).as_str())?;
        }
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DigestList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest_id",
            "digestId",
            "list_id",
            "listId",
            "data",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DigestId,
            ListId,
            Data,
            Timestamp,
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
                            "digestId" | "digest_id" => Ok(GeneratedField::DigestId),
                            "listId" | "list_id" => Ok(GeneratedField::ListId),
                            "data" => Ok(GeneratedField::Data),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DigestList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.DigestList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DigestList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest_id__ = None;
                let mut list_id__ = None;
                let mut data__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DigestId => {
                            if digest_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digestId"));
                            }
                            digest_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ListId => {
                            if list_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listId"));
                            }
                            list_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DigestList {
                    digest_id: digest_id__.unwrap_or_default(),
                    list_id: list_id__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.DigestList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DigestListAck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.digest_id != 0 {
            len += 1;
        }
        if self.list_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.DigestListAck", len)?;
        if self.digest_id != 0 {
            struct_ser.serialize_field("digestId", &self.digest_id)?;
        }
        if self.list_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("listId", ToString::to_string(&self.list_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DigestListAck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest_id",
            "digestId",
            "list_id",
            "listId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DigestId,
            ListId,
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
                            "digestId" | "digest_id" => Ok(GeneratedField::DigestId),
                            "listId" | "list_id" => Ok(GeneratedField::ListId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DigestListAck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.DigestListAck")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DigestListAck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest_id__ = None;
                let mut list_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DigestId => {
                            if digest_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digestId"));
                            }
                            digest_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ListId => {
                            if list_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listId"));
                            }
                            list_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DigestListAck {
                    digest_id: digest_id__.unwrap_or_default(),
                    list_id: list_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.DigestListAck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DigestListAckError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.digest_list_ack.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.DigestListAckError", len)?;
        if let Some(v) = self.digest_list_ack.as_ref() {
            struct_ser.serialize_field("digestListAck", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DigestListAckError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest_list_ack",
            "digestListAck",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DigestListAck,
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
                            "digestListAck" | "digest_list_ack" => Ok(GeneratedField::DigestListAck),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DigestListAckError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.DigestListAckError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DigestListAckError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest_list_ack__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DigestListAck => {
                            if digest_list_ack__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digestListAck"));
                            }
                            digest_list_ack__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DigestListAckError {
                    digest_list_ack: digest_list_ack__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.DigestListAckError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectCounterEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_entry.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.DirectCounterEntry", len)?;
        if let Some(v) = self.table_entry.as_ref() {
            struct_ser.serialize_field("tableEntry", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectCounterEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_entry",
            "tableEntry",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableEntry,
            Data,
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
                            "tableEntry" | "table_entry" => Ok(GeneratedField::TableEntry),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectCounterEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.DirectCounterEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DirectCounterEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_entry__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableEntry => {
                            if table_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableEntry"));
                            }
                            table_entry__ = map_.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DirectCounterEntry {
                    table_entry: table_entry__,
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.DirectCounterEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectMeterEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_entry.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.counter_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.DirectMeterEntry", len)?;
        if let Some(v) = self.table_entry.as_ref() {
            struct_ser.serialize_field("tableEntry", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.counter_data.as_ref() {
            struct_ser.serialize_field("counterData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectMeterEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_entry",
            "tableEntry",
            "config",
            "counter_data",
            "counterData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableEntry,
            Config,
            CounterData,
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
                            "tableEntry" | "table_entry" => Ok(GeneratedField::TableEntry),
                            "config" => Ok(GeneratedField::Config),
                            "counterData" | "counter_data" => Ok(GeneratedField::CounterData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectMeterEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.DirectMeterEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DirectMeterEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_entry__ = None;
                let mut config__ = None;
                let mut counter_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableEntry => {
                            if table_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableEntry"));
                            }
                            table_entry__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::CounterData => {
                            if counter_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterData"));
                            }
                            counter_data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DirectMeterEntry {
                    table_entry: table_entry__,
                    config: config__,
                    counter_data: counter_data__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.DirectMeterEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Entity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Entity", len)?;
        if let Some(v) = self.entity.as_ref() {
            match v {
                entity::Entity::ExternEntry(v) => {
                    struct_ser.serialize_field("externEntry", v)?;
                }
                entity::Entity::TableEntry(v) => {
                    struct_ser.serialize_field("tableEntry", v)?;
                }
                entity::Entity::ActionProfileMember(v) => {
                    struct_ser.serialize_field("actionProfileMember", v)?;
                }
                entity::Entity::ActionProfileGroup(v) => {
                    struct_ser.serialize_field("actionProfileGroup", v)?;
                }
                entity::Entity::MeterEntry(v) => {
                    struct_ser.serialize_field("meterEntry", v)?;
                }
                entity::Entity::DirectMeterEntry(v) => {
                    struct_ser.serialize_field("directMeterEntry", v)?;
                }
                entity::Entity::CounterEntry(v) => {
                    struct_ser.serialize_field("counterEntry", v)?;
                }
                entity::Entity::DirectCounterEntry(v) => {
                    struct_ser.serialize_field("directCounterEntry", v)?;
                }
                entity::Entity::PacketReplicationEngineEntry(v) => {
                    struct_ser.serialize_field("packetReplicationEngineEntry", v)?;
                }
                entity::Entity::ValueSetEntry(v) => {
                    struct_ser.serialize_field("valueSetEntry", v)?;
                }
                entity::Entity::RegisterEntry(v) => {
                    struct_ser.serialize_field("registerEntry", v)?;
                }
                entity::Entity::DigestEntry(v) => {
                    struct_ser.serialize_field("digestEntry", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Entity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extern_entry",
            "externEntry",
            "table_entry",
            "tableEntry",
            "action_profile_member",
            "actionProfileMember",
            "action_profile_group",
            "actionProfileGroup",
            "meter_entry",
            "meterEntry",
            "direct_meter_entry",
            "directMeterEntry",
            "counter_entry",
            "counterEntry",
            "direct_counter_entry",
            "directCounterEntry",
            "packet_replication_engine_entry",
            "packetReplicationEngineEntry",
            "value_set_entry",
            "valueSetEntry",
            "register_entry",
            "registerEntry",
            "digest_entry",
            "digestEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternEntry,
            TableEntry,
            ActionProfileMember,
            ActionProfileGroup,
            MeterEntry,
            DirectMeterEntry,
            CounterEntry,
            DirectCounterEntry,
            PacketReplicationEngineEntry,
            ValueSetEntry,
            RegisterEntry,
            DigestEntry,
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
                            "externEntry" | "extern_entry" => Ok(GeneratedField::ExternEntry),
                            "tableEntry" | "table_entry" => Ok(GeneratedField::TableEntry),
                            "actionProfileMember" | "action_profile_member" => Ok(GeneratedField::ActionProfileMember),
                            "actionProfileGroup" | "action_profile_group" => Ok(GeneratedField::ActionProfileGroup),
                            "meterEntry" | "meter_entry" => Ok(GeneratedField::MeterEntry),
                            "directMeterEntry" | "direct_meter_entry" => Ok(GeneratedField::DirectMeterEntry),
                            "counterEntry" | "counter_entry" => Ok(GeneratedField::CounterEntry),
                            "directCounterEntry" | "direct_counter_entry" => Ok(GeneratedField::DirectCounterEntry),
                            "packetReplicationEngineEntry" | "packet_replication_engine_entry" => Ok(GeneratedField::PacketReplicationEngineEntry),
                            "valueSetEntry" | "value_set_entry" => Ok(GeneratedField::ValueSetEntry),
                            "registerEntry" | "register_entry" => Ok(GeneratedField::RegisterEntry),
                            "digestEntry" | "digest_entry" => Ok(GeneratedField::DigestEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Entity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Entity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Entity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExternEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::ExternEntry)
;
                        }
                        GeneratedField::TableEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::TableEntry)
;
                        }
                        GeneratedField::ActionProfileMember => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileMember"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::ActionProfileMember)
;
                        }
                        GeneratedField::ActionProfileGroup => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileGroup"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::ActionProfileGroup)
;
                        }
                        GeneratedField::MeterEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meterEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::MeterEntry)
;
                        }
                        GeneratedField::DirectMeterEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directMeterEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::DirectMeterEntry)
;
                        }
                        GeneratedField::CounterEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::CounterEntry)
;
                        }
                        GeneratedField::DirectCounterEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directCounterEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::DirectCounterEntry)
;
                        }
                        GeneratedField::PacketReplicationEngineEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetReplicationEngineEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::PacketReplicationEngineEntry)
;
                        }
                        GeneratedField::ValueSetEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSetEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::ValueSetEntry)
;
                        }
                        GeneratedField::RegisterEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::RegisterEntry)
;
                        }
                        GeneratedField::DigestEntry => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digestEntry"));
                            }
                            entity__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::Entity::DigestEntry)
;
                        }
                    }
                }
                Ok(Entity {
                    entity: entity__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Entity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Error {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.canonical_code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.space.is_empty() {
            len += 1;
        }
        if self.code != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Error", len)?;
        if self.canonical_code != 0 {
            struct_ser.serialize_field("canonicalCode", &self.canonical_code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.space.is_empty() {
            struct_ser.serialize_field("space", &self.space)?;
        }
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Error {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canonical_code",
            "canonicalCode",
            "message",
            "space",
            "code",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanonicalCode,
            Message,
            Space,
            Code,
            Details,
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
                            "canonicalCode" | "canonical_code" => Ok(GeneratedField::CanonicalCode),
                            "message" => Ok(GeneratedField::Message),
                            "space" => Ok(GeneratedField::Space),
                            "code" => Ok(GeneratedField::Code),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Error;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Error")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Error, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canonical_code__ = None;
                let mut message__ = None;
                let mut space__ = None;
                let mut code__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanonicalCode => {
                            if canonical_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canonicalCode"));
                            }
                            canonical_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Space => {
                            if space__.is_some() {
                                return Err(serde::de::Error::duplicate_field("space"));
                            }
                            space__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Error {
                    canonical_code: canonical_code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    space: space__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Error", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExternEntry {
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
        if self.extern_id != 0 {
            len += 1;
        }
        if self.entry.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ExternEntry", len)?;
        if self.extern_type_id != 0 {
            struct_ser.serialize_field("externTypeId", &self.extern_type_id)?;
        }
        if self.extern_id != 0 {
            struct_ser.serialize_field("externId", &self.extern_id)?;
        }
        if let Some(v) = self.entry.as_ref() {
            struct_ser.serialize_field("entry", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extern_type_id",
            "externTypeId",
            "extern_id",
            "externId",
            "entry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternTypeId,
            ExternId,
            Entry,
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
                            "externId" | "extern_id" => Ok(GeneratedField::ExternId),
                            "entry" => Ok(GeneratedField::Entry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ExternEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExternEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extern_type_id__ = None;
                let mut extern_id__ = None;
                let mut entry__ = None;
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
                        GeneratedField::ExternId => {
                            if extern_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externId"));
                            }
                            extern_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Entry => {
                            if entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entry"));
                            }
                            entry__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExternEntry {
                    extern_type_id: extern_type_id__.unwrap_or_default(),
                    extern_id: extern_id__.unwrap_or_default(),
                    entry: entry__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ExternEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field_id != 0 {
            len += 1;
        }
        if self.field_match_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.FieldMatch", len)?;
        if self.field_id != 0 {
            struct_ser.serialize_field("fieldId", &self.field_id)?;
        }
        if let Some(v) = self.field_match_type.as_ref() {
            match v {
                field_match::FieldMatchType::Exact(v) => {
                    struct_ser.serialize_field("exact", v)?;
                }
                field_match::FieldMatchType::Ternary(v) => {
                    struct_ser.serialize_field("ternary", v)?;
                }
                field_match::FieldMatchType::Lpm(v) => {
                    struct_ser.serialize_field("lpm", v)?;
                }
                field_match::FieldMatchType::Range(v) => {
                    struct_ser.serialize_field("range", v)?;
                }
                field_match::FieldMatchType::Optional(v) => {
                    struct_ser.serialize_field("optional", v)?;
                }
                field_match::FieldMatchType::Other(v) => {
                    struct_ser.serialize_field("other", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_id",
            "fieldId",
            "exact",
            "ternary",
            "lpm",
            "range",
            "optional",
            "other",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldId,
            Exact,
            Ternary,
            Lpm,
            Range,
            Optional,
            Other,
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
                            "fieldId" | "field_id" => Ok(GeneratedField::FieldId),
                            "exact" => Ok(GeneratedField::Exact),
                            "ternary" => Ok(GeneratedField::Ternary),
                            "lpm" => Ok(GeneratedField::Lpm),
                            "range" => Ok(GeneratedField::Range),
                            "optional" => Ok(GeneratedField::Optional),
                            "other" => Ok(GeneratedField::Other),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.FieldMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_id__ = None;
                let mut field_match_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldId => {
                            if field_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldId"));
                            }
                            field_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Exact => {
                            if field_match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exact"));
                            }
                            field_match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_match::FieldMatchType::Exact)
;
                        }
                        GeneratedField::Ternary => {
                            if field_match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ternary"));
                            }
                            field_match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_match::FieldMatchType::Ternary)
;
                        }
                        GeneratedField::Lpm => {
                            if field_match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lpm"));
                            }
                            field_match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_match::FieldMatchType::Lpm)
;
                        }
                        GeneratedField::Range => {
                            if field_match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            field_match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_match::FieldMatchType::Range)
;
                        }
                        GeneratedField::Optional => {
                            if field_match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optional"));
                            }
                            field_match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_match::FieldMatchType::Optional)
;
                        }
                        GeneratedField::Other => {
                            if field_match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("other"));
                            }
                            field_match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_match::FieldMatchType::Other)
;
                        }
                    }
                }
                Ok(FieldMatch {
                    field_id: field_id__.unwrap_or_default(),
                    field_match_type: field_match_type__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.FieldMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_match::Exact {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.FieldMatch.Exact", len)?;
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for field_match::Exact {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = field_match::Exact;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.FieldMatch.Exact")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<field_match::Exact, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(field_match::Exact {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.FieldMatch.Exact", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_match::Lpm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        if self.prefix_len != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.FieldMatch.LPM", len)?;
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        if self.prefix_len != 0 {
            struct_ser.serialize_field("prefixLen", &self.prefix_len)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for field_match::Lpm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "prefix_len",
            "prefixLen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            PrefixLen,
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
                            "value" => Ok(GeneratedField::Value),
                            "prefixLen" | "prefix_len" => Ok(GeneratedField::PrefixLen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_match::Lpm;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.FieldMatch.LPM")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<field_match::Lpm, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut prefix_len__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PrefixLen => {
                            if prefix_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixLen"));
                            }
                            prefix_len__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(field_match::Lpm {
                    value: value__.unwrap_or_default(),
                    prefix_len: prefix_len__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.FieldMatch.LPM", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_match::Optional {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.FieldMatch.Optional", len)?;
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for field_match::Optional {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = field_match::Optional;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.FieldMatch.Optional")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<field_match::Optional, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(field_match::Optional {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.FieldMatch.Optional", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_match::Range {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.low.is_empty() {
            len += 1;
        }
        if !self.high.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.FieldMatch.Range", len)?;
        if !self.low.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("low", pbjson::private::base64::encode(&self.low).as_str())?;
        }
        if !self.high.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("high", pbjson::private::base64::encode(&self.high).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for field_match::Range {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "low",
            "high",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Low,
            High,
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
                            "low" => Ok(GeneratedField::Low),
                            "high" => Ok(GeneratedField::High),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_match::Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.FieldMatch.Range")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<field_match::Range, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut low__ = None;
                let mut high__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Low => {
                            if low__.is_some() {
                                return Err(serde::de::Error::duplicate_field("low"));
                            }
                            low__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::High => {
                            if high__.is_some() {
                                return Err(serde::de::Error::duplicate_field("high"));
                            }
                            high__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(field_match::Range {
                    low: low__.unwrap_or_default(),
                    high: high__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.FieldMatch.Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_match::Ternary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.mask.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.FieldMatch.Ternary", len)?;
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        if !self.mask.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("mask", pbjson::private::base64::encode(&self.mask).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for field_match::Ternary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "mask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Mask,
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
                            "value" => Ok(GeneratedField::Value),
                            "mask" => Ok(GeneratedField::Mask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_match::Ternary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.FieldMatch.Ternary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<field_match::Ternary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mask => {
                            if mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mask"));
                            }
                            mask__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(field_match::Ternary {
                    value: value__.unwrap_or_default(),
                    mask: mask__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.FieldMatch.Ternary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ForwardingPipelineConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.p4info.is_some() {
            len += 1;
        }
        if !self.p4_device_config.is_empty() {
            len += 1;
        }
        if self.cookie.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ForwardingPipelineConfig", len)?;
        if let Some(v) = self.p4info.as_ref() {
            struct_ser.serialize_field("p4info", v)?;
        }
        if !self.p4_device_config.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("p4DeviceConfig", pbjson::private::base64::encode(&self.p4_device_config).as_str())?;
        }
        if let Some(v) = self.cookie.as_ref() {
            struct_ser.serialize_field("cookie", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForwardingPipelineConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "p4info",
            "p4_device_config",
            "p4DeviceConfig",
            "cookie",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            P4info,
            P4DeviceConfig,
            Cookie,
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
                            "p4info" => Ok(GeneratedField::P4info),
                            "p4DeviceConfig" | "p4_device_config" => Ok(GeneratedField::P4DeviceConfig),
                            "cookie" => Ok(GeneratedField::Cookie),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForwardingPipelineConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ForwardingPipelineConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ForwardingPipelineConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut p4info__ = None;
                let mut p4_device_config__ = None;
                let mut cookie__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::P4info => {
                            if p4info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("p4info"));
                            }
                            p4info__ = map_.next_value()?;
                        }
                        GeneratedField::P4DeviceConfig => {
                            if p4_device_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("p4DeviceConfig"));
                            }
                            p4_device_config__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cookie => {
                            if cookie__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cookie"));
                            }
                            cookie__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ForwardingPipelineConfig {
                    p4info: p4info__,
                    p4_device_config: p4_device_config__.unwrap_or_default(),
                    cookie: cookie__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ForwardingPipelineConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for forwarding_pipeline_config::Cookie {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cookie != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ForwardingPipelineConfig.Cookie", len)?;
        if self.cookie != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("cookie", ToString::to_string(&self.cookie).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for forwarding_pipeline_config::Cookie {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cookie",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cookie,
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
                            "cookie" => Ok(GeneratedField::Cookie),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = forwarding_pipeline_config::Cookie;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ForwardingPipelineConfig.Cookie")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<forwarding_pipeline_config::Cookie, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cookie__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cookie => {
                            if cookie__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cookie"));
                            }
                            cookie__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(forwarding_pipeline_config::Cookie {
                    cookie: cookie__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ForwardingPipelineConfig.Cookie", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetForwardingPipelineConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_id != 0 {
            len += 1;
        }
        if self.response_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.GetForwardingPipelineConfigRequest", len)?;
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if self.response_type != 0 {
            let v = get_forwarding_pipeline_config_request::ResponseType::try_from(self.response_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.response_type)))?;
            struct_ser.serialize_field("responseType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetForwardingPipelineConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_id",
            "deviceId",
            "response_type",
            "responseType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceId,
            ResponseType,
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
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "responseType" | "response_type" => Ok(GeneratedField::ResponseType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetForwardingPipelineConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.GetForwardingPipelineConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetForwardingPipelineConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_id__ = None;
                let mut response_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseType => {
                            if response_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseType"));
                            }
                            response_type__ = Some(map_.next_value::<get_forwarding_pipeline_config_request::ResponseType>()? as i32);
                        }
                    }
                }
                Ok(GetForwardingPipelineConfigRequest {
                    device_id: device_id__.unwrap_or_default(),
                    response_type: response_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.GetForwardingPipelineConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_forwarding_pipeline_config_request::ResponseType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::All => "ALL",
            Self::CookieOnly => "COOKIE_ONLY",
            Self::P4infoAndCookie => "P4INFO_AND_COOKIE",
            Self::DeviceConfigAndCookie => "DEVICE_CONFIG_AND_COOKIE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for get_forwarding_pipeline_config_request::ResponseType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALL",
            "COOKIE_ONLY",
            "P4INFO_AND_COOKIE",
            "DEVICE_CONFIG_AND_COOKIE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_forwarding_pipeline_config_request::ResponseType;

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
                    "ALL" => Ok(get_forwarding_pipeline_config_request::ResponseType::All),
                    "COOKIE_ONLY" => Ok(get_forwarding_pipeline_config_request::ResponseType::CookieOnly),
                    "P4INFO_AND_COOKIE" => Ok(get_forwarding_pipeline_config_request::ResponseType::P4infoAndCookie),
                    "DEVICE_CONFIG_AND_COOKIE" => Ok(get_forwarding_pipeline_config_request::ResponseType::DeviceConfigAndCookie),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetForwardingPipelineConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.GetForwardingPipelineConfigResponse", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetForwardingPipelineConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
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
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetForwardingPipelineConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.GetForwardingPipelineConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetForwardingPipelineConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetForwardingPipelineConfigResponse {
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.GetForwardingPipelineConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdleTimeoutNotification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_entry.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.IdleTimeoutNotification", len)?;
        if !self.table_entry.is_empty() {
            struct_ser.serialize_field("tableEntry", &self.table_entry)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdleTimeoutNotification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_entry",
            "tableEntry",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableEntry,
            Timestamp,
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
                            "tableEntry" | "table_entry" => Ok(GeneratedField::TableEntry),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdleTimeoutNotification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.IdleTimeoutNotification")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdleTimeoutNotification, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_entry__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableEntry => {
                            if table_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableEntry"));
                            }
                            table_entry__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(IdleTimeoutNotification {
                    table_entry: table_entry__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.IdleTimeoutNotification", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Index {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Index", len)?;
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Index {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
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
                            "index" => Ok(GeneratedField::Index),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Index;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Index")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Index, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Index {
                    index: index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Index", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MasterArbitrationUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_id != 0 {
            len += 1;
        }
        if self.role.is_some() {
            len += 1;
        }
        if self.election_id.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.MasterArbitrationUpdate", len)?;
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if let Some(v) = self.role.as_ref() {
            struct_ser.serialize_field("role", v)?;
        }
        if let Some(v) = self.election_id.as_ref() {
            struct_ser.serialize_field("electionId", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MasterArbitrationUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_id",
            "deviceId",
            "role",
            "election_id",
            "electionId",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceId,
            Role,
            ElectionId,
            Status,
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
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "role" => Ok(GeneratedField::Role),
                            "electionId" | "election_id" => Ok(GeneratedField::ElectionId),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MasterArbitrationUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.MasterArbitrationUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MasterArbitrationUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_id__ = None;
                let mut role__ = None;
                let mut election_id__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = map_.next_value()?;
                        }
                        GeneratedField::ElectionId => {
                            if election_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("electionId"));
                            }
                            election_id__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MasterArbitrationUpdate {
                    device_id: device_id__.unwrap_or_default(),
                    role: role__,
                    election_id: election_id__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.MasterArbitrationUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cir != 0 {
            len += 1;
        }
        if self.cburst != 0 {
            len += 1;
        }
        if self.pir != 0 {
            len += 1;
        }
        if self.pburst != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.MeterConfig", len)?;
        if self.cir != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("cir", ToString::to_string(&self.cir).as_str())?;
        }
        if self.cburst != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("cburst", ToString::to_string(&self.cburst).as_str())?;
        }
        if self.pir != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("pir", ToString::to_string(&self.pir).as_str())?;
        }
        if self.pburst != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("pburst", ToString::to_string(&self.pburst).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cir",
            "cburst",
            "pir",
            "pburst",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cir,
            Cburst,
            Pir,
            Pburst,
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
                            "cir" => Ok(GeneratedField::Cir),
                            "cburst" => Ok(GeneratedField::Cburst),
                            "pir" => Ok(GeneratedField::Pir),
                            "pburst" => Ok(GeneratedField::Pburst),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.MeterConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MeterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cir__ = None;
                let mut cburst__ = None;
                let mut pir__ = None;
                let mut pburst__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cir => {
                            if cir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cir"));
                            }
                            cir__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cburst => {
                            if cburst__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cburst"));
                            }
                            cburst__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pir => {
                            if pir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pir"));
                            }
                            pir__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pburst => {
                            if pburst__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pburst"));
                            }
                            pburst__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MeterConfig {
                    cir: cir__.unwrap_or_default(),
                    cburst: cburst__.unwrap_or_default(),
                    pir: pir__.unwrap_or_default(),
                    pburst: pburst__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.MeterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeterCounterData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.green.is_some() {
            len += 1;
        }
        if self.yellow.is_some() {
            len += 1;
        }
        if self.red.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.MeterCounterData", len)?;
        if let Some(v) = self.green.as_ref() {
            struct_ser.serialize_field("green", v)?;
        }
        if let Some(v) = self.yellow.as_ref() {
            struct_ser.serialize_field("yellow", v)?;
        }
        if let Some(v) = self.red.as_ref() {
            struct_ser.serialize_field("red", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeterCounterData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "green",
            "yellow",
            "red",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Green,
            Yellow,
            Red,
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
                            "green" => Ok(GeneratedField::Green),
                            "yellow" => Ok(GeneratedField::Yellow),
                            "red" => Ok(GeneratedField::Red),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeterCounterData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.MeterCounterData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MeterCounterData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut green__ = None;
                let mut yellow__ = None;
                let mut red__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Green => {
                            if green__.is_some() {
                                return Err(serde::de::Error::duplicate_field("green"));
                            }
                            green__ = map_.next_value()?;
                        }
                        GeneratedField::Yellow => {
                            if yellow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("yellow"));
                            }
                            yellow__ = map_.next_value()?;
                        }
                        GeneratedField::Red => {
                            if red__.is_some() {
                                return Err(serde::de::Error::duplicate_field("red"));
                            }
                            red__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MeterCounterData {
                    green: green__,
                    yellow: yellow__,
                    red: red__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.MeterCounterData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeterEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.meter_id != 0 {
            len += 1;
        }
        if self.index.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.counter_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.MeterEntry", len)?;
        if self.meter_id != 0 {
            struct_ser.serialize_field("meterId", &self.meter_id)?;
        }
        if let Some(v) = self.index.as_ref() {
            struct_ser.serialize_field("index", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.counter_data.as_ref() {
            struct_ser.serialize_field("counterData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeterEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "meter_id",
            "meterId",
            "index",
            "config",
            "counter_data",
            "counterData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MeterId,
            Index,
            Config,
            CounterData,
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
                            "meterId" | "meter_id" => Ok(GeneratedField::MeterId),
                            "index" => Ok(GeneratedField::Index),
                            "config" => Ok(GeneratedField::Config),
                            "counterData" | "counter_data" => Ok(GeneratedField::CounterData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeterEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.MeterEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MeterEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut meter_id__ = None;
                let mut index__ = None;
                let mut config__ = None;
                let mut counter_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MeterId => {
                            if meter_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meterId"));
                            }
                            meter_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::CounterData => {
                            if counter_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterData"));
                            }
                            counter_data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MeterEntry {
                    meter_id: meter_id__.unwrap_or_default(),
                    index: index__,
                    config: config__,
                    counter_data: counter_data__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.MeterEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MulticastGroupEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.multicast_group_id != 0 {
            len += 1;
        }
        if !self.replicas.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.MulticastGroupEntry", len)?;
        if self.multicast_group_id != 0 {
            struct_ser.serialize_field("multicastGroupId", &self.multicast_group_id)?;
        }
        if !self.replicas.is_empty() {
            struct_ser.serialize_field("replicas", &self.replicas)?;
        }
        if !self.metadata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MulticastGroupEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "multicast_group_id",
            "multicastGroupId",
            "replicas",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MulticastGroupId,
            Replicas,
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
                            "multicastGroupId" | "multicast_group_id" => Ok(GeneratedField::MulticastGroupId),
                            "replicas" => Ok(GeneratedField::Replicas),
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
            type Value = MulticastGroupEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.MulticastGroupEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MulticastGroupEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut multicast_group_id__ = None;
                let mut replicas__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MulticastGroupId => {
                            if multicast_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multicastGroupId"));
                            }
                            multicast_group_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Replicas => {
                            if replicas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replicas"));
                            }
                            replicas__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MulticastGroupEntry {
                    multicast_group_id: multicast_group_id__.unwrap_or_default(),
                    replicas: replicas__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.MulticastGroupEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4Data {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.P4Data", len)?;
        if let Some(v) = self.data.as_ref() {
            match v {
                p4_data::Data::Bitstring(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("bitstring", pbjson::private::base64::encode(&v).as_str())?;
                }
                p4_data::Data::Varbit(v) => {
                    struct_ser.serialize_field("varbit", v)?;
                }
                p4_data::Data::Bool(v) => {
                    struct_ser.serialize_field("bool", v)?;
                }
                p4_data::Data::Tuple(v) => {
                    struct_ser.serialize_field("tuple", v)?;
                }
                p4_data::Data::Struct(v) => {
                    struct_ser.serialize_field("struct", v)?;
                }
                p4_data::Data::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                p4_data::Data::HeaderUnion(v) => {
                    struct_ser.serialize_field("headerUnion", v)?;
                }
                p4_data::Data::HeaderStack(v) => {
                    struct_ser.serialize_field("headerStack", v)?;
                }
                p4_data::Data::HeaderUnionStack(v) => {
                    struct_ser.serialize_field("headerUnionStack", v)?;
                }
                p4_data::Data::Enum(v) => {
                    struct_ser.serialize_field("enum", v)?;
                }
                p4_data::Data::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                p4_data::Data::EnumValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("enumValue", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4Data {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bitstring",
            "varbit",
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
            "enum_value",
            "enumValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bitstring,
            Varbit,
            Bool,
            Tuple,
            Struct,
            Header,
            HeaderUnion,
            HeaderStack,
            HeaderUnionStack,
            Enum,
            Error,
            EnumValue,
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
                            "varbit" => Ok(GeneratedField::Varbit),
                            "bool" => Ok(GeneratedField::Bool),
                            "tuple" => Ok(GeneratedField::Tuple),
                            "struct" => Ok(GeneratedField::Struct),
                            "header" => Ok(GeneratedField::Header),
                            "headerUnion" | "header_union" => Ok(GeneratedField::HeaderUnion),
                            "headerStack" | "header_stack" => Ok(GeneratedField::HeaderStack),
                            "headerUnionStack" | "header_union_stack" => Ok(GeneratedField::HeaderUnionStack),
                            "enum" => Ok(GeneratedField::Enum),
                            "error" => Ok(GeneratedField::Error),
                            "enumValue" | "enum_value" => Ok(GeneratedField::EnumValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4Data;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.P4Data")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4Data, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bitstring => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitstring"));
                            }
                            data__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| p4_data::Data::Bitstring(x.0));
                        }
                        GeneratedField::Varbit => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("varbit"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::Varbit)
;
                        }
                        GeneratedField::Bool => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bool"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::Bool);
                        }
                        GeneratedField::Tuple => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::Tuple)
;
                        }
                        GeneratedField::Struct => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("struct"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::Struct)
;
                        }
                        GeneratedField::Header => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::Header)
;
                        }
                        GeneratedField::HeaderUnion => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerUnion"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::HeaderUnion)
;
                        }
                        GeneratedField::HeaderStack => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerStack"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::HeaderStack)
;
                        }
                        GeneratedField::HeaderUnionStack => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerUnionStack"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::HeaderUnionStack)
;
                        }
                        GeneratedField::Enum => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enum"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::Enum);
                        }
                        GeneratedField::Error => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(p4_data::Data::Error);
                        }
                        GeneratedField::EnumValue => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumValue"));
                            }
                            data__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| p4_data::Data::EnumValue(x.0));
                        }
                    }
                }
                Ok(P4Data {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.P4Data", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4Header {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_valid {
            len += 1;
        }
        if !self.bitstrings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.P4Header", len)?;
        if self.is_valid {
            struct_ser.serialize_field("isValid", &self.is_valid)?;
        }
        if !self.bitstrings.is_empty() {
            struct_ser.serialize_field("bitstrings", &self.bitstrings.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4Header {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_valid",
            "isValid",
            "bitstrings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsValid,
            Bitstrings,
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
                            "isValid" | "is_valid" => Ok(GeneratedField::IsValid),
                            "bitstrings" => Ok(GeneratedField::Bitstrings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4Header;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.P4Header")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4Header, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_valid__ = None;
                let mut bitstrings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsValid => {
                            if is_valid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isValid"));
                            }
                            is_valid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bitstrings => {
                            if bitstrings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitstrings"));
                            }
                            bitstrings__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(P4Header {
                    is_valid: is_valid__.unwrap_or_default(),
                    bitstrings: bitstrings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.P4Header", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4HeaderStack {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.P4HeaderStack", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4HeaderStack {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
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
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4HeaderStack;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.P4HeaderStack")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4HeaderStack, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4HeaderStack {
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.P4HeaderStack", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4HeaderUnion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.valid_header_name.is_empty() {
            len += 1;
        }
        if self.valid_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.P4HeaderUnion", len)?;
        if !self.valid_header_name.is_empty() {
            struct_ser.serialize_field("validHeaderName", &self.valid_header_name)?;
        }
        if let Some(v) = self.valid_header.as_ref() {
            struct_ser.serialize_field("validHeader", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4HeaderUnion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "valid_header_name",
            "validHeaderName",
            "valid_header",
            "validHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidHeaderName,
            ValidHeader,
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
                            "validHeaderName" | "valid_header_name" => Ok(GeneratedField::ValidHeaderName),
                            "validHeader" | "valid_header" => Ok(GeneratedField::ValidHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4HeaderUnion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.P4HeaderUnion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4HeaderUnion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut valid_header_name__ = None;
                let mut valid_header__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidHeaderName => {
                            if valid_header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validHeaderName"));
                            }
                            valid_header_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidHeader => {
                            if valid_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validHeader"));
                            }
                            valid_header__ = map_.next_value()?;
                        }
                    }
                }
                Ok(P4HeaderUnion {
                    valid_header_name: valid_header_name__.unwrap_or_default(),
                    valid_header: valid_header__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.P4HeaderUnion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4HeaderUnionStack {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.P4HeaderUnionStack", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4HeaderUnionStack {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
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
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = P4HeaderUnionStack;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.P4HeaderUnionStack")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4HeaderUnionStack, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(P4HeaderUnionStack {
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.P4HeaderUnionStack", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4StructLike {
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
        let mut struct_ser = serializer.serialize_struct("p4.v1.P4StructLike", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4StructLike {
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
            type Value = P4StructLike;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.P4StructLike")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4StructLike, V::Error>
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
                Ok(P4StructLike {
                    members: members__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.P4StructLike", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for P4Varbit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bitstring.is_empty() {
            len += 1;
        }
        if self.bitwidth != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.P4Varbit", len)?;
        if !self.bitstring.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("bitstring", pbjson::private::base64::encode(&self.bitstring).as_str())?;
        }
        if self.bitwidth != 0 {
            struct_ser.serialize_field("bitwidth", &self.bitwidth)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for P4Varbit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bitstring",
            "bitwidth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bitstring,
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
                            "bitstring" => Ok(GeneratedField::Bitstring),
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
            type Value = P4Varbit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.P4Varbit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<P4Varbit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bitstring__ = None;
                let mut bitwidth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bitstring => {
                            if bitstring__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitstring"));
                            }
                            bitstring__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
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
                Ok(P4Varbit {
                    bitstring: bitstring__.unwrap_or_default(),
                    bitwidth: bitwidth__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.P4Varbit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PacketIn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payload.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.PacketIn", len)?;
        if !self.payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PacketIn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payload",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Payload,
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
                            "payload" => Ok(GeneratedField::Payload),
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
            type Value = PacketIn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.PacketIn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PacketIn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PacketIn {
                    payload: payload__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.PacketIn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PacketMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata_id != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.PacketMetadata", len)?;
        if self.metadata_id != 0 {
            struct_ser.serialize_field("metadataId", &self.metadata_id)?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PacketMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_id",
            "metadataId",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataId,
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
                            "metadataId" | "metadata_id" => Ok(GeneratedField::MetadataId),
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
            type Value = PacketMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.PacketMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PacketMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetadataId => {
                            if metadata_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataId"));
                            }
                            metadata_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PacketMetadata {
                    metadata_id: metadata_id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.PacketMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PacketOut {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payload.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.PacketOut", len)?;
        if !self.payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PacketOut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payload",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Payload,
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
                            "payload" => Ok(GeneratedField::Payload),
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
            type Value = PacketOut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.PacketOut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PacketOut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PacketOut {
                    payload: payload__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.PacketOut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PacketOutError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.packet_out.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.PacketOutError", len)?;
        if let Some(v) = self.packet_out.as_ref() {
            struct_ser.serialize_field("packetOut", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PacketOutError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "packet_out",
            "packetOut",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PacketOut,
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
                            "packetOut" | "packet_out" => Ok(GeneratedField::PacketOut),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PacketOutError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.PacketOutError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PacketOutError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut packet_out__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PacketOut => {
                            if packet_out__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetOut"));
                            }
                            packet_out__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PacketOutError {
                    packet_out: packet_out__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.PacketOutError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PacketReplicationEngineEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.PacketReplicationEngineEntry", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                packet_replication_engine_entry::Type::MulticastGroupEntry(v) => {
                    struct_ser.serialize_field("multicastGroupEntry", v)?;
                }
                packet_replication_engine_entry::Type::CloneSessionEntry(v) => {
                    struct_ser.serialize_field("cloneSessionEntry", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PacketReplicationEngineEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "multicast_group_entry",
            "multicastGroupEntry",
            "clone_session_entry",
            "cloneSessionEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MulticastGroupEntry,
            CloneSessionEntry,
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
                            "multicastGroupEntry" | "multicast_group_entry" => Ok(GeneratedField::MulticastGroupEntry),
                            "cloneSessionEntry" | "clone_session_entry" => Ok(GeneratedField::CloneSessionEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PacketReplicationEngineEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.PacketReplicationEngineEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PacketReplicationEngineEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MulticastGroupEntry => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multicastGroupEntry"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(packet_replication_engine_entry::Type::MulticastGroupEntry)
;
                        }
                        GeneratedField::CloneSessionEntry => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cloneSessionEntry"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(packet_replication_engine_entry::Type::CloneSessionEntry)
;
                        }
                    }
                }
                Ok(PacketReplicationEngineEntry {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.PacketReplicationEngineEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_id != 0 {
            len += 1;
        }
        if !self.role.is_empty() {
            len += 1;
        }
        if !self.entities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ReadRequest", len)?;
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        if !self.entities.is_empty() {
            struct_ser.serialize_field("entities", &self.entities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_id",
            "deviceId",
            "role",
            "entities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceId,
            Role,
            Entities,
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
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "role" => Ok(GeneratedField::Role),
                            "entities" => Ok(GeneratedField::Entities),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ReadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_id__ = None;
                let mut role__ = None;
                let mut entities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entities => {
                            if entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entities"));
                            }
                            entities__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadRequest {
                    device_id: device_id__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    entities: entities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ReadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ReadResponse", len)?;
        if !self.entities.is_empty() {
            struct_ser.serialize_field("entities", &self.entities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entities,
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
                            "entities" => Ok(GeneratedField::Entities),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ReadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entities => {
                            if entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entities"));
                            }
                            entities__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadResponse {
                    entities: entities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ReadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.register_id != 0 {
            len += 1;
        }
        if self.index.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.RegisterEntry", len)?;
        if self.register_id != 0 {
            struct_ser.serialize_field("registerId", &self.register_id)?;
        }
        if let Some(v) = self.index.as_ref() {
            struct_ser.serialize_field("index", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "register_id",
            "registerId",
            "index",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisterId,
            Index,
            Data,
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
                            "registerId" | "register_id" => Ok(GeneratedField::RegisterId),
                            "index" => Ok(GeneratedField::Index),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.RegisterEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut register_id__ = None;
                let mut index__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RegisterId => {
                            if register_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerId"));
                            }
                            register_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = map_.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegisterEntry {
                    register_id: register_id__.unwrap_or_default(),
                    index: index__,
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.RegisterEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Replica {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instance != 0 {
            len += 1;
        }
        if self.port_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Replica", len)?;
        if self.instance != 0 {
            struct_ser.serialize_field("instance", &self.instance)?;
        }
        if let Some(v) = self.port_kind.as_ref() {
            match v {
                replica::PortKind::EgressPort(v) => {
                    struct_ser.serialize_field("egressPort", v)?;
                }
                replica::PortKind::Port(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("port", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Replica {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "egress_port",
            "egressPort",
            "port",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            EgressPort,
            Port,
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
                            "instance" => Ok(GeneratedField::Instance),
                            "egressPort" | "egress_port" => Ok(GeneratedField::EgressPort),
                            "port" => Ok(GeneratedField::Port),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Replica;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Replica")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Replica, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut port_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EgressPort => {
                            if port_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("egressPort"));
                            }
                            port_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| replica::PortKind::EgressPort(x.0));
                        }
                        GeneratedField::Port => {
                            if port_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| replica::PortKind::Port(x.0));
                        }
                    }
                }
                Ok(Replica {
                    instance: instance__.unwrap_or_default(),
                    port_kind: port_kind__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Replica", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Role {
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
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Role", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Role {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Config,
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
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Role;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Role")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Role, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut config__ = None;
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
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Role {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Role", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SdnPort {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "SDN_PORT_UNKNOWN",
            Self::Min => "SDN_PORT_MIN",
            Self::Max => "SDN_PORT_MAX",
            Self::Recirculate => "SDN_PORT_RECIRCULATE",
            Self::Cpu => "SDN_PORT_CPU",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SdnPort {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SDN_PORT_UNKNOWN",
            "SDN_PORT_MIN",
            "SDN_PORT_MAX",
            "SDN_PORT_RECIRCULATE",
            "SDN_PORT_CPU",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SdnPort;

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
                    "SDN_PORT_UNKNOWN" => Ok(SdnPort::Unknown),
                    "SDN_PORT_MIN" => Ok(SdnPort::Min),
                    "SDN_PORT_MAX" => Ok(SdnPort::Max),
                    "SDN_PORT_RECIRCULATE" => Ok(SdnPort::Recirculate),
                    "SDN_PORT_CPU" => Ok(SdnPort::Cpu),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SetForwardingPipelineConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_id != 0 {
            len += 1;
        }
        if self.role_id != 0 {
            len += 1;
        }
        if !self.role.is_empty() {
            len += 1;
        }
        if self.election_id.is_some() {
            len += 1;
        }
        if self.action != 0 {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.SetForwardingPipelineConfigRequest", len)?;
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if self.role_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roleId", ToString::to_string(&self.role_id).as_str())?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        if let Some(v) = self.election_id.as_ref() {
            struct_ser.serialize_field("electionId", v)?;
        }
        if self.action != 0 {
            let v = set_forwarding_pipeline_config_request::Action::try_from(self.action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetForwardingPipelineConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_id",
            "deviceId",
            "role_id",
            "roleId",
            "role",
            "election_id",
            "electionId",
            "action",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceId,
            RoleId,
            Role,
            ElectionId,
            Action,
            Config,
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
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "roleId" | "role_id" => Ok(GeneratedField::RoleId),
                            "role" => Ok(GeneratedField::Role),
                            "electionId" | "election_id" => Ok(GeneratedField::ElectionId),
                            "action" => Ok(GeneratedField::Action),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetForwardingPipelineConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.SetForwardingPipelineConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetForwardingPipelineConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_id__ = None;
                let mut role_id__ = None;
                let mut role__ = None;
                let mut election_id__ = None;
                let mut action__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RoleId => {
                            if role_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleId"));
                            }
                            role_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ElectionId => {
                            if election_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("electionId"));
                            }
                            election_id__ = map_.next_value()?;
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map_.next_value::<set_forwarding_pipeline_config_request::Action>()? as i32);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetForwardingPipelineConfigRequest {
                    device_id: device_id__.unwrap_or_default(),
                    role_id: role_id__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    election_id: election_id__,
                    action: action__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.SetForwardingPipelineConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for set_forwarding_pipeline_config_request::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Verify => "VERIFY",
            Self::VerifyAndSave => "VERIFY_AND_SAVE",
            Self::VerifyAndCommit => "VERIFY_AND_COMMIT",
            Self::Commit => "COMMIT",
            Self::ReconcileAndCommit => "RECONCILE_AND_COMMIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for set_forwarding_pipeline_config_request::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "VERIFY",
            "VERIFY_AND_SAVE",
            "VERIFY_AND_COMMIT",
            "COMMIT",
            "RECONCILE_AND_COMMIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = set_forwarding_pipeline_config_request::Action;

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
                    "UNSPECIFIED" => Ok(set_forwarding_pipeline_config_request::Action::Unspecified),
                    "VERIFY" => Ok(set_forwarding_pipeline_config_request::Action::Verify),
                    "VERIFY_AND_SAVE" => Ok(set_forwarding_pipeline_config_request::Action::VerifyAndSave),
                    "VERIFY_AND_COMMIT" => Ok(set_forwarding_pipeline_config_request::Action::VerifyAndCommit),
                    "COMMIT" => Ok(set_forwarding_pipeline_config_request::Action::Commit),
                    "RECONCILE_AND_COMMIT" => Ok(set_forwarding_pipeline_config_request::Action::ReconcileAndCommit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SetForwardingPipelineConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.v1.SetForwardingPipelineConfigResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetForwardingPipelineConfigResponse {
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
            type Value = SetForwardingPipelineConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.SetForwardingPipelineConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetForwardingPipelineConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SetForwardingPipelineConfigResponse {
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.SetForwardingPipelineConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.canonical_code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.space.is_empty() {
            len += 1;
        }
        if self.code != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.StreamError", len)?;
        if self.canonical_code != 0 {
            struct_ser.serialize_field("canonicalCode", &self.canonical_code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.space.is_empty() {
            struct_ser.serialize_field("space", &self.space)?;
        }
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.details.as_ref() {
            match v {
                stream_error::Details::PacketOut(v) => {
                    struct_ser.serialize_field("packetOut", v)?;
                }
                stream_error::Details::DigestListAck(v) => {
                    struct_ser.serialize_field("digestListAck", v)?;
                }
                stream_error::Details::Other(v) => {
                    struct_ser.serialize_field("other", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canonical_code",
            "canonicalCode",
            "message",
            "space",
            "code",
            "packet_out",
            "packetOut",
            "digest_list_ack",
            "digestListAck",
            "other",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanonicalCode,
            Message,
            Space,
            Code,
            PacketOut,
            DigestListAck,
            Other,
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
                            "canonicalCode" | "canonical_code" => Ok(GeneratedField::CanonicalCode),
                            "message" => Ok(GeneratedField::Message),
                            "space" => Ok(GeneratedField::Space),
                            "code" => Ok(GeneratedField::Code),
                            "packetOut" | "packet_out" => Ok(GeneratedField::PacketOut),
                            "digestListAck" | "digest_list_ack" => Ok(GeneratedField::DigestListAck),
                            "other" => Ok(GeneratedField::Other),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.StreamError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canonical_code__ = None;
                let mut message__ = None;
                let mut space__ = None;
                let mut code__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanonicalCode => {
                            if canonical_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canonicalCode"));
                            }
                            canonical_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Space => {
                            if space__.is_some() {
                                return Err(serde::de::Error::duplicate_field("space"));
                            }
                            space__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PacketOut => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetOut"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_error::Details::PacketOut)
;
                        }
                        GeneratedField::DigestListAck => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digestListAck"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_error::Details::DigestListAck)
;
                        }
                        GeneratedField::Other => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("other"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_error::Details::Other)
;
                        }
                    }
                }
                Ok(StreamError {
                    canonical_code: canonical_code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    space: space__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.StreamError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.StreamMessageRequest", len)?;
        if let Some(v) = self.update.as_ref() {
            match v {
                stream_message_request::Update::Arbitration(v) => {
                    struct_ser.serialize_field("arbitration", v)?;
                }
                stream_message_request::Update::Packet(v) => {
                    struct_ser.serialize_field("packet", v)?;
                }
                stream_message_request::Update::DigestAck(v) => {
                    struct_ser.serialize_field("digestAck", v)?;
                }
                stream_message_request::Update::Other(v) => {
                    struct_ser.serialize_field("other", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "arbitration",
            "packet",
            "digest_ack",
            "digestAck",
            "other",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Arbitration,
            Packet,
            DigestAck,
            Other,
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
                            "arbitration" => Ok(GeneratedField::Arbitration),
                            "packet" => Ok(GeneratedField::Packet),
                            "digestAck" | "digest_ack" => Ok(GeneratedField::DigestAck),
                            "other" => Ok(GeneratedField::Other),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.StreamMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Arbitration => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arbitration"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_request::Update::Arbitration)
;
                        }
                        GeneratedField::Packet => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packet"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_request::Update::Packet)
;
                        }
                        GeneratedField::DigestAck => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digestAck"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_request::Update::DigestAck)
;
                        }
                        GeneratedField::Other => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("other"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_request::Update::Other)
;
                        }
                    }
                }
                Ok(StreamMessageRequest {
                    update: update__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.StreamMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.StreamMessageResponse", len)?;
        if let Some(v) = self.update.as_ref() {
            match v {
                stream_message_response::Update::Arbitration(v) => {
                    struct_ser.serialize_field("arbitration", v)?;
                }
                stream_message_response::Update::Packet(v) => {
                    struct_ser.serialize_field("packet", v)?;
                }
                stream_message_response::Update::Digest(v) => {
                    struct_ser.serialize_field("digest", v)?;
                }
                stream_message_response::Update::IdleTimeoutNotification(v) => {
                    struct_ser.serialize_field("idleTimeoutNotification", v)?;
                }
                stream_message_response::Update::Other(v) => {
                    struct_ser.serialize_field("other", v)?;
                }
                stream_message_response::Update::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "arbitration",
            "packet",
            "digest",
            "idle_timeout_notification",
            "idleTimeoutNotification",
            "other",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Arbitration,
            Packet,
            Digest,
            IdleTimeoutNotification,
            Other,
            Error,
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
                            "arbitration" => Ok(GeneratedField::Arbitration),
                            "packet" => Ok(GeneratedField::Packet),
                            "digest" => Ok(GeneratedField::Digest),
                            "idleTimeoutNotification" | "idle_timeout_notification" => Ok(GeneratedField::IdleTimeoutNotification),
                            "other" => Ok(GeneratedField::Other),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.StreamMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Arbitration => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arbitration"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_response::Update::Arbitration)
;
                        }
                        GeneratedField::Packet => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packet"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_response::Update::Packet)
;
                        }
                        GeneratedField::Digest => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_response::Update::Digest)
;
                        }
                        GeneratedField::IdleTimeoutNotification => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeoutNotification"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_response::Update::IdleTimeoutNotification)
;
                        }
                        GeneratedField::Other => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("other"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_response::Update::Other)
;
                        }
                        GeneratedField::Error => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            update__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_message_response::Update::Error)
;
                        }
                    }
                }
                Ok(StreamMessageResponse {
                    update: update__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.StreamMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamOtherError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.other.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.StreamOtherError", len)?;
        if let Some(v) = self.other.as_ref() {
            struct_ser.serialize_field("other", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamOtherError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "other",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Other,
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
                            "other" => Ok(GeneratedField::Other),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamOtherError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.StreamOtherError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamOtherError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut other__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Other => {
                            if other__.is_some() {
                                return Err(serde::de::Error::duplicate_field("other"));
                            }
                            other__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamOtherError {
                    other: other__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.StreamOtherError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.TableAction", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                table_action::Type::Action(v) => {
                    struct_ser.serialize_field("action", v)?;
                }
                table_action::Type::ActionProfileMemberId(v) => {
                    struct_ser.serialize_field("actionProfileMemberId", v)?;
                }
                table_action::Type::ActionProfileGroupId(v) => {
                    struct_ser.serialize_field("actionProfileGroupId", v)?;
                }
                table_action::Type::ActionProfileActionSet(v) => {
                    struct_ser.serialize_field("actionProfileActionSet", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
            "action_profile_member_id",
            "actionProfileMemberId",
            "action_profile_group_id",
            "actionProfileGroupId",
            "action_profile_action_set",
            "actionProfileActionSet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
            ActionProfileMemberId,
            ActionProfileGroupId,
            ActionProfileActionSet,
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
                            "action" => Ok(GeneratedField::Action),
                            "actionProfileMemberId" | "action_profile_member_id" => Ok(GeneratedField::ActionProfileMemberId),
                            "actionProfileGroupId" | "action_profile_group_id" => Ok(GeneratedField::ActionProfileGroupId),
                            "actionProfileActionSet" | "action_profile_action_set" => Ok(GeneratedField::ActionProfileActionSet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.TableAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TableAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(table_action::Type::Action)
;
                        }
                        GeneratedField::ActionProfileMemberId => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileMemberId"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| table_action::Type::ActionProfileMemberId(x.0));
                        }
                        GeneratedField::ActionProfileGroupId => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileGroupId"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| table_action::Type::ActionProfileGroupId(x.0));
                        }
                        GeneratedField::ActionProfileActionSet => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionProfileActionSet"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(table_action::Type::ActionProfileActionSet)
;
                        }
                    }
                }
                Ok(TableAction {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.TableAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_id != 0 {
            len += 1;
        }
        if !self.r#match.is_empty() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.controller_metadata != 0 {
            len += 1;
        }
        if self.meter_config.is_some() {
            len += 1;
        }
        if self.counter_data.is_some() {
            len += 1;
        }
        if self.meter_counter_data.is_some() {
            len += 1;
        }
        if self.is_default_action {
            len += 1;
        }
        if self.idle_timeout_ns != 0 {
            len += 1;
        }
        if self.time_since_last_hit.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.is_const {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.TableEntry", len)?;
        if self.table_id != 0 {
            struct_ser.serialize_field("tableId", &self.table_id)?;
        }
        if !self.r#match.is_empty() {
            struct_ser.serialize_field("match", &self.r#match)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if self.controller_metadata != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("controllerMetadata", ToString::to_string(&self.controller_metadata).as_str())?;
        }
        if let Some(v) = self.meter_config.as_ref() {
            struct_ser.serialize_field("meterConfig", v)?;
        }
        if let Some(v) = self.counter_data.as_ref() {
            struct_ser.serialize_field("counterData", v)?;
        }
        if let Some(v) = self.meter_counter_data.as_ref() {
            struct_ser.serialize_field("meterCounterData", v)?;
        }
        if self.is_default_action {
            struct_ser.serialize_field("isDefaultAction", &self.is_default_action)?;
        }
        if self.idle_timeout_ns != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("idleTimeoutNs", ToString::to_string(&self.idle_timeout_ns).as_str())?;
        }
        if let Some(v) = self.time_since_last_hit.as_ref() {
            struct_ser.serialize_field("timeSinceLastHit", v)?;
        }
        if !self.metadata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
        }
        if self.is_const {
            struct_ser.serialize_field("isConst", &self.is_const)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_id",
            "tableId",
            "match",
            "action",
            "priority",
            "controller_metadata",
            "controllerMetadata",
            "meter_config",
            "meterConfig",
            "counter_data",
            "counterData",
            "meter_counter_data",
            "meterCounterData",
            "is_default_action",
            "isDefaultAction",
            "idle_timeout_ns",
            "idleTimeoutNs",
            "time_since_last_hit",
            "timeSinceLastHit",
            "metadata",
            "is_const",
            "isConst",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableId,
            Match,
            Action,
            Priority,
            ControllerMetadata,
            MeterConfig,
            CounterData,
            MeterCounterData,
            IsDefaultAction,
            IdleTimeoutNs,
            TimeSinceLastHit,
            Metadata,
            IsConst,
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
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            "match" => Ok(GeneratedField::Match),
                            "action" => Ok(GeneratedField::Action),
                            "priority" => Ok(GeneratedField::Priority),
                            "controllerMetadata" | "controller_metadata" => Ok(GeneratedField::ControllerMetadata),
                            "meterConfig" | "meter_config" => Ok(GeneratedField::MeterConfig),
                            "counterData" | "counter_data" => Ok(GeneratedField::CounterData),
                            "meterCounterData" | "meter_counter_data" => Ok(GeneratedField::MeterCounterData),
                            "isDefaultAction" | "is_default_action" => Ok(GeneratedField::IsDefaultAction),
                            "idleTimeoutNs" | "idle_timeout_ns" => Ok(GeneratedField::IdleTimeoutNs),
                            "timeSinceLastHit" | "time_since_last_hit" => Ok(GeneratedField::TimeSinceLastHit),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "isConst" | "is_const" => Ok(GeneratedField::IsConst),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.TableEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TableEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_id__ = None;
                let mut r#match__ = None;
                let mut action__ = None;
                let mut priority__ = None;
                let mut controller_metadata__ = None;
                let mut meter_config__ = None;
                let mut counter_data__ = None;
                let mut meter_counter_data__ = None;
                let mut is_default_action__ = None;
                let mut idle_timeout_ns__ = None;
                let mut time_since_last_hit__ = None;
                let mut metadata__ = None;
                let mut is_const__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ControllerMetadata => {
                            if controller_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controllerMetadata"));
                            }
                            controller_metadata__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MeterConfig => {
                            if meter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meterConfig"));
                            }
                            meter_config__ = map_.next_value()?;
                        }
                        GeneratedField::CounterData => {
                            if counter_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterData"));
                            }
                            counter_data__ = map_.next_value()?;
                        }
                        GeneratedField::MeterCounterData => {
                            if meter_counter_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meterCounterData"));
                            }
                            meter_counter_data__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefaultAction => {
                            if is_default_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefaultAction"));
                            }
                            is_default_action__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdleTimeoutNs => {
                            if idle_timeout_ns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeoutNs"));
                            }
                            idle_timeout_ns__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TimeSinceLastHit => {
                            if time_since_last_hit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeSinceLastHit"));
                            }
                            time_since_last_hit__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsConst => {
                            if is_const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isConst"));
                            }
                            is_const__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TableEntry {
                    table_id: table_id__.unwrap_or_default(),
                    r#match: r#match__.unwrap_or_default(),
                    action: action__,
                    priority: priority__.unwrap_or_default(),
                    controller_metadata: controller_metadata__.unwrap_or_default(),
                    meter_config: meter_config__,
                    counter_data: counter_data__,
                    meter_counter_data: meter_counter_data__,
                    is_default_action: is_default_action__.unwrap_or_default(),
                    idle_timeout_ns: idle_timeout_ns__.unwrap_or_default(),
                    time_since_last_hit: time_since_last_hit__,
                    metadata: metadata__.unwrap_or_default(),
                    is_const: is_const__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.TableEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for table_entry::IdleTimeout {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.elapsed_ns != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.TableEntry.IdleTimeout", len)?;
        if self.elapsed_ns != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("elapsedNs", ToString::to_string(&self.elapsed_ns).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for table_entry::IdleTimeout {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "elapsed_ns",
            "elapsedNs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ElapsedNs,
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
                            "elapsedNs" | "elapsed_ns" => Ok(GeneratedField::ElapsedNs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = table_entry::IdleTimeout;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.TableEntry.IdleTimeout")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<table_entry::IdleTimeout, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut elapsed_ns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ElapsedNs => {
                            if elapsed_ns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elapsedNs"));
                            }
                            elapsed_ns__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(table_entry::IdleTimeout {
                    elapsed_ns: elapsed_ns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.TableEntry.IdleTimeout", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Uint128 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.high != 0 {
            len += 1;
        }
        if self.low != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Uint128", len)?;
        if self.high != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("high", ToString::to_string(&self.high).as_str())?;
        }
        if self.low != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("low", ToString::to_string(&self.low).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Uint128 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "high",
            "low",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            High,
            Low,
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
                            "high" => Ok(GeneratedField::High),
                            "low" => Ok(GeneratedField::Low),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Uint128;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Uint128")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Uint128, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut high__ = None;
                let mut low__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::High => {
                            if high__.is_some() {
                                return Err(serde::de::Error::duplicate_field("high"));
                            }
                            high__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Low => {
                            if low__.is_some() {
                                return Err(serde::de::Error::duplicate_field("low"));
                            }
                            low__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Uint128 {
                    high: high__.unwrap_or_default(),
                    low: low__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Uint128", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Update {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.entity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.Update", len)?;
        if self.r#type != 0 {
            let v = update::Type::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Update {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "entity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Entity,
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
                            "type" => Ok(GeneratedField::Type),
                            "entity" => Ok(GeneratedField::Entity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Update;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.Update")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Update, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<update::Type>()? as i32);
                        }
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Update {
                    r#type: r#type__.unwrap_or_default(),
                    entity: entity__,
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.Update", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for update::Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Insert => "INSERT",
            Self::Modify => "MODIFY",
            Self::Delete => "DELETE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for update::Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "INSERT",
            "MODIFY",
            "DELETE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = update::Type;

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
                    "UNSPECIFIED" => Ok(update::Type::Unspecified),
                    "INSERT" => Ok(update::Type::Insert),
                    "MODIFY" => Ok(update::Type::Modify),
                    "DELETE" => Ok(update::Type::Delete),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ValueSetEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_set_id != 0 {
            len += 1;
        }
        if !self.members.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ValueSetEntry", len)?;
        if self.value_set_id != 0 {
            struct_ser.serialize_field("valueSetId", &self.value_set_id)?;
        }
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValueSetEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_set_id",
            "valueSetId",
            "members",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueSetId,
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
                            "valueSetId" | "value_set_id" => Ok(GeneratedField::ValueSetId),
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
            type Value = ValueSetEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ValueSetEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValueSetEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_set_id__ = None;
                let mut members__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueSetId => {
                            if value_set_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSetId"));
                            }
                            value_set_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValueSetEntry {
                    value_set_id: value_set_id__.unwrap_or_default(),
                    members: members__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ValueSetEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValueSetMember {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#match.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.ValueSetMember", len)?;
        if !self.r#match.is_empty() {
            struct_ser.serialize_field("match", &self.r#match)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValueSetMember {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "match",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Match,
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
                            "match" => Ok(GeneratedField::Match),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValueSetMember;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.ValueSetMember")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValueSetMember, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#match__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValueSetMember {
                    r#match: r#match__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.ValueSetMember", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_id != 0 {
            len += 1;
        }
        if self.role_id != 0 {
            len += 1;
        }
        if !self.role.is_empty() {
            len += 1;
        }
        if self.election_id.is_some() {
            len += 1;
        }
        if !self.updates.is_empty() {
            len += 1;
        }
        if self.atomicity != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("p4.v1.WriteRequest", len)?;
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if self.role_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roleId", ToString::to_string(&self.role_id).as_str())?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        if let Some(v) = self.election_id.as_ref() {
            struct_ser.serialize_field("electionId", v)?;
        }
        if !self.updates.is_empty() {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        if self.atomicity != 0 {
            let v = write_request::Atomicity::try_from(self.atomicity)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.atomicity)))?;
            struct_ser.serialize_field("atomicity", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_id",
            "deviceId",
            "role_id",
            "roleId",
            "role",
            "election_id",
            "electionId",
            "updates",
            "atomicity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceId,
            RoleId,
            Role,
            ElectionId,
            Updates,
            Atomicity,
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
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "roleId" | "role_id" => Ok(GeneratedField::RoleId),
                            "role" => Ok(GeneratedField::Role),
                            "electionId" | "election_id" => Ok(GeneratedField::ElectionId),
                            "updates" => Ok(GeneratedField::Updates),
                            "atomicity" => Ok(GeneratedField::Atomicity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.WriteRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_id__ = None;
                let mut role_id__ = None;
                let mut role__ = None;
                let mut election_id__ = None;
                let mut updates__ = None;
                let mut atomicity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RoleId => {
                            if role_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleId"));
                            }
                            role_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ElectionId => {
                            if election_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("electionId"));
                            }
                            election_id__ = map_.next_value()?;
                        }
                        GeneratedField::Updates => {
                            if updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Atomicity => {
                            if atomicity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("atomicity"));
                            }
                            atomicity__ = Some(map_.next_value::<write_request::Atomicity>()? as i32);
                        }
                    }
                }
                Ok(WriteRequest {
                    device_id: device_id__.unwrap_or_default(),
                    role_id: role_id__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    election_id: election_id__,
                    updates: updates__.unwrap_or_default(),
                    atomicity: atomicity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.WriteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for write_request::Atomicity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ContinueOnError => "CONTINUE_ON_ERROR",
            Self::RollbackOnError => "ROLLBACK_ON_ERROR",
            Self::DataplaneAtomic => "DATAPLANE_ATOMIC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for write_request::Atomicity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTINUE_ON_ERROR",
            "ROLLBACK_ON_ERROR",
            "DATAPLANE_ATOMIC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = write_request::Atomicity;

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
                    "CONTINUE_ON_ERROR" => Ok(write_request::Atomicity::ContinueOnError),
                    "ROLLBACK_ON_ERROR" => Ok(write_request::Atomicity::RollbackOnError),
                    "DATAPLANE_ATOMIC" => Ok(write_request::Atomicity::DataplaneAtomic),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WriteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("p4.v1.WriteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteResponse {
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
            type Value = WriteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct p4.v1.WriteResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WriteResponse {
                })
            }
        }
        deserializer.deserialize_struct("p4.v1.WriteResponse", FIELDS, GeneratedVisitor)
    }
}

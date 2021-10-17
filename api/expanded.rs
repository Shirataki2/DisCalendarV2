#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate async_trait;
mod client {
    use crate::error::ApiError;
    use actix_web::HttpRequest;
    use anyhow::Context;
    use reqwest::header;
    use serenity::model::{guild::PartialMember, id::GuildId, user::CurrentUser};
    #[allow(dead_code)]
    const API_ROOT_URL: &str = "https://discord.com/api";
    pub fn create_client(token: String) -> Result<reqwest::Client, ApiError> {
        let mut headers = header::HeaderMap::new();
        let token = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Bot "],
                &match (&&token,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        };
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&token).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to get client")?;
        Ok(client)
    }
    pub struct GuildInfo {
        pub id: GuildId,
        pub icon: Option<String>,
        pub name: String,
        #[serde(default)]
        pub owner: bool,
        #[serde(rename = "permissions_new")]
        pub permissions: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for GuildInfo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                GuildInfo {
                    id: ref __self_0_0,
                    icon: ref __self_0_1,
                    name: ref __self_0_2,
                    owner: ref __self_0_3,
                    permissions: ref __self_0_4,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "GuildInfo");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "icon",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "owner",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "permissions",
                        &&(*__self_0_4),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for GuildInfo {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "icon" => _serde::__private::Ok(__Field::__field1),
                            "name" => _serde::__private::Ok(__Field::__field2),
                            "owner" => _serde::__private::Ok(__Field::__field3),
                            "permissions_new" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"icon" => _serde::__private::Ok(__Field::__field1),
                            b"name" => _serde::__private::Ok(__Field::__field2),
                            b"owner" => _serde::__private::Ok(__Field::__field3),
                            b"permissions_new" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GuildInfo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GuildInfo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct GuildInfo")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<GuildId>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GuildInfo with 5 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct GuildInfo with 5 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct GuildInfo with 5 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => _serde::__private::Default::default(),
                            };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct GuildInfo with 5 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(GuildInfo {
                            id: __field0,
                            icon: __field1,
                            name: __field2,
                            owner: __field3,
                            permissions: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<GuildId> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<GuildId>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "icon",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "owner",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "permissions_new",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("icon") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("permissions_new") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GuildInfo {
                            id: __field0,
                            icon: __field1,
                            name: __field2,
                            owner: __field3,
                            permissions: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["id", "icon", "name", "owner", "permissions_new"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GuildInfo",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GuildInfo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct PartialGuild {
        pub id: GuildId,
        pub icon: Option<String>,
        pub name: String,
        #[serde(default)]
        pub owner: bool,
        #[serde(rename = "permissions_new")]
        pub permissions: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PartialGuild {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PartialGuild {
                    id: ref __self_0_0,
                    icon: ref __self_0_1,
                    name: ref __self_0_2,
                    owner: ref __self_0_3,
                    permissions: ref __self_0_4,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "PartialGuild");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "icon",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "owner",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "permissions",
                        &&(*__self_0_4),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PartialGuild {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "icon" => _serde::__private::Ok(__Field::__field1),
                            "name" => _serde::__private::Ok(__Field::__field2),
                            "owner" => _serde::__private::Ok(__Field::__field3),
                            "permissions_new" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"icon" => _serde::__private::Ok(__Field::__field1),
                            b"name" => _serde::__private::Ok(__Field::__field2),
                            b"owner" => _serde::__private::Ok(__Field::__field3),
                            b"permissions_new" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PartialGuild>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PartialGuild;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct PartialGuild")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<GuildId>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct PartialGuild with 5 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct PartialGuild with 5 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct PartialGuild with 5 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => _serde::__private::Default::default(),
                            };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct PartialGuild with 5 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(PartialGuild {
                            id: __field0,
                            icon: __field1,
                            name: __field2,
                            owner: __field3,
                            permissions: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<GuildId> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<GuildId>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "icon",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "owner",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "permissions_new",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("icon") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("permissions_new") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(PartialGuild {
                            id: __field0,
                            icon: __field1,
                            name: __field2,
                            owner: __field3,
                            permissions: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["id", "icon", "name", "owner", "permissions_new"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PartialGuild",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PartialGuild>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PartialGuild {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "PartialGuild",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "icon",
                    &self.icon,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "owner",
                    &self.owner,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "permissions_new",
                    &self.permissions,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub trait DiscordClient {
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn get_current_user_guilds<'life0, 'life1, 'async_trait>(
            &'life0 self,
            user_token: &'life1 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<Vec<GuildInfo>, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn get_current_user<'life0, 'life1, 'async_trait>(
            &'life0 self,
            user_token: &'life1 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<CurrentUser, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn get_guild<'life0, 'life1, 'async_trait>(
            &'life0 self,
            guild_id: &'life1 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<PartialGuild, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn get_guild_member<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 self,
            guild_id: &'life1 str,
            user_id: &'life2 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<PartialMember, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn check_member<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 self,
            guild_id: &'life1 str,
            token: &'life2 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<PartialGuild, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait;
        fn get_user_token(&self, req: &HttpRequest) -> Result<String, ApiError>;
    }
    impl DiscordClient for reqwest::Client {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_current_user_guilds<'life0, 'life1, 'async_trait>(
            &'life0 self,
            user_token: &'life1 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<Vec<GuildInfo>, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<Vec<GuildInfo>, ApiError>>
                {
                    return __ret;
                }
                let __self = self;
                let user_token = user_token;
                let __ret: Result<Vec<GuildInfo>, ApiError> = {
                    let url = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", "/users/@me/guilds"],
                            &match (&API_ROOT_URL,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    };
                    let mut headers = header::HeaderMap::new();
                    let token = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Bearer "],
                            &match (&&user_token,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    };
                    headers.insert(
                        header::AUTHORIZATION,
                        header::HeaderValue::from_str(&token).unwrap(),
                    );
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Getting current user guilds"],
                                    &match () {
                                        () => [],
                                    },
                                ),
                                lvl,
                                &("api::client", "api::client", "src/client.rs", 82u32),
                            );
                        }
                    };
                    let guilds = __self
                        .get(&url)
                        .headers(headers)
                        .send()
                        .await?
                        .json::<Vec<GuildInfo>>()
                        .await?;
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Fetched ", " guilds"],
                                    &match (&guilds.len(),) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ),
                                lvl,
                                &("api::client", "api::client", "src/client.rs", 91u32),
                            );
                        }
                    };
                    Ok(guilds)
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_current_user<'life0, 'life1, 'async_trait>(
            &'life0 self,
            user_token: &'life1 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<CurrentUser, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<CurrentUser, ApiError>>
                {
                    return __ret;
                }
                let __self = self;
                let user_token = user_token;
                let __ret: Result<CurrentUser, ApiError> = {
                    let url = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", "/users/@me"],
                            &match (&API_ROOT_URL,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    };
                    let mut headers = header::HeaderMap::new();
                    let token = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Bearer "],
                            &match (&&user_token,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    };
                    headers.insert(
                        header::AUTHORIZATION,
                        header::HeaderValue::from_str(&token).unwrap(),
                    );
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Getting current user"],
                                    &match () {
                                        () => [],
                                    },
                                ),
                                lvl,
                                &("api::client", "api::client", "src/client.rs", 103u32),
                            );
                        }
                    };
                    let user = __self
                        .get(&url)
                        .headers(headers)
                        .send()
                        .await?
                        .json::<CurrentUser>()
                        .await?;
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Fetched user: "],
                                    &match (&user,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Debug::fmt,
                                        )],
                                    },
                                ),
                                lvl,
                                &("api::client", "api::client", "src/client.rs", 111u32),
                            );
                        }
                    };
                    Ok(user)
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_guild<'life0, 'life1, 'async_trait>(
            &'life0 self,
            guild_id: &'life1 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<PartialGuild, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<PartialGuild, ApiError>>
                {
                    return __ret;
                }
                let __self = self;
                let guild_id = guild_id;
                let __ret: Result<PartialGuild, ApiError> = {
                    let url = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", "/guilds/"],
                            &match (&API_ROOT_URL, &guild_id) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    };
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Getting guild: "],
                                    &match (&guild_id,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ),
                                lvl,
                                &("api::client", "api::client", "src/client.rs", 117u32),
                            );
                        }
                    };
                    let guild = __self
                        .get(&url)
                        .send()
                        .await?
                        .json::<PartialGuild>()
                        .await?;
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Fetched guild: "],
                                    &match (&guild,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Debug::fmt,
                                        )],
                                    },
                                ),
                                lvl,
                                &("api::client", "api::client", "src/client.rs", 124u32),
                            );
                        }
                    };
                    Ok(guild)
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_guild_member<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 self,
            guild_id: &'life1 str,
            user_id: &'life2 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<PartialMember, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<PartialMember, ApiError>>
                {
                    return __ret;
                }
                let __self = self;
                let guild_id = guild_id;
                let user_id = user_id;
                let __ret: Result<PartialMember, ApiError> = {
                    let url = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", "/guilds/", "/members/"],
                            &match (&API_ROOT_URL, &guild_id, &user_id) {
                                (arg0, arg1, arg2) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    };
                    let member = __self
                        .get(&url)
                        .send()
                        .await?
                        .json::<PartialMember>()
                        .await?;
                    Ok(member)
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn check_member<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 self,
            guild_id: &'life1 str,
            token: &'life2 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<PartialGuild, ApiError>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<PartialGuild, ApiError>>
                {
                    return __ret;
                }
                let __self = self;
                let guild_id = guild_id;
                let token = token;
                let __ret: Result<PartialGuild, ApiError> = {
                    let user_guilds = __self.get_current_user_guilds(token).await?;
                    let user: CurrentUser = __self.get_current_user(token).await?;
                    __self
                        .get_guild_member(guild_id, &user.id.0.to_string())
                        .await?;
                    let guild = __self.get_guild(guild_id).await?;
                    match user_guilds
                        .iter()
                        .find(|&guild| guild.id.0.to_string() == guild_id)
                    {
                        Some(_) => Ok(guild),
                        None => Err(ApiError::Unauthorized),
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        fn get_user_token(&self, req: &HttpRequest) -> Result<String, ApiError> {
            let token = req
                .cookie("access_token")
                .ok_or(ApiError::Unauthorized)?
                .value()
                .to_string();
            Ok(token)
        }
    }
}
mod error {
    use actix_web::{http::StatusCode, HttpResponse, ResponseError};
    pub enum ApiError {
        #[error("Unauthorized")]
        Unauthorized,
        #[error("Requested resource is not found")]
        NotFound,
        #[error("Failed to fetch data from discord.\n{0}")]
        DiscordClientError(#[from] reqwest::Error),
        #[error("Failed to fetch data: {0}")]
        DatabaseError(#[from] sqlx::Error),
        #[error("Failed to fetch data")]
        DeserializeFailed(#[from] serde::de::value::Error),
        #[error("{0}")]
        Internal(#[from] actix_web::error::Error),
        #[error("{0}")]
        Other(#[from] anyhow::Error),
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for ApiError {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;
            #[allow(deprecated)]
            match self {
                ApiError::Unauthorized { .. } => std::option::Option::None,
                ApiError::NotFound { .. } => std::option::Option::None,
                ApiError::DiscordClientError { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                ApiError::DatabaseError { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                ApiError::DeserializeFailed { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                ApiError::Internal { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                ApiError::Other { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for ApiError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_imports)]
            use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                ApiError::Unauthorized {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Unauthorized"],
                    &match () {
                        () => [],
                    },
                )),
                ApiError::NotFound {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Requested resource is not found"],
                    &match () {
                        () => [],
                    },
                )),
                ApiError::DiscordClientError(_0) => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Failed to fetch data from discord.\n"],
                        &match (&_0.as_display(),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                ApiError::DatabaseError(_0) => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Failed to fetch data: "],
                        &match (&_0.as_display(),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                ApiError::DeserializeFailed(_0) => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Failed to fetch data"],
                        &match () {
                            () => [],
                        },
                    ))
                }
                ApiError::Internal(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&_0.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
                ApiError::Other(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&_0.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<reqwest::Error> for ApiError {
        #[allow(deprecated)]
        fn from(source: reqwest::Error) -> Self {
            ApiError::DiscordClientError { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<sqlx::Error> for ApiError {
        #[allow(deprecated)]
        fn from(source: sqlx::Error) -> Self {
            ApiError::DatabaseError { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<serde::de::value::Error> for ApiError {
        #[allow(deprecated)]
        fn from(source: serde::de::value::Error) -> Self {
            ApiError::DeserializeFailed { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<actix_web::error::Error> for ApiError {
        #[allow(deprecated)]
        fn from(source: actix_web::error::Error) -> Self {
            ApiError::Internal { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<anyhow::Error> for ApiError {
        #[allow(deprecated)]
        fn from(source: anyhow::Error) -> Self {
            ApiError::Other { 0: source }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ApiError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ApiError::Unauthorized,) => ::core::fmt::Formatter::write_str(f, "Unauthorized"),
                (&ApiError::NotFound,) => ::core::fmt::Formatter::write_str(f, "NotFound"),
                (&ApiError::DiscordClientError(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "DiscordClientError");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ApiError::DatabaseError(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "DatabaseError");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ApiError::DeserializeFailed(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "DeserializeFailed");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ApiError::Internal(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Internal");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ApiError::Other(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Other");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    struct ErrorResponse {
        code: u16,
        message: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ErrorResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ErrorResponse",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "code",
                    &self.code,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message",
                    &self.message,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ErrorResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ErrorResponse {
                    code: ref __self_0_0,
                    message: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ErrorResponse");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "code",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "message",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ResponseError for ApiError {
        fn status_code(&self) -> StatusCode {
            match *self {
                ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
                ApiError::NotFound => StatusCode::NOT_FOUND,
                ApiError::DiscordClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::DeserializeFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::Other(_) => StatusCode::SERVICE_UNAVAILABLE,
            }
        }
        fn error_response(&self) -> HttpResponse {
            let status_code = self.status_code();
            let resp = ErrorResponse {
                code: status_code.as_u16(),
                message: self.to_string(),
            };
            HttpResponse::build(status_code).json(resp)
        }
    }
}
#[allow(clippy::match_single_binding)]
mod models {
    pub mod event_settings {
        use sqlx_macros::*;
        #[table(name = "event_settings")]
        pub struct EventSettings {
            #[table(pk, skip)]
            pub id: i32,
            pub guild_id: String,
            pub channel_id: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for EventSettings {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    EventSettings {
                        id: ref __self_0_0,
                        guild_id: ref __self_0_1,
                        channel_id: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "EventSettings");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "guild_id",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "channel_id",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for EventSettings {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "EventSettings",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "guild_id",
                        &self.guild_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "channel_id",
                        &self.channel_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for EventSettings {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "guild_id" => _serde::__private::Ok(__Field::__field1),
                                "channel_id" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"guild_id" => _serde::__private::Ok(__Field::__field1),
                                b"channel_id" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<EventSettings>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = EventSettings;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct EventSettings",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct EventSettings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct EventSettings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct EventSettings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(EventSettings {
                                id: __field0,
                                guild_id: __field1,
                                channel_id: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<i32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "guild_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "channel_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("guild_id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("channel_id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(EventSettings {
                                id: __field0,
                                guild_id: __field1,
                                channel_id: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["id", "guild_id", "channel_id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "EventSettings",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<EventSettings>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl EventSettings {
            pub async fn get(pool: &::sqlx::PgPool, id: i32) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("SELECT * FROM event_settings WHERE id = $1" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_channel_id = row . try_get_unchecked :: < String , _ > (2usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , channel_id : sqlx_query_as_channel_id , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
        impl EventSettings {
            pub async fn set_guild_id(
                self,
                pool: &::sqlx::PgPool,
                guild_id: &String,
            ) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (guild_id) ; let arg1 = & (self . id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg1) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (2usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg1)) ; query_args . add (arg0) ; query_args . add (arg1) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("UPDATE event_settings SET guild_id = $1 WHERE id = $2 RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_channel_id = row . try_get_unchecked :: < String , _ > (2usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , channel_id : sqlx_query_as_channel_id , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
            pub async fn set_channel_id(
                self,
                pool: &::sqlx::PgPool,
                channel_id: &String,
            ) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (channel_id) ; let arg1 = & (self . id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg1) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (2usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg1)) ; query_args . add (arg0) ; query_args . add (arg1) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("UPDATE event_settings SET channel_id = $1 WHERE id = $2 RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_channel_id = row . try_get_unchecked :: < String , _ > (2usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , channel_id : sqlx_query_as_channel_id , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
        impl EventSettings {
            #[allow(clippy::too_many_arguments)]
            pub async fn create(
                pool: &::sqlx::PgPool,
                guild_id: &String,
                channel_id: &String,
            ) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (guild_id) ; let arg1 = & (channel_id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg1) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (2usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg1)) ; query_args . add (arg0) ; query_args . add (arg1) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("INSERT INTO event_settings (guild_id,channel_id) VALUES ($1,$2) RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_channel_id = row . try_get_unchecked :: < String , _ > (2usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , channel_id : sqlx_query_as_channel_id , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
        impl EventSettings {
            pub async fn delete(&self, pool: &::sqlx::PgPool) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (self . id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("DELETE FROM event_settings WHERE id = $1 RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_channel_id = row . try_get_unchecked :: < String , _ > (2usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , channel_id : sqlx_query_as_channel_id , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
    }
    pub mod events {
        use sqlx_macros::*;
        use chrono::NaiveDateTime;
        #[table(name = "events")]
        pub struct Events {
            #[table(pk, skip)]
            pub id: i32,
            pub guild_id: String,
            pub name: String,
            pub description: Option<String>,
            pub notifications: Vec<String>,
            pub color: String,
            pub is_all_day: bool,
            pub start_at: NaiveDateTime,
            pub end_at: NaiveDateTime,
            pub created_at: NaiveDateTime,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Events {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Events {
                        id: ref __self_0_0,
                        guild_id: ref __self_0_1,
                        name: ref __self_0_2,
                        description: ref __self_0_3,
                        notifications: ref __self_0_4,
                        color: ref __self_0_5,
                        is_all_day: ref __self_0_6,
                        start_at: ref __self_0_7,
                        end_at: ref __self_0_8,
                        created_at: ref __self_0_9,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Events");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "guild_id",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "name",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "description",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "notifications",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "color",
                            &&(*__self_0_5),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "is_all_day",
                            &&(*__self_0_6),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "start_at",
                            &&(*__self_0_7),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "end_at",
                            &&(*__self_0_8),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "created_at",
                            &&(*__self_0_9),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Events {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Events",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "guild_id",
                        &self.guild_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "notifications",
                        &self.notifications,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "color",
                        &self.color,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_all_day",
                        &self.is_all_day,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "start_at",
                        &self.start_at,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "end_at",
                        &self.end_at,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "created_at",
                        &self.created_at,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Events {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "guild_id" => _serde::__private::Ok(__Field::__field1),
                                "name" => _serde::__private::Ok(__Field::__field2),
                                "description" => _serde::__private::Ok(__Field::__field3),
                                "notifications" => _serde::__private::Ok(__Field::__field4),
                                "color" => _serde::__private::Ok(__Field::__field5),
                                "is_all_day" => _serde::__private::Ok(__Field::__field6),
                                "start_at" => _serde::__private::Ok(__Field::__field7),
                                "end_at" => _serde::__private::Ok(__Field::__field8),
                                "created_at" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"guild_id" => _serde::__private::Ok(__Field::__field1),
                                b"name" => _serde::__private::Ok(__Field::__field2),
                                b"description" => _serde::__private::Ok(__Field::__field3),
                                b"notifications" => _serde::__private::Ok(__Field::__field4),
                                b"color" => _serde::__private::Ok(__Field::__field5),
                                b"is_all_day" => _serde::__private::Ok(__Field::__field6),
                                b"start_at" => _serde::__private::Ok(__Field::__field7),
                                b"end_at" => _serde::__private::Ok(__Field::__field8),
                                b"created_at" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Events>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Events;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Events")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Vec<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 =
                                match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                6usize,
                                                &"struct Events with 10 elements",
                                            ),
                                        );
                                    }
                                };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                NaiveDateTime,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                NaiveDateTime,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<
                                NaiveDateTime,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Events with 10 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Events {
                                id: __field0,
                                guild_id: __field1,
                                name: __field2,
                                description: __field3,
                                notifications: __field4,
                                color: __field5,
                                is_all_day: __field6,
                                start_at: __field7,
                                end_at: __field8,
                                created_at: __field9,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<i32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<Vec<String>> =
                                _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field6: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            let mut __field7: _serde::__private::Option<NaiveDateTime> =
                                _serde::__private::None;
                            let mut __field8: _serde::__private::Option<NaiveDateTime> =
                                _serde::__private::None;
                            let mut __field9: _serde::__private::Option<NaiveDateTime> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "guild_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "notifications",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "color",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "is_all_day",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<bool>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "start_at",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<NaiveDateTime>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "end_at",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<NaiveDateTime>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "created_at",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<NaiveDateTime>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("guild_id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("description") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("notifications") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("color") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("is_all_day") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("start_at") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("end_at") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("created_at") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Events {
                                id: __field0,
                                guild_id: __field1,
                                name: __field2,
                                description: __field3,
                                notifications: __field4,
                                color: __field5,
                                is_all_day: __field6,
                                start_at: __field7,
                                end_at: __field8,
                                created_at: __field9,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "guild_id",
                        "name",
                        "description",
                        "notifications",
                        "color",
                        "is_all_day",
                        "start_at",
                        "end_at",
                        "created_at",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Events",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Events>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Events {
            pub async fn get(pool: &::sqlx::PgPool, id: i32) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("SELECT * FROM events WHERE id = $1" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (2usize) ? ; let sqlx_query_as_description = row . try_get_unchecked :: < :: std :: option :: Option < String > , _ > (3usize) ? ; let sqlx_query_as_notifications = row . try_get_unchecked :: < Vec < String > , _ > (4usize) ? ; let sqlx_query_as_color = row . try_get_unchecked :: < String , _ > (5usize) ? ; let sqlx_query_as_is_all_day = row . try_get_unchecked :: < bool , _ > (6usize) ? ; let sqlx_query_as_start_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (7usize) ? ; let sqlx_query_as_end_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (8usize) ? ; let sqlx_query_as_created_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (9usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , name : sqlx_query_as_name , description : sqlx_query_as_description , notifications : sqlx_query_as_notifications , color : sqlx_query_as_color , is_all_day : sqlx_query_as_is_all_day , start_at : sqlx_query_as_start_at , end_at : sqlx_query_as_end_at , created_at : sqlx_query_as_created_at , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
        impl Events {
            #[allow(clippy::too_many_arguments)]
            pub async fn create(
                pool: &::sqlx::PgPool,
                guild_id: &String,
                name: &String,
                description: &Option<String>,
                notifications: &Vec<String>,
                color: &String,
                is_all_day: &bool,
                start_at: &NaiveDateTime,
                end_at: &NaiveDateTime,
                created_at: &NaiveDateTime,
            ) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (guild_id) ; let arg1 = & (name) ; let arg2 = & (description) ; let arg3 = & (notifications) ; let arg4 = & (color) ; let arg5 = & (is_all_day) ; let arg6 = & (start_at) ; let arg7 = & (end_at) ; let arg8 = & (created_at) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg1) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg2) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg3) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & [String] , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg4) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg5) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < bool , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg6) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < sqlx :: types :: chrono :: NaiveDateTime , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg7) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < sqlx :: types :: chrono :: NaiveDateTime , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg8) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < sqlx :: types :: chrono :: NaiveDateTime , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (9usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg1) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg2) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg3) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg4) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg5) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg6) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg7) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg8)) ; query_args . add (arg0) ; query_args . add (arg1) ; query_args . add (arg2) ; query_args . add (arg3) ; query_args . add (arg4) ; query_args . add (arg5) ; query_args . add (arg6) ; query_args . add (arg7) ; query_args . add (arg8) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("INSERT INTO events (guild_id,name,description,notifications,color,is_all_day,start_at,end_at,created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (2usize) ? ; let sqlx_query_as_description = row . try_get_unchecked :: < :: std :: option :: Option < String > , _ > (3usize) ? ; let sqlx_query_as_notifications = row . try_get_unchecked :: < Vec < String > , _ > (4usize) ? ; let sqlx_query_as_color = row . try_get_unchecked :: < String , _ > (5usize) ? ; let sqlx_query_as_is_all_day = row . try_get_unchecked :: < bool , _ > (6usize) ? ; let sqlx_query_as_start_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (7usize) ? ; let sqlx_query_as_end_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (8usize) ? ; let sqlx_query_as_created_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (9usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , name : sqlx_query_as_name , description : sqlx_query_as_description , notifications : sqlx_query_as_notifications , color : sqlx_query_as_color , is_all_day : sqlx_query_as_is_all_day , start_at : sqlx_query_as_start_at , end_at : sqlx_query_as_end_at , created_at : sqlx_query_as_created_at , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
        impl Events {
            pub async fn delete(&self, pool: &::sqlx::PgPool) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (self . id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("DELETE FROM events WHERE id = $1 RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (2usize) ? ; let sqlx_query_as_description = row . try_get_unchecked :: < :: std :: option :: Option < String > , _ > (3usize) ? ; let sqlx_query_as_notifications = row . try_get_unchecked :: < Vec < String > , _ > (4usize) ? ; let sqlx_query_as_color = row . try_get_unchecked :: < String , _ > (5usize) ? ; let sqlx_query_as_is_all_day = row . try_get_unchecked :: < bool , _ > (6usize) ? ; let sqlx_query_as_start_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (7usize) ? ; let sqlx_query_as_end_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (8usize) ? ; let sqlx_query_as_created_at = row . try_get_unchecked :: < sqlx :: types :: chrono :: NaiveDateTime , _ > (9usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , name : sqlx_query_as_name , description : sqlx_query_as_description , notifications : sqlx_query_as_notifications , color : sqlx_query_as_color , is_all_day : sqlx_query_as_is_all_day , start_at : sqlx_query_as_start_at , end_at : sqlx_query_as_end_at , created_at : sqlx_query_as_created_at , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
    }
    pub mod guilds {
        use sqlx_macros::*;
        #[table(name = "guilds")]
        pub struct Guilds {
            #[table(pk, skip)]
            pub id: i32,
            pub guild_id: String,
            pub name: String,
            pub avatar_url: Option<String>,
            pub locale: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Guilds {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Guilds {
                        id: ref __self_0_0,
                        guild_id: ref __self_0_1,
                        name: ref __self_0_2,
                        avatar_url: ref __self_0_3,
                        locale: ref __self_0_4,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Guilds");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "guild_id",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "name",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "avatar_url",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "locale",
                            &&(*__self_0_4),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Guilds {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Guilds",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "guild_id",
                        &self.guild_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "avatar_url",
                        &self.avatar_url,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "locale",
                        &self.locale,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Guilds {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "guild_id" => _serde::__private::Ok(__Field::__field1),
                                "name" => _serde::__private::Ok(__Field::__field2),
                                "avatar_url" => _serde::__private::Ok(__Field::__field3),
                                "locale" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"guild_id" => _serde::__private::Ok(__Field::__field1),
                                b"name" => _serde::__private::Ok(__Field::__field2),
                                b"avatar_url" => _serde::__private::Ok(__Field::__field3),
                                b"locale" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Guilds>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Guilds;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Guilds")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Guilds with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Guilds with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Guilds with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Guilds with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Guilds with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Guilds {
                                id: __field0,
                                guild_id: __field1,
                                name: __field2,
                                avatar_url: __field3,
                                locale: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<i32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "guild_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "avatar_url",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "locale",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("guild_id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("avatar_url") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("locale") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Guilds {
                                id: __field0,
                                guild_id: __field1,
                                name: __field2,
                                avatar_url: __field3,
                                locale: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["id", "guild_id", "name", "avatar_url", "locale"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Guilds",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Guilds>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Guilds {
            pub async fn get(pool: &::sqlx::PgPool, id: i32) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("SELECT * FROM guilds WHERE id = $1" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (2usize) ? ; let sqlx_query_as_avatar_url = row . try_get_unchecked :: < :: std :: option :: Option < String > , _ > (3usize) ? ; let sqlx_query_as_locale = row . try_get_unchecked :: < String , _ > (4usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , name : sqlx_query_as_name , avatar_url : sqlx_query_as_avatar_url , locale : sqlx_query_as_locale , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
        impl Guilds {
            #[allow(clippy::too_many_arguments)]
            pub async fn create(
                pool: &::sqlx::PgPool,
                guild_id: &String,
                name: &String,
                avatar_url: &Option<String>,
                locale: &String,
            ) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (guild_id) ; let arg1 = & (name) ; let arg2 = & (avatar_url) ; let arg3 = & (locale) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg1) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg2) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg3) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < & str , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (4usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg1) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg2) + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg3)) ; query_args . add (arg0) ; query_args . add (arg1) ; query_args . add (arg2) ; query_args . add (arg3) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("INSERT INTO guilds (guild_id,name,avatar_url,locale) VALUES ($1,$2,$3,$4) RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (2usize) ? ; let sqlx_query_as_avatar_url = row . try_get_unchecked :: < :: std :: option :: Option < String > , _ > (3usize) ? ; let sqlx_query_as_locale = row . try_get_unchecked :: < String , _ > (4usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , name : sqlx_query_as_name , avatar_url : sqlx_query_as_avatar_url , locale : sqlx_query_as_locale , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
        impl Guilds {
            pub async fn delete(&self, pool: &::sqlx::PgPool) -> Result<Self, ::sqlx::Error> {
                let data = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (self . id) ; if false { use :: sqlx :: ty_match :: { WrapSameExt as _ , MatchBorrowExt as _ } ; let expr = :: sqlx :: ty_match :: dupe_value (arg0) ; let ty_check = :: sqlx :: ty_match :: WrapSame :: < i32 , _ > :: new (& expr) . wrap_same () ; let (mut _ty_check , match_borrow) = :: sqlx :: ty_match :: MatchBorrow :: new (ty_check , & expr) ; _ty_check = match_borrow . match_borrow () ; { :: std :: rt :: begin_panic ("explicit panic") } ; } let mut query_args = < sqlx :: postgres :: Postgres as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: postgres :: Postgres > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: postgres :: Postgres , _ > ("DELETE FROM guilds WHERE id = $1 RETURNING *" , query_args) . try_map (| row : sqlx :: postgres :: PgRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i32 , _ > (0usize) ? ; let sqlx_query_as_guild_id = row . try_get_unchecked :: < String , _ > (1usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (2usize) ? ; let sqlx_query_as_avatar_url = row . try_get_unchecked :: < :: std :: option :: Option < String > , _ > (3usize) ? ; let sqlx_query_as_locale = row . try_get_unchecked :: < String , _ > (4usize) ? ; Ok (Self { id : sqlx_query_as_id , guild_id : sqlx_query_as_guild_id , name : sqlx_query_as_name , avatar_url : sqlx_query_as_avatar_url , locale : sqlx_query_as_locale , }) }) } } } . fetch_one (pool) . await ? ;
                Ok(data)
            }
        }
    }
}
mod routes {
    use actix_web::web;
    use anyhow::Context;
    mod events {
        mod get {
            use super::check_token;
            use crate::{
                client::DiscordClient,
                error::ApiError,
                models::{prelude::*, *},
                routes::{get_client, get_connection},
            };
            use actix_session::Session;
            use actix_web::{get, web, HttpRequest, HttpResponse};
            use chrono::Duration;
            pub struct EventQuery {
                start_at: chrono::NaiveDateTime,
                date_type: String,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for EventQuery {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "start_at" => _serde::__private::Ok(__Field::__field0),
                                    "date_type" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"start_at" => _serde::__private::Ok(__Field::__field0),
                                    b"date_type" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<EventQuery>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = EventQuery;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct EventQuery",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<
                                    chrono::NaiveDateTime,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct EventQuery with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct EventQuery with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(EventQuery {
                                    start_at: __field0,
                                    date_type: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<chrono::NaiveDateTime> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("start_at")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    chrono::NaiveDateTime,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("date_type")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("start_at") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("date_type") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(EventQuery {
                                    start_at: __field0,
                                    date_type: __field1,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["start_at", "date_type"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "EventQuery",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<EventQuery>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[allow(non_camel_case_types, missing_docs)]
            pub struct get_events;
            impl actix_web::dev::HttpServiceFactory for get_events {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    async fn get_events(
                        session: Session,
                        guild_id: web::Path<String>,
                        query: web::Query<EventQuery>,
                        req: HttpRequest,
                    ) -> Result<HttpResponse, ApiError> {
                        let id = guild_id.into_inner();
                        let query = query.into_inner();
                        let start_at = query.start_at + Duration::days(-6);
                        let date_type = query.date_type;
                        let end_at = match date_type.as_str() {
                            "month" => start_at + Duration::days(43),
                            "week" => start_at + Duration::days(19),
                            "4day" => start_at + Duration::days(16),
                            "day" => start_at + Duration::days(13),
                            _ => start_at + Duration::days(43),
                        };
                        let client = get_client(&req)?;
                        let token = client.get_user_token(&req)?;
                        let pool = get_connection(&req)?;
                        check_token(&session, &id, &token, client).await?;
                        Ok(HttpResponse::Ok().json(events))
                    }
                    let __resource = actix_web::Resource::new("/{guild_id}")
                        .name("get_events")
                        .guard(actix_web::guard::Get())
                        .to(get_events);
                    actix_web::dev::HttpServiceFactory::register(__resource, __config)
                }
            }
        }
        mod create {
            use super::check_token;
            use crate::{
                client::DiscordClient,
                error::ApiError,
                models::*,
                routes::{get_client, get_connection},
            };
            use actix_session::Session;
            use actix_web::{web, HttpRequest, HttpResponse};
            #[allow(non_camel_case_types, missing_docs)]
            pub struct create_event;
            impl actix_web::dev::HttpServiceFactory for create_event {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    async fn create_event(
                        session: Session,
                        guild_id: web::Path<String>,
                        event: web::Json<EventCreate>,
                        req: HttpRequest,
                    ) -> Result<HttpResponse, ApiError> {
                        let id = guild_id.into_inner();
                        let event = event.into_inner();
                        if id != event.guild_id {
                            return Err(ApiError::Unauthorized);
                        }
                        let client = get_client(&req)?;
                        let token = client.get_user_token(&req)?;
                        let pool = get_connection(&req)?;
                        check_token(&session, &id, &token, client).await?;
                        Ok(HttpResponse::Ok().json(event.create(pool).await?))
                    }
                    let __resource = actix_web::Resource::new("/{guild_id}")
                        .name("create_event")
                        .guard(actix_web::guard::Post())
                        .to(create_event);
                    actix_web::dev::HttpServiceFactory::register(__resource, __config)
                }
            }
        }
        mod update {
            use super::check_token;
            use crate::{
                client::DiscordClient,
                error::ApiError,
                models::*,
                routes::{get_client, get_connection},
            };
            use actix_session::Session;
            use actix_web::{web, HttpRequest, HttpResponse};
            struct EventPath {
                guild_id: String,
                event_id: i32,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for EventPath {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "guild_id" => _serde::__private::Ok(__Field::__field0),
                                    "event_id" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"guild_id" => _serde::__private::Ok(__Field::__field0),
                                    b"event_id" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<EventPath>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = EventPath;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct EventPath",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct EventPath with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<i32>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct EventPath with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(EventPath {
                                    guild_id: __field0,
                                    event_id: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("guild_id")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("event_id")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("guild_id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("event_id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(EventPath {
                                    guild_id: __field0,
                                    event_id: __field1,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["guild_id", "event_id"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "EventPath",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<EventPath>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[allow(non_camel_case_types, missing_docs)]
            pub struct update_event;
            impl actix_web::dev::HttpServiceFactory for update_event {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    async fn update_event(
                        session: Session,
                        path: web::Path<EventPath>,
                        event: web::Json<EventUpdate>,
                        req: HttpRequest,
                    ) -> Result<HttpResponse, ApiError> {
                        let path = path.into_inner();
                        let gid = path.guild_id;
                        let eid = path.event_id;
                        let event = event.into_inner();
                        let client = get_client(&req)?;
                        let token = client.get_user_token(&req)?;
                        let pool = get_connection(&req)?;
                        check_token(&session, &gid, &token, client).await?;
                        event.update(eid, pool).await?;
                        Ok(HttpResponse::NoContent().finish())
                    }
                    let __resource = actix_web::Resource::new("/{guild_id}/{event_id}")
                        .name("update_event")
                        .guard(actix_web::guard::Put())
                        .to(update_event);
                    actix_web::dev::HttpServiceFactory::register(__resource, __config)
                }
            }
        }
        use crate::{error::ApiError, client::DiscordClient};
        use actix_web::web;
        use actix_session::Session;
        pub fn check_session(session: &Session, token: &str) -> Result<bool, ApiError> {
            if let Some(session_token) = session.get::<String>("token")? {
                if session_token == token {
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
        pub async fn check_token(
            session: &Session,
            id: &str,
            token: &str,
            client: &reqwest::Client,
        ) -> Result<(), ApiError> {
            if check_session(session, token)? {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["Session Access"],
                                &match () {
                                    () => [],
                                },
                            ),
                            lvl,
                            &(
                                "api::routes::events",
                                "api::routes::events",
                                "src/routes/events/mod.rs",
                                28u32,
                            ),
                        );
                    }
                };
            } else {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["Register New Session"],
                                &match () {
                                    () => [],
                                },
                            ),
                            lvl,
                            &(
                                "api::routes::events",
                                "api::routes::events",
                                "src/routes/events/mod.rs",
                                30u32,
                            ),
                        );
                    }
                };
                client.check_member(id, token).await?;
                session.insert("token", token)?;
            }
            Ok(())
        }
        pub fn init_routes(cfg: &mut web::ServiceConfig) {
            cfg.service(get::get_events);
            cfg.service(create::create_event);
            cfg.service(update::update_event);
        }
    }
    mod guilds {
        mod id {
            use crate::{error::ApiError, routes::get_client, client::DiscordClient};
            use actix_web::{get, web, HttpResponse, HttpRequest};
            #[allow(non_camel_case_types, missing_docs)]
            pub struct get_guild;
            impl actix_web::dev::HttpServiceFactory for get_guild {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    async fn get_guild(
                        guild_id: web::Path<String>,
                        req: HttpRequest,
                    ) -> Result<HttpResponse, ApiError> {
                        let client = get_client(&req)?;
                        Ok(
                            HttpResponse::Ok()
                                .json(client.get_guild(&guild_id.into_inner()).await?),
                        )
                    }
                    let __resource = actix_web::Resource::new("/{guild_id}")
                        .name("get_guild")
                        .guard(actix_web::guard::Get())
                        .to(get_guild);
                    actix_web::dev::HttpServiceFactory::register(__resource, __config)
                }
            }
            #[allow(non_camel_case_types, missing_docs)]
            pub struct check_joined;
            impl actix_web::dev::HttpServiceFactory for check_joined {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    async fn check_joined(
                        guild_id: web::Path<String>,
                        req: HttpRequest,
                    ) -> Result<HttpResponse, ApiError> {
                        let client = get_client(&req)?;
                        let token = client.get_user_token(&req)?;
                        Ok(HttpResponse::Ok()
                            .json(client.check_member(&guild_id.into_inner(), &token).await?))
                    }
                    let __resource = actix_web::Resource::new("/check/{guild_id}")
                        .name("check_joined")
                        .guard(actix_web::guard::Get())
                        .to(check_joined);
                    actix_web::dev::HttpServiceFactory::register(__resource, __config)
                }
            }
        }
        mod joined {
            use actix_web::{HttpRequest, HttpResponse, web};
            use crate::{error::ApiError, models::*, routes::get_connection};
            pub struct GuildIds {
                guild_ids: String,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GuildIds {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "guild_ids" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"guild_ids" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GuildIds>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GuildIds;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GuildIds",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct GuildIds with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(GuildIds {
                                    guild_ids: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("guild_ids")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("guild_ids") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(GuildIds {
                                    guild_ids: __field0,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["guild_ids"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GuildIds",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GuildIds>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[allow(non_camel_case_types, missing_docs)]
            pub struct get_joined_guild;
            impl actix_web::dev::HttpServiceFactory for get_joined_guild {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    async fn get_joined_guild(
                        req: HttpRequest,
                        q: web::Query<GuildIds>,
                    ) -> Result<HttpResponse, ApiError> {
                        let guild_ids: Vec<String> = q
                            .into_inner()
                            .guild_ids
                            .split(',')
                            .map(|s| s.to_string())
                            .collect();
                        let pool = get_connection(&req)?;
                        Ok(HttpResponse::Ok().json(guilds))
                    }
                    let __resource = actix_web::Resource::new("/joined")
                        .name("get_joined_guild")
                        .guard(actix_web::guard::Get())
                        .to(get_joined_guild);
                    actix_web::dev::HttpServiceFactory::register(__resource, __config)
                }
            }
        }
        use actix_web::web;
        pub fn init_routes(cfg: &mut web::ServiceConfig) {
            cfg.service(joined::get_joined_guild);
            cfg.service(id::check_joined);
            cfg.service(id::get_guild);
        }
    }
    pub fn init_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/guilds").configure(guilds::init_routes));
        cfg.service(web::scope("/events").configure(events::init_routes));
    }
    pub fn get_connection(req: &actix_web::HttpRequest) -> Result<&sqlx::PgPool, anyhow::Error> {
        req.app_data().context("Database Connection Failed")
    }
    pub fn get_client(req: &actix_web::HttpRequest) -> Result<&reqwest::Client, anyhow::Error> {
        req.app_data().context("Client creation failed")
    }
}
use actix_session::CookieSession;
use actix_web::{cookie::SameSite, middleware::Logger, HttpServer};
use anyhow::Context;
use error::ApiError;
#[allow(non_camel_case_types, missing_docs)]
pub struct index;
impl actix_web::dev::HttpServiceFactory for index {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn index() -> String {
            String::from("DisCalendar API v2.0")
        }
        let __resource = actix_web::Resource::new("/")
            .name("index")
            .guard(actix_web::guard::Get())
            .to(index);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
fn main() -> Result<(), ApiError> {
    <::actix_web::rt::System>::new().block_on(async move {
        {
            dotenv::dotenv().ok();
            env_logger::builder()
                .filter_level(log::LevelFilter::Debug)
                .is_test(true)
                .init();
            let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
            let port = std::env::var("APP_PORT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse::<u16>()
                .unwrap();
            let addr = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ":"],
                    &match (&host, &port) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            };
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = sqlx::PgPool::connect(&database_url)
                .await
                .context("Failed to connect to database")?;
            let bot_token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");
            let client = client::create_client(bot_token).expect("Failed to get client");
            let server = HttpServer::new(move || {
                actix_web::App::new()
                    .app_data(pool.clone())
                    .app_data(client.clone())
                    .wrap(Logger::default())
                    .wrap(
                        CookieSession::signed(&[0; 32])
                            .secure(false)
                            .same_site(SameSite::Lax),
                    )
                    .configure(routes::init_routes)
                    .service(index)
            })
            .bind(&addr)
            .context({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to bind "],
                    &match (&addr,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            })
            .unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Start serving at "],
                            &match (&addr,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ),
                        lvl,
                        &("api", "api", "src/main.rs", 66u32),
                    );
                }
            };
            server.run().await.context("Failed to run server")?;
            Ok(())
        }
    })
}

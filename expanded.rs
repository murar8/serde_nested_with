#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde::Serialize;
use serde_test::{assert_tokens, Token};
use serde_with_nested::serde_nested_with;
use std::collections::BTreeMap;
use time::serde::rfc3339;
use time::OffsetDateTime;
mod __serde_nested_with_1207106974161097716 {
    use super::*;
    use serde::{Serialize, Deserialize};
    #[serde(transparent)]
    struct Wrapper(#[serde(serialize_with = "rfc3339::serialize")] OffsetDateTime);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Wrapper {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                rfc3339::serialize(&self.0, __serializer)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Wrapper {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                _serde::__private::Result::map(
                    _serde::Deserialize::deserialize(__deserializer),
                    |__transparent| Wrapper { 0: __transparent },
                )
            }
        }
    };
    pub fn serialize<S: serde::Serializer>(
        val: &Option<OffsetDateTime>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        let val: &Option<Wrapper> = unsafe { std::mem::transmute(val) };
        val.serialize(serializer)
    }
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> std::result::Result<Option<OffsetDateTime>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = Option::<Wrapper>::deserialize(deserializer)?;
        Ok(unsafe { std::mem::transmute(v) })
    }
}
pub struct Foo {
    #[serde(serialize_with = "rfc3339::serialize")]
    pub bar0: OffsetDateTime,
    #[serde(serialize_with = "__serde_nested_with_1207106974161097716")]
    pub bar1: Option<OffsetDateTime>,
    pub bar3: Option<BTreeMap<i32, OffsetDateTime>>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Foo {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Foo",
            "bar0",
            &self.bar0,
            "bar1",
            &self.bar1,
            "bar3",
            &&self.bar3,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Foo {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Foo {
    #[inline]
    fn eq(&self, other: &Foo) -> bool {
        self.bar0 == other.bar0 && self.bar1 == other.bar1 && self.bar3 == other.bar3
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Foo {}
#[automatically_derived]
impl ::core::cmp::Eq for Foo {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<OffsetDateTime>;
        let _: ::core::cmp::AssertParamIsEq<Option<OffsetDateTime>>;
        let _: ::core::cmp::AssertParamIsEq<Option<BTreeMap<i32, OffsetDateTime>>>;
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Foo {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Foo",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "bar0",
                {
                    #[doc(hidden)]
                    struct __SerializeWith<'__a> {
                        values: (&'__a OffsetDateTime,),
                        phantom: _serde::__private::PhantomData<Foo>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            rfc3339::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.bar0,),
                        phantom: _serde::__private::PhantomData::<Foo>,
                    }
                },
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "bar1",
                {
                    #[doc(hidden)]
                    struct __SerializeWith<'__a> {
                        values: (&'__a Option<OffsetDateTime>,),
                        phantom: _serde::__private::PhantomData<Foo>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            __serde_nested_with_1207106974161097716(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.bar1,),
                        phantom: _serde::__private::PhantomData::<Foo>,
                    }
                },
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "bar3",
                &self.bar3,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}

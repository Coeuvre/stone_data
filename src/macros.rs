macro_rules! model {
    (
        $($T:ident {
            type: $ty:tt,
            attributes: {
                $($attribute_name:tt : $attribute_type:ident,)*
            },
            relationships: {
                $($relationship_name:tt : $relationship_type:ident<$relationship:ident>,)*
            },
        }),*
    ) => {
        model! {
            $($T {
                type: $ty,
                primary_key: "id",
                attributes: {
                    $($attribute_name: $attribute_type,)*
                },
                relationships: {
                    $($relationship_name: $relationship_type<$relationship>,)*
                },
            }),*
        }
    };

    (
        $($T:ident {
            type: $ty:tt,
            primary_key: $primary_key:tt,
            attributes: {
                $($attribute_name:tt : $attribute_type:ident,)*
            },
            relationships: {
                $($relationship_name:tt : $relationship_type:ident<$relationship:ident>,)*
            },
        }),*
    ) => {
        lazy_static! {
            $(
                pub static ref $T: $crate::model::Model = $crate::model::model($ty, $primary_key), |m| {
                    $(
                        m.attributes.insert($attribute_name, $crate::attribute::AttributeType::$attribute_type);
                    )*

                    $(
                        m.relationships.insert($relationship_name, $crate::relationship::RelationshipType::$relationship_type($relationship.get_ref()));
                    )*
                };
            )*
        }

        $(
            impl AsRef<$crate::model::Model> for $T {
                fn as_ref(&self) -> &$crate::model::Model {
                    &*self
                }
            }
        )*
    };
}

macro_rules! lazy_static {
    (static ref $N:ident : $T:ty = $alloc:expr; $($t:tt)*) => {
        lazy_static!(PRIV static ref $N : $T = $alloc, |__| {}; $($t)*);
    };
    (pub static ref $N:ident : $T:ty = $alloc:expr; $($t:tt)*) => {
        lazy_static!(PUB static ref $N : $T = $alloc, |__| {}; $($t)*);
    };
    (pub static ref $N:ident : $T:ty = $alloc:expr, |$S:ident| $init:expr; $($t:tt)*) => {
        lazy_static!(PUB static ref $N : $T = $alloc, |$S| $init; $($t)*);
    };
    ($VIS:ident static ref $N:ident : $T:ty = $alloc:expr,|$S:ident| $init:expr; $($t:tt)*) => {
        lazy_static!(MAKE TY $VIS $N);

        impl $N {
            #[allow(dead_code)]
            #[inline(always)]
            fn get_ref(&self) -> &$T {
                unsafe { &*self.get_raw_pointer() }
            }

            #[inline(always)]
            fn get_raw_pointer(&self) -> *const $T {
                static mut DATA: *const $T = 0 as *const $T;

                #[inline(always)]
                fn __static_ref_allocate() -> Box<$T> { Box::new($alloc) }

                unsafe {
                    use std::sync::{Once, ONCE_INIT};
                    use std::mem::transmute;

                    static mut ONCE: Once = ONCE_INIT;
                    ONCE.call_once(|| {
                        DATA = transmute::<Box<$T>, *const $T>(__static_ref_allocate());
                    });

                    DATA
                }
            }

            #[inline(always)]
            unsafe fn initialize(&self) -> *const $T {
                #[inline(always)]
                fn __static_ref_initialize($S: &mut $T) { $init; }

                use std::sync::{Once, ONCE_INIT};
                use std::mem::transmute;

                let data = self.get_raw_pointer();

                static mut ONCE: Once = ONCE_INIT;
                ONCE.call_once(|| {
                    __static_ref_initialize(transmute(data));
                });

                data
            }
        }

        impl ::std::ops::Deref for $N {
            type Target = $T;
            fn deref(&self) -> &$T {
                unsafe {
                    #[inline(always)]
                    fn require_sync<T: Sync>(_: &T) { }

                    let data = self.initialize();

                    let static_ref = &*data;
                    require_sync(static_ref);
                    static_ref
                }
            }
        }

        lazy_static!($($t)*);
    };
    (MAKE TY PUB $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct $N {__private_field: ()}
        #[allow(non_upper_case_globals)]
        pub static $N: $N = $N {__private_field: ()};
    };
    (MAKE TY PRIV $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct $N {__private_field: ()}
        #[allow(non_upper_case_globals)]
        static $N: $N = $N {__private_field: ()};
    };
    () => ()
}

macro_rules! try_opt {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => return None,
    })
}

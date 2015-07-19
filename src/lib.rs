#![feature(optin_builtin_traits)]

extern crate anymap;

pub mod attr;
pub mod model;
pub mod transform;

// Error
//     RecordNotFound

//
// /*
// pub struct Record<M> {
//     val: M,
// }
//
// impl<M: Model> Record<M> {
//
// }
//
// pub trait Store {
//     fn all();
//
//     fn first();
//     fn last();
//     fn take();
//
//     fn find<M: Model>() -> Option<Record<M>>;
//     fn find_in();
//     fn find_by();
// }
// */
//

#![feature(convert)]

#[macro_use]
pub mod macros;

pub mod adapter;
pub mod attribute;
pub mod model;
pub mod query;
pub mod relationship;
// pub mod store;
pub mod serializer;
// pub mod transform;

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

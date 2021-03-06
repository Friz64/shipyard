//! Shipyard is an Entity Component System focused on usability and speed.
//!
//! The [user guide](https://leudz.github.io/shipyard/guide) is a great place to learn all about Shipyard!  
//!
//! ## Features
//!
//! - **panic** *(default)* adds panicking functions
//! - **parallel** *(default)* &mdash; adds parallel iterators and dispatch
//! - **serde1** &mdash; adds (de)serialization support with [serde](https://github.com/serde-rs/serde)
//! - **non_send** &mdash; add methods and types required to work with `!Send` components
//! - **non_sync** &mdash; add methods and types required to work with `!Sync` components
//! - **std** *(default)* &mdash; let shipyard use the standard library

#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_qualifications)]
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

mod atomic_refcell;
mod borrow;
mod delete;
pub mod error;
mod get;
mod not;
mod pack {
    pub(crate) mod update;
}
mod add_component;
mod add_entity;
mod contains;
pub mod iter;
mod r#mut;
mod remove;
mod reserve;
mod sparse_set;
mod storage;
mod system;
mod system_macro;
mod type_id;
mod unknown_storage;
mod view;
mod world;

#[cfg(feature = "non_send")]
pub use crate::borrow::NonSend;
#[cfg(all(feature = "non_send", feature = "non_sync"))]
pub use crate::borrow::NonSendSync;
#[cfg(feature = "non_sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "non_sync")))]
pub use crate::borrow::NonSync;
pub use add_component::AddComponent;
pub use add_entity::AddEntity;
pub use atomic_refcell::{ExclusiveBorrow, Ref, RefMut, SharedBorrow};
pub use borrow::{AllStoragesBorrow, Borrow, FakeBorrow, Mutability};
pub use contains::Contains;
pub use delete::Delete;
pub use get::Get;
pub use iter::{IntoFastIter, IntoIter, IntoWithId};
pub use not::Not;
pub use pack::update::{Inserted, InsertedOrModified, Modified};
pub use r#mut::Mut;
pub use remove::Remove;
pub use reserve::{BulkEntityIter, BulkReserve};
pub use sparse_set::{sort, sort::IntoSortable, SparseSet};
pub use storage::{
    AllStorages, CustomDeleteAny, DeleteAny, Entities, EntityId, Retain, StorageId, Unique,
};
#[doc(hidden)]
pub use system::{AllSystem, Nothing, System};
pub use unknown_storage::UnknownStorage;
pub use view::{
    AllStoragesViewMut, EntitiesView, EntitiesViewMut, UniqueView, UniqueViewMut, View, ViewMut,
};
pub use world::scheduler::info;
pub use world::{Workload, WorkloadBuilder, World};

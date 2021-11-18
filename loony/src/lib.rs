//! loony - framework for composable network services
//!
//! ## Package feature
//!
//! * `openssl` - enables ssl support via `openssl` crate
//! * `rustls` - enables ssl support via `rustls` crate
//! * `compress` - enables compression support in http and web modules
//! * `cookie` - enables cookie support in http and web modules

#![warn(
    rust_2018_idioms,
    unreachable_pub,
    // missing_debug_implementations,
    // missing_docs,
)]
#![allow(
    type_alias_bounds,
    clippy::type_complexity,
    clippy::borrow_interior_mutable_const,
    clippy::needless_doctest_main,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;

#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub use loony_macros::{rt_main as main, rt_test as test};

#[cfg(test)]
pub(crate) use loony_macros::rt_test2 as rt_test;

pub mod channel;
pub mod connect;
pub mod framed;
#[cfg(feature = "http-framework")]
pub mod http;
pub mod server;
pub mod testing;
pub mod time;
pub mod util;
#[cfg(feature = "http-framework")]
pub mod web;
pub mod ws;
#[cfg(feature = "cookies")]
pub use cookie;

pub use self::service::{
    apply_fn, boxed, fn_factory, fn_factory_with_config, fn_service, into_service,
    pipeline, pipeline_factory, IntoService, IntoServiceFactory, Service,
    ServiceFactory, Transform,
};

pub use futures_core::stream::Stream;
pub use futures_sink::Sink;
pub use loony_util::task;

pub mod codec {
    //! Utilities for encoding and decoding frames.
    pub use loony_codec::*;
}

pub mod router {
    //! Resource path matching library.
    pub use loony_router::*;
}

pub mod rt {
    //! A runtime implementation that runs everything on the current thread.
    pub use loony_rt::*;
}

pub mod service {
    pub use loony_service::*;
}

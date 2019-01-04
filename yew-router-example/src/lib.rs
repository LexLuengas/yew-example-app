//! Yew router

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate stdweb;
extern crate yew;

#[macro_use]
extern crate log;

#[macro_use]
pub mod macros;
pub mod router;
pub mod routing_service;

pub use router::{Request, Route, Router};
pub use routing_service::RouterService;
#![recursion_limit = "512"]

#[macro_use]
extern crate log;
extern crate web_logger;
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate yew_router;
extern crate stdweb;

// Module structure
mod components;
mod routes;

// Expose Model from app component
pub use crate::components::root::Model;
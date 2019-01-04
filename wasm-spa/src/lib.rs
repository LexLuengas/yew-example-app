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
extern crate util;

// Module structure
mod components;

// Expose Model from app component
pub use crate::components::root::Model;
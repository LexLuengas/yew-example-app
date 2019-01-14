#[macro_use]
extern crate log;
#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate yew_router;
extern crate url;

extern crate header;
extern crate header_input;
extern crate tweets;
extern crate common;
extern crate identifiers;

mod root;
mod instructions;

pub use self::root::Model;
//! Contains all of the uuid NewTypes.
//! The Uuid type is newtyped to prevent programmer error when passing identifiers around.
//! Conditionally, this crate can be compiled with rocket support,
//! allowing the NewTypes to be extracted from query params and paths.

extern crate uuid;

#[macro_use]
extern crate serde_derive;

pub mod user;
pub mod keyword;

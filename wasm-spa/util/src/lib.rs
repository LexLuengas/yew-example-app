#[macro_use]
extern crate log;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate stdweb;
extern crate base64;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate yew;
extern crate yew_router;

extern crate identifiers;
extern crate wire;

pub mod user;
pub mod fetch;
pub mod notification;
pub mod directional;
pub mod loadable;
pub mod loading;
pub mod uploadable;
pub mod wrappers;
pub mod input;
pub mod button;
pub mod link;
pub mod table;

#[macro_export]
macro_rules! column {
    ($a:expr) => {{
        $crate::table::Column {
            name: $a.to_string(),
            short_name: Some($a.to_string()),
            data_property: Some($a.to_string()),
        }
    }};
    ($a:expr, $b:expr) => {{
        $crate::table::Column {
            name: $b.to_string(),
            short_name: Some($b.to_string()),
            data_property: Some($a.to_string()),
        }
    }};
    ($a:expr, $b:expr, $c:expr) => {
        $crate::table::Column {
            name: $b.to_string(),
            short_name: Some($c.to_string()),
            data_property: Some($a.to_string()),
        }
    }
}

#[macro_export]
macro_rules! columns {
    ( $( ( $($args:expr),* ) )+ ) => {
        vec![$(
            column![$($args),*]
        ),+];
    };
}
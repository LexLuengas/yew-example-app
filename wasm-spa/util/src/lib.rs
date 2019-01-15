#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;
extern crate pulldown_cmark;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_value;
extern crate common;
#[macro_use]
extern crate smart_default;

pub mod loadable;
pub mod loading;
pub mod uploadable;
pub mod wrappers;
pub mod input;
pub mod markdown;
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
    ($a:expr; $b:expr) => {{
        $crate::table::Column {
            name: $b.to_string(),
            short_name: $b.to_string(),
            data_property: $a.to_string(),
        }
    }};
    ($a:expr; $b:expr, $c:expr) => {
        $crate::table::Column {
            name: $b.to_string(),
            short_name: $c.to_string(),
            data_property: $a.to_string(),
        }
    }
}

#[macro_export]
macro_rules! columns {
    ( $( ($($args:expr),*) )+ ) => {
        vec![$(
            column![$($args);*],
        ),+];
    };
}
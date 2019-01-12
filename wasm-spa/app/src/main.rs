#![recursion_limit="128"]

#[macro_use]
extern crate log;
extern crate web_logger;
extern crate yew;
extern crate console_error_panic_hook;

extern crate root;

use root::Model;
use yew::prelude::App;


fn main() {
    console_error_panic_hook::set_once();
    web_logger::init();
    info!("Starting SPA");
    
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

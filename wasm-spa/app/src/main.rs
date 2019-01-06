extern crate wasm_spa;
extern crate yew;

#[macro_use]
extern crate log;
extern crate web_logger;

use wasm_spa::Model;
use yew::prelude::App;

fn main() {
    web_logger::init();
    info!("Starting SPA");
    
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

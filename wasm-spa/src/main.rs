extern crate wasm_spa;
extern crate yew;

use wasm_spa::Model;
use yew::prelude::App;

fn main() {
    web_logger::init();
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

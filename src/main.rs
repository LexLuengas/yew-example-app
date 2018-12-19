extern crate yew;
extern crate wasm_spa;

use yew::prelude::App;
use wasm_spa::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

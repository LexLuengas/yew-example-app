extern crate wasm_spa;
extern crate yew;

use wasm_spa::Model;
use yew::prelude::App;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    #[allow(dead_code)]
    keywords: Vec<String>,
}

pub enum Msg {
    AddKeyword,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            keywords: vec![]
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <nav class="navbar is-light",>
                <div class="navbar-item",>
                    <div class="field has-addons",>
                        <p class="control",>
                            <input class="input", type="text", placeholder="New keyword",/>
                        </p>
                        <div class="control",>
                            <a class="button is-info",>
                            { "Add" }
                            </a>
                        </div>
                    </div>
                </div>
                <div class="navbar-item is-expanded",>
                    <span class="tag is-warning is-medium",>
                        {"Hello"}
                        <button class="delete is-small",></button>
                    </span>
                </div>
                </nav>
            </>
        }
    }
}

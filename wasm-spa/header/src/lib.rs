#[macro_use]
extern crate yew;
extern crate yew_router;
extern crate stdweb;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterLink;
use stdweb::web::{Element, IParentNode, INonElementParentNode, IElement, document};
use stdweb::unstable::TryInto;

pub struct Model {
    is_expanded: bool,
}

pub enum Msg {
    Toggle,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            is_expanded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                let navbar_burger = document()
                    .query_selector_all(".navbar-burger").unwrap()
                    .item(0).expect("No '.navbar-burger' DOM Element found");
                let navbar_burger_el: Element = navbar_burger.try_into().unwrap();
                let navbar_menu_id = navbar_burger_el.get_attribute("data-target").unwrap();
                let navbar_menu_el: Element = document().get_element_by_id(&navbar_menu_id).unwrap();
                if self.is_expanded {
                    navbar_burger_el.class_list().remove("is-active").unwrap();
                    navbar_menu_el.class_list().remove("is-active").unwrap();
                } else {
                    navbar_burger_el.class_list().add("is-active").unwrap();
                    navbar_menu_el.class_list().add("is-active").unwrap();
                }
                self.is_expanded = !self.is_expanded;
            }
        }
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <nav class="navbar is-dark is-fixed-top",>
                <div class="container",>
                    <div class="navbar-brand",>
                        <a class="navbar-item brand-text", href="../",>
                            {"Trend Analyser"}
                        </a>
                        <div onclick=|_| Msg::Toggle, class="navbar-burger burger", data-target="navMenu",>
                            <span></span>
                            <span></span>
                            <span></span>
                        </div>
                    </div>
                    <div id="navMenu", class="navbar-menu",>
                        <div class="navbar-start",>
                            <RouterLink: text=String::from("List"), route=Route::parse("/list"), class="navbar-item", />
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
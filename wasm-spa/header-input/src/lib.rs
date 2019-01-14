extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate common;
extern crate identifiers;
extern crate util;

use common::datatypes::keyword::Keyword;
use identifiers::keyword::KeywordUuid;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::html_element::InputElement;
use yew::prelude::*;

pub struct Model {
    keywords: Vec<Keyword>,
    onchange: Option<Callback<Msg>>,
}

#[derive(Debug)]
pub enum Msg {
    AddKeyword(String),
    RemoveKeyword { uuid: KeywordUuid },
    ClearKeywords,
}

#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub keywords: Vec<Keyword>,
    pub onchange: Option<Callback<Msg>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            keywords: props.keywords,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        if let Some(ref mut callback) = self.onchange {
            match msg {
                RemoveKeyword { uuid } => {
                    callback.emit(RemoveKeyword { uuid });
                }
                AddKeyword(_) => {
                    let keyword_input: InputElement = document()
                        .query_selector("#keyword-input")
                        .unwrap()
                        .unwrap()
                        .try_into()
                        .unwrap();
                    let keyword_value = keyword_input.raw_value();
                    keyword_input.set_raw_value("");
                    callback.emit(AddKeyword(keyword_value));
                }
                ClearKeywords => {
                    callback.emit(ClearKeywords);
                }
            }
            true
        } else {
            false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.keywords = props.keywords;
        self.onchange = props.onchange;
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <nav class="navbar is-light",>
                <div class="navbar-item",>
                    <div class="field is-grouped",>
                        <p class="control",>
                            <div class="field has-addons",>
                                <p class="control",>
                                    <input id="keyword-input", class="input", type="text", placeholder="New keyword",/>
                                </p>
                                <div class="control",>
                                    <a class="button is-info", onclick=|_| Msg::AddKeyword(String::new()),>
                                    { "Add" }
                                    </a>
                                </div>
                            </div>
                        </p>
                        { self.view_button_clear_all() }
                    </div>
                    
                </div>
                <div class="navbar-item is-expanded tags",>
                    { for self.keywords.iter().map(Keyword::view) }
                </div>
                </nav>
            </>
        }
    }
}

impl Renderable<Model> for Keyword {
    fn view(&self) -> Html<Model> {
        let uuid: KeywordUuid = self.uuid;
        html! {
            <span class="tag is-warning is-medium",>
                { &self.title }
                <button class="delete is-small", onclick=|_| Msg::RemoveKeyword{ uuid }, ></button>
            </span>
        }
    }
}

impl Model {
    fn view_button_clear_all(&self) -> Html<Model> {
        if self.keywords.len() > 0 {
            html! {
                // <div class="field",>
                    <p class="control",>
                        <a class="button is-outlined", onclick=|_| Msg::ClearKeywords,>
                            { "Clear all" }
                        </a>
                    </p>
                // </div>
            }
        } else {
            ::util::wrappers::empty_vdom_node()
        }
    }
}

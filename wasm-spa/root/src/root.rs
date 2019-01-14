use url::form_urlencoded::byte_serialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::instructions::Model as Instructions;
use common::datatypes::keyword::Keyword;
use header::Model as Header;
use header_input::{Model as HeaderInput, Msg as HeaderInputMsg};
use identifiers::keyword::KeywordUuid;
use tweets::TweetList;

pub struct Model {
    keywords: Vec<Keyword>,
    router: Box<Bridge<RouterAgent>>,
    current_route: Route,
}

#[derive(SmartDefault)]
pub enum Msg {
    AddKeyword(Keyword),
    ClearKeywords,
    RemoveKeyword(KeywordUuid),
    KeywordAction(HeaderInputMsg),
    HandleRoute(Route),
    #[default]
    NoOp,
}

impl From<HeaderInputMsg> for Msg {
    fn from(msg: HeaderInputMsg) -> Self {
        match msg {
            HeaderInputMsg::AddKeyword(s) => Msg::AddKeyword(Keyword {
                title: s.clone(),
                query: s,
                uuid: KeywordUuid::default(),
            }),
            HeaderInputMsg::ClearKeywords => Msg::ClearKeywords,
            HeaderInputMsg::RemoveKeyword { uuid } => Msg::RemoveKeyword(uuid),
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route| Msg::HandleRoute(route));
        let mut router = RouterAgent::bridge(callback);
        router.send(RouterRequest::GetCurrentRoute);

        Model {
            keywords: vec![],
            router,
            current_route: route!("/"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            AddKeyword(k) => {
                self.keywords.push(k);
                let mut new_route = self.update_route();
                if "/" == self.current_route.to_route_string().as_str() {
                    new_route.path_segments = vec!["list".into()];
                }
                self.router.send(RouterRequest::ChangeRoute(new_route));
            }
            ClearKeywords => {
                self.keywords.clear();
                self.router.send(RouterRequest::ReplaceRoute(route!("/")));
            }
            RemoveKeyword(uuid) => {
                // UUID is assumed to exist
                let index = self.keywords.iter().position(|i| i.uuid == uuid).unwrap();
                self.keywords.remove(index);
                if self.keywords.is_empty() {
                    self.router.send(RouterRequest::ReplaceRoute(route!("/")));
                } else {
                    let new_route = self.update_route();
                    self.router.send(RouterRequest::ChangeRoute(new_route));
                }
            }
            KeywordAction(header_msg) => {
                info!("Received keyword action message: {:?}", header_msg);
                self.update(Msg::from(header_msg));
            }
            HandleRoute(route) => {
                self.current_route = route;
            }
            NoOp => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Model {
    fn query_from_keywords(&self) -> String {
        let query = self
            .keywords
            .clone()
            .into_iter()
            .map(|k| k.query)
            .collect::<Vec<String>>()
            .join(" OR ");
        let urlencoded: String = byte_serialize(&query.as_bytes()).collect();
        format!("q={}", urlencoded)
    }

    fn update_route(&self) -> Route {
        Route {
            query: Some(self.query_from_keywords()),
            ..self.current_route.clone()
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Header: is_active=self.keywords.len() > 0,/>
                <HeaderInput: keywords=self.keywords.clone(), onchange=|msg: HeaderInputMsg| Msg::KeywordAction(msg), />
                <section class="section body-container",>
                    <div class="container",>
                        <YewRouter: routes=routes![Instructions, TweetList], page_not_found=Some(DefaultPage(routing_failed_page)), />
                    </div>
                </section>
            </>
        }
    }
}

fn routing_failed_page(_route: &Route) -> Html<YewRouter> {
    html! {
        <>
            {"The page you are looking for could not be found."}
        </>
    }
}

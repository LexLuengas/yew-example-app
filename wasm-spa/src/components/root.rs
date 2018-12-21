
use yew::prelude::*;
use yew_router::router::{self, Route};
use routes::RouterTarget;
use super::b::BModel;

pub struct Model {
    child: RouterTarget,
    router: Box<Bridge<router::Router<()>>>,
}

pub enum Msg {
    NavigateTo(RouterTarget),
    HandleRoute(Route<()>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        Model {
            child: RouterTarget::A,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(child) => {
                let path_segments = match child {
                    RouterTarget::A => vec!["a".into()],
                    RouterTarget::B => vec!["b".into()],
                    RouterTarget::Error => vec!["error".into()],
                };

                let route = router::Route {
                    path_segments,
                    query: None,
                    fragment: None,
                    state: (),
                };

                self.router.send(router::Request::ChangeRoute(route));
                false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                self.child = if let Some(first_segment) = route.path_segments.get(0) {
                    match first_segment.as_str() {
                        "a" => RouterTarget::A,
                        "b" => RouterTarget::B,
                        _ => RouterTarget::Error,
                    }
                } else {
                    RouterTarget::Error
                };

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::NavigateTo(RouterTarget::A),>{ "Go to A" }</button>
                    <button onclick=|_| Msg::NavigateTo(RouterTarget::B),>{ "Go to B" }</button>
                </nav>
                <div>
                    {self.child.view()}
                </div>
            </div>
        }
    }
}

impl Renderable<Model> for RouterTarget {
    fn view(&self) -> Html<Model> {
        match *self {
            RouterTarget::A => html! {
                <>
                    {"This corresponds to route 'a'"}
                </>
            },
            RouterTarget::B => html! {
                <>
                    {"This corresponds to route 'b'"}
                    <BModel: />
                </>
            },
            RouterTarget::Error => html! {
                <>
                    {"Invalid path"}
                </>
            },
        }
    }
}

use yew::prelude::*;
use yew_router::prelude::*;

pub struct TweetList {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Props;

pub enum Msg {}

impl Component for TweetList {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, mut _link: ComponentLink<Self>) -> Self {
        TweetList {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn destroy(&mut self) {
        warn!("Destroying tweet list")
    }
}

impl Renderable<TweetList> for TweetList {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                {"I am the Tweet list component"}
            </div>
        }
    }
}

impl Routable for TweetList {
    fn resolve_props(route: &Route) -> Option<Self::Properties> {
        let first_segment = route.path_segments.get(0).unwrap();
        if "list" == first_segment.as_str() || "" == first_segment.as_str() {
            Some(Props)
        } else {
            None
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        route.path_segments.get(0).is_some()
    }
}

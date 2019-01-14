use yew::prelude::*;
use yew_router::prelude::*;

pub struct Model;

#[derive(Clone, PartialEq, Default)]
pub struct Props;

impl Component for Model {
    type Message = ();
    type Properties = Props;

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="notification",>
                // <button class="delete"></button>
                { "Add a keyword to get some tweets." }
            </div>
        }
    }
}

impl Routable for Model {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        let first_segment = route.path_segments.get(0).unwrap();
        if "" == first_segment.as_str() {
            Some(Props)
        } else {
            None
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        route.path_segments.get(0).is_some()
    }
}

use yew::prelude::*;
use yew_router::prelude::*;

use tweets::TweetList;
use header::Model as Header;
use header_input::Model as HeaderInput;


pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Header: />
                <HeaderInput: />
                <section class="section body-container",>
                    <div class="container",>
                        <YewRouter: routes=routes![TweetList], page_not_found=Some(DefaultPage(routing_failed_page)), />
                    </div>
                </section>
            </>
        }
    }
}

fn routing_failed_page(route: &Route) -> Html<YewRouter> {
    html! {
        <>
            {"The page you are looking for could not be found."}
            <br/>
            {format!("Could not route to '{}'", route.to_route_string())}
        </>
    }
}
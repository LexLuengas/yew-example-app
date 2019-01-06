use yew::prelude::*;
use yew_router::prelude::*;

use common::datatypes::tweet::TweetData;
use common::fetch::{FetchResponse, Networking};
use components::twitter_requests::TwitterRequest;
use util::loadable::Loadable;
use util::button::Button;
use wire::tweet::TweetResponse;
use wire::tweet::TestResponse;

pub struct TweetList {
    tweets: Loadable<Vec<TweetData>>,
    networking: Networking,
    link: ComponentLink<TweetList>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Props;

pub enum Msg {
    HandleGetTweetListResponse(FetchResponse<Vec<TweetData>>),
    Test,
    NoOp,
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl Component for TweetList {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tweet_list = TweetList {
            tweets: Loadable::default(),
            networking: Networking::new(&link),
            link,
        };

        tweet_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            HandleGetTweetListResponse(response) => {
                self.tweets = Loadable::from_fetch_response(response);
                true
            }
            Test => {
                info!("Fetching something...");
                self.networking.fetch(
                    &TwitterRequest::Test,
                    |_r: FetchResponse<TestResponse>| {
                        info!("Responded");
                        Msg::NoOp
                    },
                    &self.link,
                );
                true
            }
            NoOp => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<TweetList> for TweetData {
    fn view(&self) -> Html<TweetList> {
        html! {
            <li class="tweet-list-element",>
                <div>
                    // <RouterLink: text=&self.title, route=route!("forum/{}",self.uuid), />
                </div>
                <div>
                    { &self.text }
                </div>
            </li>
        }
    }
}

impl Renderable<TweetList> for TweetList {
    fn view(&self) -> Html<Self> {
        // fn forum_list_fn(tweets: &Vec<TweetData>) -> Html<TweetList> {
        //     html! {
        //             <ul class=("tweet-list"),>
        //                 { for tweets.iter().map(TweetData::view) }
        //             </ul>
        //     }
        // };
        // html! {
        //     <div>
        //         <Button: title="Fetch!", onclick=|_| Msg::Test, />
        //         { self.tweets.default_view(forum_list_fn) }
        //     </div>
        // }
        html! {
            <div>
                { "Success!" }
                <Button: title="Fetch!", onclick=|_| Msg::Test, />
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

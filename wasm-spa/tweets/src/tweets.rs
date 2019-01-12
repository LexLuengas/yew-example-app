use yew::prelude::*;
use yew_router::prelude::*;

use crate::requests::TwitterRequest;
use common::{
    datatypes::tweet::TweetData,
    fetch::{FetchResponse, Networking}
};
use util::loadable::Loadable;
use wire::tweet::TwitterResponse;


pub struct TweetList {
    tweets: Loadable<Vec<TweetData>>,
    networking: Networking,
    link: ComponentLink<TweetList>,
}

#[derive(Clone, PartialEq, Default)]
pub struct TweetListProps;

pub enum Msg {
    HandleGetTweetListResponse(FetchResponse<Vec<TweetData>>),
    NoOp,
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl Component for TweetList {
    type Message = Msg;
    type Properties = TweetListProps;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut tweet_list = TweetList {
            tweets: Loadable::default(),
            networking: Networking::new(&link),
            link
        };

        warn!("Creating tweet list component");
        tweet_list.networking.fetch(
            &TwitterRequest::Test,
            |r: FetchResponse<Vec<TwitterResponse>>| Msg::HandleGetTweetListResponse(r.map(
                |x: Vec<TwitterResponse>| {
                    x.into_iter()
                        .map(TweetData::from)
                        .collect()
                }
            )),
            &tweet_list.link
        );

        tweet_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            HandleGetTweetListResponse(response) => {
                self.tweets = Loadable::from_fetch_response(response);
                true
            }
            NoOp => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
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
        fn forum_list_fn(tweets: &Vec<TweetData>) -> Html<TweetList> {
            html! {
                <ul class=("tweet-list"),>
                    { for tweets.iter().map(TweetData::view) }
                </ul>
            }
        };
        html! {
            <div>
                { self.tweets.default_view(forum_list_fn) }
            </div>
        }
    }
}

impl Routable for TweetList {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        let first_segment = route.path_segments.get(0).unwrap();
        if "list" == first_segment.as_str() || "" == first_segment.as_str() {
            Some(TweetListProps)
        } else {
            None
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        route.path_segments.get(0).is_some()
    }
}

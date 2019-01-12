use yew::prelude::*;
use yew_router::prelude::*;

use crate::requests::TwitterRequest;
use common::{
    datatypes::tweet::TweetData,
    fetch::{FetchResponse, Networking},
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

#[derive(SmartDefault)]
pub enum Msg {
    HandleGetTweetListResponse(FetchResponse<Vec<TweetData>>),
    FetchTweets(String),
    #[default]
    NoOp,
}

impl Component for TweetList {
    type Message = Msg;
    type Properties = TweetListProps;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tweet_list = TweetList {
            tweets: Loadable::default(),
            networking: Networking::new(&link),
            link,
        };

        // tweet_list.search();

        tweet_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            HandleGetTweetListResponse(response) => {
                self.tweets = Loadable::from_fetch_response(response);
                true
            }
            FetchTweets(query) => {
                self.search(query);
                true
            }
            NoOp => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl TweetList {
    fn search(&mut self, query: String) {
        warn!("Searching for tweets");
        self.networking.fetch(
            &TwitterRequest::Search { query },
            |r: FetchResponse<Vec<TwitterResponse>>| {
                Msg::HandleGetTweetListResponse(
                    r.map(|x: Vec<TwitterResponse>| x.into_iter().map(TweetData::from).collect()),
                )
            },
            &self.link,
        );
    }

    fn forum_list_fn(tweets: &Vec<TweetData>) -> Html<TweetList> {
        html! {
            <ul class=("tweet-list"),>
                { for tweets.iter().map(TweetData::view) }
            </ul>
        }
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
        html! {
            <div>
                { self.tweets.default_view(Self::forum_list_fn) }
            </div>
        }
    }
}

impl Routable for TweetList {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        let first_segment = route.path_segments.get(0).unwrap();
        if "list" == first_segment.as_str() {
            Some(TweetListProps)
        } else {
            None
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        route.path_segments.get(0).is_some()
    }
}

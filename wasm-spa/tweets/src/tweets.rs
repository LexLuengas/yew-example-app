use log::warn;
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use yew::prelude::*;
use yew::{html, html_impl};
use yew_router::prelude::*;

use crate::requests::TwitterRequest;
use common::{
    datatypes::keyword::Keyword,
    datatypes::tweet::TweetData,
    fetch::{FetchResponse, Networking},
};
use util::loadable::Loadable;
use wire::tweet::FullTwitterResponse;

pub struct TweetList {
    tweets: Loadable<Vec<TweetData>>,
    keywords: Vec<Keyword>,
    _router: Box<Bridge<RouterAgent>>,
    networking: Networking,
    link: ComponentLink<TweetList>,
}

#[derive(Clone, PartialEq, Default)]
pub struct TweetListProps {
    pub keywords: Vec<Keyword>,
}

#[derive(SmartDefault, Serialize, Deserialize, Clone)]
pub enum Msg {
    HandleGetTweetListResponse(FetchResponse<Vec<TweetData>>),
    HandleRoute(Route),
    #[default]
    NoOp,
}

impl Component for TweetList {
    type Message = Msg;
    type Properties = TweetListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route| Msg::HandleRoute(route));
        let mut _router = RouterAgent::bridge(callback);
        _router.send(RouterRequest::GetCurrentRoute);

        let networking = Networking::new(&link);

        TweetList {
            tweets: Loadable::default(),
            keywords: props.keywords,
            _router,
            networking,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            HandleGetTweetListResponse(response) => {
                self.tweets = Loadable::from_fetch_response(response);
                true
            }
            HandleRoute(route) => {
                if let Some(query) = route.query {
                    let query: String = query.trim_start_matches("q=").to_string();
                    self.search(query);
                }
                true
            }
            NoOp => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.keywords = props.keywords;
        true
    }
}

impl TweetList {
    fn search(&mut self, query: String) {
        warn!("Searching for tweets");
        self.networking.fetch(
            &TwitterRequest::Search { query },
            |r: FetchResponse<FullTwitterResponse>| {
                Msg::HandleGetTweetListResponse(
                    r.map(|x| x.tweets.into_iter().map(TweetData::from).collect()),
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

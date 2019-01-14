use crate::tweets::{TweetList, TweetListProps};
use common::datatypes::keyword::{Keyword, keywords_from_route};
use yew::prelude::*;
use yew_router::prelude::*;

impl Routable for TweetList {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        let first_segment = route.path_segments.get(0).unwrap();
        if "list" == first_segment.as_str() {
            let keywords: Vec<Keyword> = keywords_from_route(route);
            Some(TweetListProps { keywords })
        } else {
            None
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        route.path_segments.get(0).is_some()
    }
}

// goal here is to send a bunch of requests but
// instead of just failing of one of them fails,
// return a seperate result for each request.
//
// credit to seamonster for example usage of reqwest::async
// that I'm adapting.
// https://github.com/seanmonstar/reqwest/blob/master/examples/async_multiple_requests.rs
//
use failure::{format_err, Error};
use futures::{
    future::{err, join_all, ok},
    Future,
};
use reqwest::r#async::{Client, Response};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Slideshow {
    title: String,
    author: String,
}

#[derive(Deserialize, Debug)]
struct SlideshowContainer {
    slideshow: Slideshow,
}

fn fetch() -> impl Future<Item = Vec<Result<SlideshowContainer, Error>>, Error = ()> {
    let client = Client::new();

    let urls = [
        "https://httpbin.org/json",
        "https://httpbin.org/json",
        "https://httpbin.org/json",
    ];

    let json = |mut res: Response| res.json::<SlideshowContainer>();

    let initial_futures = urls
        .into_iter()
        .map(|url| {
            client
                .get(*url)
                .send()
                .and_then(json)
                .map_err(|_| format_err!("whoopsie!"))
        })
        .collect::<Vec<_>>();

    let final_futures = initial_futures
        .into_iter()
        .map(|response| {
            response.then(|rsp| match rsp {
                Ok(slideshow) => ok(Ok(slideshow)),
                Err(whoopsie) => ok(Err(whoopsie)),
            })
        // is the err value of the future inferred by type declaration of this function?
        })
        .collect::<Vec<_>>();

    join_all(final_futures)
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    //    let results: Vec<Result<SlideshowContainer, Error>> = rt
    //        .block_on(fetch())
    //        .map(|things| things.collect::<Vec<_>>())
    //        .unwrap();
}

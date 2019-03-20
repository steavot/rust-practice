// The goal here is to send a bunch of requests but
// instead of just failing if one of them fails,
// return a seperate result for each request.
//
// credit to seamonster for example usage of reqwest::async
// that I've used as a starting point for this task.
// https://github.com/seanmonstar/reqwest/blob/master/examples/async_multiple_requests.rs
//
#![recursion_limit = "128"]

use failure::{format_err, Error};
use futures::{
    future::{join_all, ok},
    Future,
};
use log::debug;
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
                .then(|x| {
                    debug!("sent request");
                    x
                })
                .and_then(json)
                .map_err(|_| format_err!("whoopsie!"))
                .then(|x| {
                    debug!("parsed response");
                    x
                })
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
    env_logger::init();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let mut results: Vec<Result<SlideshowContainer, Error>> = rt
        .block_on(fetch())
        // We unwrap here because it's impossible for our joined future to return Err.
        .unwrap();

    results.push(Err(format_err!("Let's pretend one request failed")));

    for response in &results {
        println!("{:?}", response);
    }
}

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

// The problem we face is that if we use the Error type of our futures
// to track requests that fail, when we join_all and run the combined
// future we'll either get a collection of Items or an Error.
// That'll just be the first Error, and all our successes will be thrown away.
//
// So we'll create a future that can't fail, and it's Item will encapsulate
// an expression of the various successes or failures of each request.
//
// I think on principle this might be an abuse of the Result Enum and perhaps
// I should be using Either instead. https://docs.rs/either/
// Either way...
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
            // Is the Error type of these futures inferred by type declaration of this function?...
            // must be.
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

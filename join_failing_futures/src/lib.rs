//!
//! Get the results of a collection of futures.
//!
//! Using the old futures API from futures 0.1.
//!
use failure::{format_err, Error};
use futures::{
    future::{err, join_all, ok},
    Future,
};

enum Outcome {
    Good,
    Bad,
}

fn get_single_future(outcome: Outcome) -> impl Future<Item = String, Error = Error> {
    match outcome {
        Outcome::Good => ok("Success!".to_string()),
        Outcome::Bad => err(format_err!("Failure")),
    }
}

fn get_joined_future() -> impl Future<Item = Vec<Result<String, Error>>, Error = ()> {
    let outcomes = vec![Outcome::Good, Outcome::Bad, Outcome::Good];

    let packed_futures = outcomes
        .into_iter()
        .map(|x| {
            get_single_future(x).then(|res| match res {
                // To avoid join_all() returning a single error as soon as
                // one of these futures fails, pack the result of each future
                // into an ok the joined future will resolves into an Ok, which
                // we can unwrap once resolved.
                Ok(message) => ok(Ok(message)),
                Err(whoopsie) => ok(Err(whoopsie)),
            })
        })
        .collect::<Vec<_>>();

    join_all(packed_futures)
}

pub fn get_results() -> Vec<Result<String, Error>> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(get_joined_future()).unwrap()
}

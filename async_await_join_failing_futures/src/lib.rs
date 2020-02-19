//!
//! Get the results of a collection of futures.
//!
//! Using async await syntax and futures 0.3.
//!
use failure::{format_err, Error};
use futures::{
    future::join_all,
    executor::block_on,
stream::{FuturesUnordered, StreamExt as _},
};

enum Outcome {
    Good,
    Bad,
}

async fn get_single_future(outcome: Outcome) -> Result<String, Error> {
    match outcome {
        Outcome::Good => Ok("Success!".to_string()),
        Outcome::Bad => Err(format_err!("Failure")),
    }
}

async fn get_joined_future() -> Vec<Result<String, Error>> {
    let outcomes = vec![Outcome::Good, Outcome::Bad, Outcome::Good];

    let some_futures = outcomes
        .into_iter()
        .map(|outcome| async {
            get_single_future(outcome).await
        })
        .collect::<Vec<_>>();

    join_all(some_futures).await
}

async fn get_futures_via_stream() -> Vec<Result<String, Error>> {
    let outcomes = vec![Outcome::Good, Outcome::Bad, Outcome::Good];

    outcomes
        .into_iter()
        .map(|outcome| async {
            get_single_future(outcome).await
        })
        .collect::<FuturesUnordered<_>>()
        .collect()
        .await
}

pub fn get_results_with_join() -> Vec<Result<String, Error>> {
    block_on(get_joined_future())
}

pub fn get_results_with_stream() -> Vec<Result<String, Error>> {
    block_on(get_futures_via_stream())
}

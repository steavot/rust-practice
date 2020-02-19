//!
//! Get the results of a collection of futures.
//!
//! Using async await syntax and futures 0.3.
//!
use failure::{format_err, Error};
use futures::{
    executor::block_on,
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
    use futures::future::join_all;

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
    use futures::stream::{FuturesUnordered, StreamExt as _};

    let outcomes = vec![Outcome::Good, Outcome::Bad, Outcome::Good];

    outcomes
        .into_iter()
        .map(|outcome| async {
            get_single_future(outcome).await
        })
        // First we collect the collection of futures into the
        // stream type `FuturesUnordered`.
        .collect::<FuturesUnordered<_>>()
        // Then we collect the stream into a future, and await.
        // This collect() method is from the StreamExt trait.
        .collect()
        .await
}

pub fn get_results_with_join() -> Vec<Result<String, Error>> {
    block_on(get_joined_future())
}

pub fn get_results_with_stream() -> Vec<Result<String, Error>> {
    block_on(get_futures_via_stream())
}

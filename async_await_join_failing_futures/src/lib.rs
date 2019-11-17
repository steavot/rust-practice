//!
//! Get the results of a collection of futures.
//!
//! Using async await syntax and futures 0.3.
//!
use failure::{format_err, Error};
use futures::{
    future::join_all,
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
    let outcomes = vec![Outcome::Good, Outcome::Bad, Outcome::Good];

    let packed_futures = outcomes
        .into_iter()
        .map(|outcome| async {
            // To avoid join_all() returning a single error as soon as
            // one of these futures fails, pack the result of each future
            // into an Ok which we'll unwrap below after joining.
            match get_single_future(outcome).await {
                Ok(message) => Ok::<Result<String, Error>, ()>(Ok(message)),
                Err(whoopsie) => Ok(Err(whoopsie)),
            }
        })
        .collect::<Vec<_>>();

    join_all(packed_futures).await.into_iter().map(|x| x.unwrap()).collect()
}

pub fn get_results() -> Vec<Result<String, Error>> {
    block_on(get_joined_future())
}

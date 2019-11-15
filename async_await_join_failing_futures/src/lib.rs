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

async fn get_joined_future() -> Vec<Result<Result<String, Error>, Error>> {
    let outcomes = vec![Outcome::Good, Outcome::Bad, Outcome::Good];

    let packed_futures = outcomes
        .into_iter()
        .map(|outcome| async {
            match get_single_future(outcome).await {
                Ok(message) => Ok(Ok(message)),
                Err(whoopsie) => Ok(Err(whoopsie)),
            }
        })
        .collect::<Vec<_>>();

    join_all(packed_futures).await

}

pub fn get_results() -> Vec<Result<String, Error>> {
    block_on(get_joined_future()).into_iter().map(|x| x.unwrap()).collect()
}

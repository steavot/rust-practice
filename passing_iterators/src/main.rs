#![feature(wrapping_next_power_of_two)]

use std::collections::HashSet;

fn returns_iterator() -> impl Iterator<Item=usize> {
    vec![1,2,3,4,5,6,7,8,9]
        .into_iter()
        .map(|x| x*x)
}

// fn returns_iterator_in_iterator() -> impl Iterator<Item=Iterator<Item=usize> + 'static> {
//     vec![1,2,3,4,5,6,7,8,9]
//         .into_iter()
//         .map(|_| vec![1,2,3,4,5,6,7,8,9].iter())
// }

fn takes_iterator<I, T>(thing: I)
    where I: Iterator<Item=T>,
          T: std::fmt::Debug,
{
    println!("{:?}", thing.collect::<Vec<_>>());
}

fn returns_iterator_in_result<A>(thing: A) -> Result<impl Iterator<Item=usize>, ()>
    where A: Iterator<Item=usize>,
{
    Ok(thing.map(|x| x.wrapping_next_power_of_two()))
}

fn returns_result_in_iterator<A, B, C>(thing: A) -> impl Iterator<Item=Result<B, C>>
    where A: Iterator<Item=B>,
{
    thing.map(|x| Ok(x))
}

fn main() {
    let thing = returns_iterator();
    // println!("{:?}", thing);
    // `impl std::iter::Iterator` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`

    println!("{:?}", thing.collect::<HashSet<_>>());

    takes_iterator(returns_iterator());

    println!("{:?}", returns_iterator_in_result(returns_iterator()).map(|x| x.collect::<Vec<_>>()));
    println!("{:?}", returns_iterator_in_result(returns_iterator()).map(|x| x.collect::<HashSet<_>>()));

    println!("{:?}", returns_result_in_iterator(returns_iterator()).collect::<Result<Vec<_>, ()>>());

    //println!("{:?}", returns_iterator_in_iterator()
}

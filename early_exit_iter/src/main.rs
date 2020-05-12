use anyhow::{anyhow, Result};

fn foo(x: usize) -> Result<usize> {
    // std::thread::sleep(std::time::Duration::from_secs(1));
    if x != 10 {
        println!("{}", x);
        Ok(x)
    } else {
        let msg = format!("{} is equal 10", x);
        println!("{}", msg);
        Err(anyhow!("{}", msg))
    }
}

fn bar(x: usize) -> Option<usize> {
    // std::thread::sleep(std::time::Duration::from_secs(1));
    if x != 10 {
        println!("{}", x);
        Some(x)
    } else {
        let msg = format!("{} is equal 10", x);
        println!("{}", msg);
        None
    }
}

#[allow(unused_must_use)]
fn main() {
    // With maps this works fine.
    println!("\nwith a for loop:");
    for n in 1.. {
        if let Err(_) = foo(n) {
            println!("found error");
            break;
        }
    }

    println!("\nwith a map:");
    (1..).map(foo).collect::<Result<Vec<_>>>();

    println!("\nwith a map with Option:");
    (1..).map(bar).collect::<Option<Vec<_>>>();

    // With folds...
    let mut acc = 0;
    for n in 1.. {
        match foo(n) {
            Err(_e) => {
                println!("found error");
                break;
            }
            Ok(n) => acc = n,
        };
    }
    println!("final acc {}", acc);

    println!("\ntry a fold:");
    (1..20).fold(Ok(0), |acc: Result<usize>, x| match acc {
        Err(e) => {
            println!("found error");
            Err(e)
        }
        Ok(_) => foo(x),
    });

    use itertools::Itertools as _;

    println!("\nuse a fold_results:");
    (1..).map(foo).fold_results(0, |_acc, x| x);

    println!("\ntry a fold with Option accumulator:");
    (1..20).fold(Some(0), |acc: Option<usize>, x| match acc {
        None => {
            println!("found error");
            None
        }
        Some(_) => bar(x),
    });

    println!("\nuse a fold_options:");
    (1..).map(bar).fold_options(0, |_acc, x| x);
}

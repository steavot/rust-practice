use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SomeData {
    name: String,
    age: String,
    other: OtherData,
}

#[derive(Debug, Serialize, Deserialize)]
struct OtherData {
    town: String,
}

fn main() -> anyhow::Result<()> {
    // One liner, useful in future.
    // let typed = serde_yaml::from_reader::<std::io::Stdin, SomeData>(std::io::stdin());

    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;

    // let typed = serde_yaml::from_str::<SomeData>(&input)?;
    let untyped = serde_yaml::from_str::<serde_yaml::Value>(&input)?;

    // println!("\ntyped read as:");
    // println!("{:?}", typed);
    println!("\nuntyped read as:");
    println!("{:?}", untyped);

    Ok(())
}

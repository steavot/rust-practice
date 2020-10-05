use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{to_value as to_json_value, Value};
use serde_yaml::{from_reader, to_vec};
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    output: String,
    format: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct Data {
    name: String,
    data: Value,
    aliases: HashSet<String>,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let input: Data = from_reader(stdin()).context("couldn't read stdin")?;

    let output = if args.output == "data" {
        val_to_vec(input.data, &args.format)?
    } else if args.output == "schema" {
        let mut schema = schema_for!(Data);
        schema.schema.metadata().description = Some(format!(
            "aliases can be {}",
            input.aliases.into_iter().format(", ")
        ));

        val_to_vec(to_json_value(schema)?, &args.format)?
    } else {
        return Err(anyhow!("Unknown output type"));
    };

    stdout().write_all(&output)?;

    Ok(())
}

fn val_to_vec(value: Value, form: &str) -> Result<Vec<u8>> {
    if form == "yaml" {
        to_vec(&value).map_err(Into::into)
    } else if form == "json" {
        serde_json::to_vec(&value).map_err(Into::into)
    } else if form == "toml" {
        toml::to_vec(&value).map_err(Into::into)
    } else {
        Err(anyhow!("Unknown output format"))
    }
}

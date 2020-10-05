#[derive(Debug, structopt::StructOpt)]
struct Cli {
    output: String,
    format: String,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct Data {
    name: String,
    data: serde_json::Value,
    aliases: std::collections::HashSet<String>,
}

fn main() -> anyhow::Result<()> {
    let args = {
        use structopt::StructOpt as _;
        Cli::from_args()
    };

    let input: Data = {
        use anyhow::Context as _;
        serde_yaml::from_reader(std::io::stdin()).context("couldn't read stdin")?
    };

    let output = if args.output == "data" {
        val_to_vec(input.data, &args.format)?
    } else if args.output == "schema" {
        let mut schema = schemars::schema_for!(Data);

        schema.schema.metadata().description = Some({
            use itertools::Itertools as _;
            format!("aliases can be {}", input.aliases.into_iter().format(", "))
        });

        val_to_vec(serde_json::to_value(schema)?, &args.format)?
    } else {
        return Err(anyhow::anyhow!("Unknown output type"));
    };

    {
        use std::io::Write as _;
        std::io::stdout().write_all(&output)?;
    }

    Ok(())
}

fn val_to_vec(value: serde_json::Value, form: &str) -> anyhow::Result<Vec<u8>> {
    if form == "yaml" {
        serde_json::to_vec(&value).map_err(Into::into)
    } else if form == "json" {
        serde_json::to_vec(&value).map_err(Into::into)
    } else if form == "toml" {
        toml::to_vec(&value).map_err(Into::into)
    } else {
        Err(anyhow::anyhow!("Unknown output format"))
    }
}

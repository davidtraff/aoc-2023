use reqwest::StatusCode;
pub use tokio;
pub use anyhow;

use anyhow::{Result, bail};
use std::path::Path;
use tokio::fs::{create_dir_all, read_to_string, try_exists, write};

const AOC_TOKEN_ENV_NAME: &str = "AOC_TOKEN";

fn find_token() -> Option<String> {
    let token = dotenv::vars().find(|(v, _)| v.eq(AOC_TOKEN_ENV_NAME));

    if let Some((_, token)) = token {
        return Some(token);
    }

    None
}

async fn get_input(year: u32, day: u32) -> Result<String> {
    let Some(token) = find_token() else {
        bail!("Missing Advent of Code token");
    };

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("cookie", format!("session={}", token))
        .send()
        .await?;

    let status = response.status();
    if status != StatusCode::OK {
        bail!("Server returned status code {}. Did you set your AoC token?", status);
    }

    response.text().await.map_err(|e| e.into())
}

pub async fn start(year: u32, day: u32) -> Result<String>
{
    let year = 2022;
    let day = 1;

    let filename = format!("{}-{:0>2}.txt", year, day);
    let current_dir = std::env::current_dir()?;
    let path = Path::new(&current_dir).join("input");
    let file_path = path.join(filename);

    create_dir_all(&path).await?;

    let input = match try_exists(&file_path).await {
        Ok(true) => {
            println!("Serving local input.");
            
            read_to_string(file_path).await?
        },
        _ => {
            println!("Fetching input from AoC");

            let input = get_input(year, day).await;

            let input = match input {
                Ok(i) => i,
                Err(e) => {
                    bail!(
                        "Could not get the puzzle input for year {}, day {}. Reason: {:?}",
                        year,
                        day,
                        e
                    )
                }
            };

            write(file_path, &input).await?;

            input
        }
    };

    Ok(input)
}

#[macro_export]
macro_rules! aoc_main {
    ($yr: literal, $day: literal) => {
        use aoc_core::start;
        use aoc_core::tokio;
        use aoc_core::anyhow::Result;

        #[tokio::main]
        async fn main() -> Result<()>
        {
            let input = start($yr, $day).await?;

            println!("Running solution for part 1");
            let result = part_one(&input);
            println!("Solution for part 1 was \"{:?}\"", result);

            println!("Running solution for part 2");
            let result = part_two(&input);
            println!("Solution for part 2 was \"{:?}\"", result);

            Ok(())
        }
    };
}

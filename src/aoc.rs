use chrono::{DateTime, Utc};
use std::{fs::File, io::Write, path::Path};

use crate::cli::Aoc;
use crate::utils;

impl Aoc {
    pub fn is_valid_date(&self) -> Result<bool, String> {
        // Fix the timestamp with the EST (UTC-5) timezone
        let timestamp = Utc::now().timestamp() - 5 * 60 * 60;
        let now = DateTime::from_timestamp(timestamp, 0).unwrap();
        let current_year = utils::last_year();
        let current_month = format!("{}", now.format("%m")).parse::<u32>().unwrap();
        let current_day = format!("{}", now.format("%d")).parse::<u32>().unwrap();

        if self.year < 2015 || self.year > current_year {
            return Err(format!(
                "ERROR: The year must be between 2015 and {}",
                current_year
            ));
        }

        if self.day < 1 || self.day > 25 {
            return Err(format!("ERROR: The day must be between 1 and 25"));
        }

        if self.year == current_year && current_month == 12 && self.day > current_day {
            return Err(format!(
                "ERROR: The challenge {} was not release yet this year {}",
                self.day, current_year
            ));
        }

        Ok(true)
    }

    pub fn fetch_puzzle(&self, token: &str) -> Result<String, ureq::Error> {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        );
        let session = format!("session={}", token);

        let body: String = ureq::get(url)
            .header("cookie", session)
            .call()?
            .body_mut()
            .read_to_string()?;

        let puzzle = body.trim_end_matches('\n').to_string();

        Ok(puzzle)
    }

    pub fn write_file(&self, puzzle: &str) -> Result<String, String> {
        if !Path::new(&self.path).exists() || !Path::new(&self.path).is_dir() {
            return Err(format!(
                "ERROR: The folder '{}' does not exist or is not a directory.",
                self.path
            ));
        }

        let filename = format!("puzzle_{}_{:02}.txt", self.year, self.day);
        let filepath = Path::new(self.path.as_str()).join(filename.as_str());

        let file = File::create(filepath.clone());
        if let Err(_) = file {
            return Err(format!("ERROR: Failed to create file"));
        }

        if let Err(_) = file.unwrap().write_all(puzzle.as_bytes()) {
            return Err(format!("ERROR: Write failed"));
        }

        return Ok(format!("File saved as: {}", filepath.to_str().unwrap()));
    }
}

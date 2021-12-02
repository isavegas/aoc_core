use crate::AoCDay;
use std::collections::HashMap;
use clap::{clap_app, AppSettings};

pub struct AoCProject {
    pub year: usize,
    pub version: String,
    pub title: String,
    pub author: String,
    pub url: Option<String>,
}

impl AoCProject {
    pub fn new(
        year: usize,
        version: String,
        title: Option<String>,
        author: String,
        url: Option<String>,
    ) -> Self {
        AoCProject {
            year,
            version,
            title: title.unwrap_or_else(|| format!("AoC {}", year)),
            author,
            url,
        }
    }
    pub fn run<S>(&self, days: &[Box<dyn AoCDay>], inputs: &HashMap<usize, S>) where S: AsRef<str> + std::fmt::Display {
        let matches = clap_app!(aoc_core =>
            (version: self.version.as_str())
            (author: self.author.as_str())
            (about: format!("Solutions for Advent of Code {}", self.year).as_str())
            (setting: AppSettings::SubcommandRequiredElseHelp)
            (@arg verbose: --verbose -v ... "Sets the level of verbosity")
            (@subcommand version =>
                (about: "shows version number")
            )
            (@subcommand run =>
                (about: "executes requested solution(s)")
                (@arg day: --day -d +takes_value "indicate a specific day")
                (@arg part: --part -p +takes_value #{1,2} "indicate a specific part")
                // TODO: (@arg stdin: --stdin -i "flag for providing input over stdin")
                (@arg input_file: --("input-file") -f +takes_value "location of an input file")
            )
            (@subcommand list =>
                (about: "lists implemented days")
            )
        )
        .get_matches();

        let mut day: Option<usize> = None;
        let mut part: Option<usize> = None;
        let mut err: bool = false;

        if let Some(run_cmd) = matches.subcommand_matches("run") {
            if let Some(day_str) = run_cmd.value_of("day") {
                match day_str.parse::<usize>() {
                    Ok(d) => day = Some(d),
                    Err(_) => {
                        err = true;
                        eprintln!("Unable to parse day value");
                    }
                }
            }
            if let Some(part_str) = run_cmd.value_of("part") {
                match part_str.parse::<usize>() {
                    Ok(p) => {
                        if p == 1 || p == 2 {
                            part = Some(p);
                        } else {
                            err = true;
                            eprintln!("Invalid part value")
                        }
                    },
                    Err(_) => {
                        err = true;
                        eprintln!("Unable to parse part value")
                    }
                }
            }

            if !err {
                match day {
                    None => {
                        if run_cmd.value_of("part").is_some() {
                            eprintln!("Cannot specify part without day!");
                        } else {
                            for d in days.iter() {
                                if let Some(input) = inputs.get(&d.day()) {
                                    run_day(d, None, input);
                                } else {
                                    eprintln!("Input not found for day {}", d.day());
                                }
                            }
                        }
                    }
                    Some(selected_day) => {
                        let mut input_opt = None;

                        if let Some(path) = run_cmd.value_of("input_file") {
                            input_opt = Some(std::fs::read_to_string(path).expect(""));
                        }

                        if input_opt == None {
                            input_opt = inputs.get(&selected_day).map(|s| s.to_string());
                        }

                        if let Some(input) = input_opt {
                            if let Some(day) = days.iter().find(|d| d.day() == selected_day) {
                                run_day(day, part, input);
                            } else {
                                eprintln!("Implementation not found for day {}", selected_day);
                            }
                        } else {
                            eprintln!("Input not found for day {}", selected_day);
                        }
                    }
                }
            }
        } else if let Some(_list_cmd) = matches.subcommand_matches("list") {
            for d in days.iter() {
                println!("Day {:>1}", d.day());
            }
        }
    }
}

#[allow(clippy::redundant_pattern_matching, clippy::borrowed_box)]
pub fn run_day<S>(day: &Box<dyn AoCDay>, part: Option<usize>, input: S) where S: AsRef<str> + std::fmt::Display {
    match part {
        Some(p) => {
            let (expected, val) = match p {
                1 => (day.expected().0, day.part1(input.as_ref())),
                2 => (day.expected().1, day.part2(input.as_ref())),
                _ => unreachable!(),
            };

            let (status1, value) = check_status(expected, val);
            println!(
                "Day {:02}, Part {}: {} {}",
                day.day(),
                p,
                status1,
                value
            );
        }
        None => {
            let (status1, value1) = check_status(day.expected().0, day.part1(input.as_ref()));
            let (status2, value2) = check_status(day.expected().1, day.part2(input.as_ref()));
            println!("Day {:02}, Part 1: {} {}", day.day(), status1, value1);
            println!("        Part 2: {} {}", status2, value2);
        }
    }
}


#[derive(Debug)]
enum TestStatus {
    Unknown,
    Failure,
    Success,
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Failure => write!(f, "✗"),
            Self::Success => write!(f, "✓"),
        }
    }
}

fn check_status(
    expected: Option<&str>,
    value: Result<String, crate::ErrorWrapper>,
) -> (TestStatus, String) {
    match value {
        Ok(val) => {
            let status = match expected {
                Some(v) => match v == val {
                    true => TestStatus::Success,
                    false => TestStatus::Failure,
                },
                None => TestStatus::Unknown,
            };
            (status, val)
        }
        Err(err) => (TestStatus::Failure, err.to_string()),
    }
}

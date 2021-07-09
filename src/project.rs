use crate::AoCDay;
//use clap::{clap_app, App, Arg, SubCommand};
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
    pub fn run(&self, days: &[Box<dyn AoCDay>]) {
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
                                run_day(d, None);
                            }
                        }
                    }
                    Some(selected_day) => {
                        if let Some(day) = days.iter().find(|d| d.day() == selected_day) {
                            run_day(day, part);
                        } else {
                            eprintln!("Day not found!");
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::redundant_pattern_matching, clippy::borrowed_box)]
pub fn run_day(day: &Box<dyn AoCDay>, part: Option<usize>) {
    match part {
        Some(p) => {
            let (expected, val) = match p {
                1 => (day.expected().0, day.part1()),
                2 => (day.expected().1, day.part2()),
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
            let (status1, value1) = check_status(day.expected().0, day.part1());
            let (status2, value2) = check_status(day.expected().1, day.part2());
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

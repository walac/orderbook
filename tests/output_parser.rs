#[path = "util.rs"]
mod util;

use orderbook::{LogEntry, Side};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use util::{parse_usize, State};

#[derive(Debug)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub logs: Vec<LogEntry>,
}

/// Parse the output file
pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Scenario>> {
    let f = File::open(path)?;
    let lines = BufReader::new(f)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;

    let mut state = State::Name;

    let mut ret: Vec<Scenario> = Vec::new();
    let mut scenario = Scenario {
        name: "".to_owned(),
        description: "".to_owned(),
        logs: Vec::new(),
    };

    for line in &lines {
        state = match state {
            State::Name => {
                if line.starts_with("#name") {
                    scenario.name = line
                        .split(':')
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_owned();

                    State::Descr
                } else {
                    State::Name
                }
            }

            State::Descr => {
                if line.starts_with("#descr") {
                    scenario.description = line
                        .split(':')
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_owned();

                    State::Fields
                } else {
                    State::Descr
                }
            }

            State::Fields => {
                if line.starts_with("#name") {
                    ret.push(scenario);
                    scenario = Scenario {
                        name: line
                            .split(':')
                            .skip(1)
                            .collect::<Vec<_>>()
                            .join("")
                            .trim()
                            .to_owned(),
                        description: "".to_owned(),
                        logs: Vec::new(),
                    };

                    State::Descr
                } else if line.len() > 0 {
                    let op = line
                        .chars()
                        .nth(0)
                        .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid line"))?;

                    let fields = line
                        .split(',')
                        .skip(1)
                        .map(|s| s.trim())
                        .collect::<Vec<_>>();

                    match op {
                        'A' => scenario.logs.push(LogEntry::Acknowledge {
                            user_id: parse_usize(fields[0])?,
                            order_id: parse_usize(fields[1])?,
                        }),
                        'R' => scenario.logs.push(LogEntry::Reject {
                            user_id: parse_usize(fields[0])?,
                            order_id: parse_usize(fields[1])?,
                        }),
                        'B' => {
                            let order_entry = if fields[1] == "-" && fields[2] == "-" {
                                LogEntry::TopOfBook {
                                    side: None,
                                    price: 0,
                                    volume: 0,
                                }
                            } else {
                                LogEntry::TopOfBook {
                                    side: Some(Side::new(fields[0].chars().nth(0).ok_or(
                                        io::Error::new(
                                            io::ErrorKind::InvalidInput,
                                            "Can't index side",
                                        ),
                                    )?))
                                    .ok_or(
                                        io::Error::new(io::ErrorKind::InvalidInput, "Invalid side"),
                                    )?,
                                    price: parse_usize(fields[1])?,
                                    volume: parse_usize(fields[2])?,
                                }
                            };

                            scenario.logs.push(order_entry);
                        }
                        'T' => scenario.logs.push(LogEntry::Trade {
                            user_id_buy: parse_usize(fields[0])?,
                            order_id_buy: parse_usize(fields[1])?,
                            user_id_sell: parse_usize(fields[2])?,
                            order_id_sell: parse_usize(fields[3])?,
                            price: parse_usize(fields[4])?,
                            volume: parse_usize(fields[5])?,
                        }),
                        _ => (),
                    }

                    State::Fields
                } else {
                    State::Fields
                }
            }
        }
    }

    if state == State::Fields {
        ret.push(scenario);
    }

    Ok(ret)
}

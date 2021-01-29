#[path = "util.rs"]
mod util;

use orderbook::{Order, Side};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use util::{parse_usize, State};

#[derive(Debug)]
pub enum OrderType {
    New(String, Order),
    Cancel(usize, usize),
}

#[derive(Debug)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub orders: Vec<OrderType>,
}

/// Parse the input file
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
        orders: Vec::new(),
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
                if line.len() > 0 {
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
                        'N' => scenario.orders.push(OrderType::New(
                            fields[1].to_owned(),
                            Order {
                                user_id: parse_usize(fields[0])?,
                                price: parse_usize(fields[2])?,
                                volume: parse_usize(fields[3])?,
                                side: Side::new(fields[4].chars().nth(0).ok_or(io::Error::new(
                                    io::ErrorKind::InvalidInput,
                                    "Can't index side",
                                ))?)
                                .ok_or(io::Error::new(
                                    io::ErrorKind::InvalidInput,
                                    "Invalid side",
                                ))?,
                                order_id: parse_usize(fields[5])?,
                            },
                        )),
                        'C' => scenario.orders.push(OrderType::Cancel(
                            parse_usize(fields[0])?,
                            parse_usize(fields[1])?,
                        )),
                        'F' => {
                            ret.push(scenario);
                            scenario = Scenario {
                                name: "".to_owned(),
                                description: "".to_owned(),
                                orders: Vec::new(),
                            };
                        }
                        _ => (),
                    }
                }

                State::Fields
            }
        }
    }

    Ok(ret)
}

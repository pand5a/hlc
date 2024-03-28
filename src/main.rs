// Parsing Cockroach HLC to Date Time
// 1657662798.107432301,2147483647
// 1657662798.107432301.2147483647
// 1657662798107432301,2147483647
// 1657662798107432301.2147483647
// 1657662798.107432301
// 1657662798107432301
// 1708329839396 // unix timestamp

use std::fmt::Debug;
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use clap::{App, Arg};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use chrono::{DateTime, Local, Utc};
extern crate clap;

#[derive(Parser)]
#[grammar = "hlc.pest"]
struct HLCParser;

fn print_time(nsec: u64, utc: bool) {
    let d = Duration::from_nanos(nsec);
    let time = SystemTime::from(UNIX_EPOCH.add(d));
    let fmt = "%Y-%m-%d %H:%M:%S%.f";

    let format_time: String;
    if utc {
        let datetime: DateTime<Utc> = time.into();
        format_time = datetime.format(fmt).to_string()
    }else {
        let datetime: DateTime<Local> = time.into();
        format_time = datetime.format(fmt).to_string()
    }
    println!("{}, {}", format_time, nsec);
}

fn parse_hlc(pair: Pair<Rule>, utc: bool) {
    match pair.as_rule() {
        Rule::hlc | Rule::input => {
            parse_hlc(pair.into_inner().next().unwrap(), utc);
        },
        Rule::hlc_wall_time =>{
            let wall_time = pair.as_str().replace(".", "");
            let n: u64 = wall_time.parse().unwrap();
            print_time(n, utc);
        },
        Rule::unix_timestamp => {
            let unix_ms= pair.as_str();
            let n: u64 = unix_ms.parse().unwrap();
            print_time(n* 1000000, utc);
        },
        _ => {}
    }
}

fn hlc(input: &str, utc: bool) {
    match HLCParser::parse(Rule::hlc, &input) {
        Ok(mut pairs) => {
            parse_hlc(pairs.next().unwrap(), utc);
        }
        Err(_) => {
            println!("input hlc timestamp error: '{}'", input)
        }
    }
}

fn main() {
    let matches = App::new("hlc to date")
        .version("0.1.0")
        .author("mini.pand5a@gmail.com")
        .arg(Arg::with_name("utc")
            .short("utc")
            .takes_value(false)
            .help("show utc date"))
        .arg(Arg::with_name("input")
            .index(1)
            .required(true)
            .help("hlc timestamp")
        ).get_matches();

    let utc = matches.is_present("utc");
    let input = matches.value_of("input").unwrap();

    hlc(input, utc);
}
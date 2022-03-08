use std::env;
use std::process::exit;
use std::ops::Sub;
use fake::Fake;
use fake::faker::address::en::{CityName, CountryName, StreetName, BuildingNumber, PostCode};
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::name::en::{FirstName, LastName};
use fake::faker::phone_number::en::CellNumber;
use chrono::Duration;
use chrono::prelude::*;
use owo_colors::OwoColorize;

macro_rules! named_println {
    ($x:expr, $( $more:expr ),*) => {
        println!("{} {}", $x.green().bold(), $($more)*)
    };
}

fn flags_to_str(flags: &Vec<(bool, char, &str)>) -> String {
    let str = &mut String::new();
    for c in flags.iter().filter(|f| f.0) {
        str.push(c.1);
    }
    return str.to_string()
}

fn fake_city() -> String {
    CityName().fake::<String>() + ", " + &CountryName().fake::<String>() + " [" + &PostCode().fake::<String>() + "]"
}

fn fake_address() -> String {
  StreetName().fake::<String>() + " " + &BuildingNumber().fake::<String>() + ", " + &fake_city()
}

fn fail(flags: &Vec<(bool, char, &str)>) {
    println!("Usage: {} flags", env::args().collect::<Vec<String>>()[0]);
    println!("The `flags` parameter is a sequence of characters each toggling the generation of a field");
    println!("By default its value is `{}`", flags_to_str(flags));
    for flag in flags.iter() {
        println!("    {} \t\t{} (default: {:?})", flag.1, flag.2, flag.0);
    }
    exit(1);
}

fn main() {
    let flags = &mut vec![
        (true, 'f', "Generate a first name"),
        (true, 'l', "Generate a last name"),
        (true, 'b', "Generate a birth date"),
        (true, 'p', "Generate a birth place"),
        (true, 't', "Generate a phone number"),
        (false, 's', "Generate a shipping location")
    ];
    let (min_date, max_date) = (Utc.ymd(1970, 1, 1).and_time(NaiveTime::from_hms(0,0,0)).unwrap(),
        Utc::now().sub(Duration::weeks(21*53))); // 21 years just to stay safe

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        fail(&flags);
    }

    if args.len() == 2 {
        for c in args[1].chars() {
            match flags.iter_mut().find(|flag| flag.1 == c) {
                Some(e) => {
                    e.0 = !e.0;
                },
                None => {
                    println!("Invalid flag: '{}'", c);
                    fail(&flags);
                }
            }
        }
    }

    for flag in flags.iter().filter(|flag| flag.0) {
        match flag.1 {
            'f' => named_println!("  First name", FirstName().fake::<String>()),
            'l' => named_println!("   Last name", LastName().fake::<String>()),
            'b' => named_println!("   Bith date", DateTimeBetween(min_date, max_date).fake::<DateTime<Utc>>().date()),
            'p' => named_println!("  Bith place", fake_city()),
            't' => named_println!("       Phone", CellNumber().fake::<String>()),
            's' => named_println!("    Shipping", fake_address()),
            v => println!("'{}' is not implemented yet", v)
        }
    }
}

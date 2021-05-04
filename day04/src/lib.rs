use aoc2020::parse;
use itertools::Itertools;
use std::{path::Path};
use thiserror::Error;

#[derive(Debug, Clone)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        }
    }

    pub fn set_byr(&mut self, val: String) {
        match val.parse::<i32>() {
            Ok(val) => {
                if val >= 1920 && val <= 2002 {
                    self.byr = Some(val);      
                }
            },
            Err(err) => return
        }
    }

    pub fn set_iyr(&mut self, val: String){
        match val.parse::<i32>() {
            Ok(val) => {
                if val >= 2010 && val <= 2020 {
                    self.iyr = Some(val);
                }
            },
            Err(err) => return
        }
    }

    pub fn set_eyr(&mut self, val: String) {
        match val.parse::<i32>() {
            Ok(val) => {
                if val >= 2020 && val <= 2030 {
                    self.eyr = Some(val);      
                }
            },
            Err(err) => return
        }
    }

    pub fn is_valid(&self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input: Vec<String> = parse(input)?.collect();

    let mut passports: Vec<Passport> = Vec::new();
    let mut passport: Passport = Passport::new();    
    for line in input {
        if line.is_empty() {
            passports.push(passport.clone());
            passport = Passport::new();
            continue;
        }
        let tokens = line.split_whitespace();
        for token in tokens {
            let (key, val): (&str, &str) = token.split(":")
               .collect_tuple()
               .unwrap();

            match key {
                "byr" => passport.byr = Some(val.parse().unwrap()),
                "iyr" => passport.iyr = Some(val.parse().unwrap()),
                "eyr" => passport.eyr = Some(val.parse().unwrap()),
                "hgt" => passport.hgt = Some(val.to_string()),
                "hcl" => passport.hcl = Some(val.to_string()),
                "ecl" => passport.ecl = Some(val.to_string()),
                "pid" => passport.pid = Some(val.to_string()),
                "cid" => passport.cid = Some(val.to_string()),
                _ => continue
            }
        }
    }
    
    println!("Valid passports: {}", passports.iter().filter(|passport| passport.is_valid() ).count());

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let input: Vec<String> = parse(input)?.collect();

    let mut passports: Vec<Passport> = Vec::new();
    let mut passport: Passport = Passport::new();    
    for line in input {
        if line.is_empty() {
            passports.push(passport.clone());
            passport = Passport::new();
            continue;
        }
        let tokens = line.split_whitespace();
        for token in tokens {
            let (key, val): (&str, &str) = token.split(":")
               .collect_tuple()
               .unwrap();

            match key {
                "byr" => passport.set_byr(val.to_string()),
                "iyr" => passport.set_iyr(val.to_string()),
                "eyr" => passport.set_eyr(val.to_string()),
                "hgt" => passport.hgt = Some(val.to_string()),
                "hcl" => passport.hcl = Some(val.to_string()),
                "ecl" => passport.ecl = Some(val.to_string()),
                "pid" => passport.pid = Some(val.to_string()),
                "cid" => passport.cid = Some(val.to_string()),
                _ => continue
            }
        }
    }
    
    println!("Valid passports: {}", passports.iter().filter(|passport| passport.is_valid() ).count());

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid argument(s) `{0}`")]
    InvalidArgs(String),
}


/* Alternative Solution
use aoc2020::input::parse_newline_sep;

use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[cfg(feature = "emit_json")]
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref EYE_COLOR_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

#[derive(Default)]
#[cfg_attr(feature = "emit_json", derive(Serialize, Deserialize))]
struct Passport {
    #[cfg_attr(feature = "emit_json", serde(rename = "birth_year", alias = "byr"))]
    byr: Option<u32>,
    #[cfg_attr(feature = "emit_json", serde(rename = "issued_year", alias = "iyr"))]
    iyr: Option<u32>,
    #[cfg_attr(feature = "emit_json", serde(rename = "expiry_year", alias = "eyr"))]
    eyr: Option<u32>,
    #[cfg_attr(feature = "emit_json", serde(skip))]
    hgt_set: bool,
    #[cfg_attr(feature = "emit_json", serde(rename = "height", alias = "hgt"))]
    hgt: Option<Height>,
    #[cfg_attr(feature = "emit_json", serde(rename = "hair_color", alias = "hcl"))]
    hcl: Option<String>,
    #[cfg_attr(feature = "emit_json", serde(rename = "eye_color", alias = "ecl"))]
    ecl: Option<String>,
    #[cfg_attr(feature = "emit_json", serde(rename = "passport_id", alias = "pid"))]
    pid: Option<String>,
}

impl Passport {
    fn has_northpole_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt_set
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_inner(&self) -> Option<bool> {
        let valid = (1920..=2002).contains(&self.byr?)
            && (2010..=2020).contains(&self.iyr?)
            && (2020..=2030).contains(&self.eyr?)
            && match self.hgt.as_ref()? {
                Height::Cm(cm) => (150..=193).contains(cm),
                Height::In(inch) => (59..=76).contains(inch),
            }
            && HAIR_COLOR_RE.is_match(self.hcl.as_ref()?)
            && EYE_COLOR_RE.is_match(self.ecl.as_ref()?)
            && PASSPORT_ID_RE.is_match(self.pid.as_ref()?);
        Some(valid)
    }

    fn is_valid(&self) -> bool {
        self.is_valid_inner().unwrap_or_default()
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();

        for field in s.split_whitespace() {
            let mut parts = field.split(':');
            let key = parts
                .next()
                .ok_or_else(|| format!("missing key in '{}'", field))?;
            let value = parts
                .next()
                .ok_or_else(|| format!("missing value in '{}'", field))?;
            if parts.next().is_some() {
                return Err(format!("too many parts in '{}'", field));
            }

            match key {
                "byr" => passport.byr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
                "iyr" => passport.iyr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
                "eyr" => passport.eyr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
                "hgt" => {
                    passport.hgt_set = true;
                    passport.hgt = value.parse().ok();
                }
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                _ => {
                    // don't care about extra fields
                }
            }
        }

        Ok(passport)
    }
}

#[derive(parse_display::FromStr, Debug)]
#[cfg_attr(
    feature = "emit_json",
    derive(Serialize, Deserialize),
    serde(tag = "unit", content = "qty", rename_all = "snake_case")
)]
enum Height {
    #[display("{0}cm")]
    Cm(u32),
    #[display("{0}in")]
    In(u32),
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let valid = parse_newline_sep::<Passport>(input)?
        .filter(|passport| passport.has_northpole_fields())
        .count();
    println!("count (northpole): {}", valid);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let valid = parse_newline_sep::<Passport>(input)?
        .filter(|passport| passport.is_valid())
        .count();
    println!("valid: {}", valid);
    Ok(())
}

#[cfg(feature = "emit_json")]
pub fn emit_json(input: &Path) -> Result<(), Error> {
    use std::io::Write;

    let writer = std::io::stdout();
    let writer = writer.lock();
    let mut writer = std::io::BufWriter::new(writer);

    for passport in parse_newline_sep::<Passport>(input)?.filter(|passport| passport.is_valid()) {
        serde_json::to_writer(&mut writer, &passport)?;
    }
    writer.flush()?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(feature = "emit_json")]
    #[error("json")]
    Json(#[from] serde_json::error::Error),
}
*/

use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

pub enum Color {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "amb" => Self::Amber,
            "blu" => Self::Blue,
            "brn" => Self::Brown,
            "gry" => Self::Gray,
            "grn" => Self::Green,
            "hzl" => Self::Hazel,
            "oth" => Self::Other,
            _ => return Err(String::from(format!("Unknown color {}", value))),
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum UnitType {
    Centimeters,
    Inches,
}

#[derive(thiserror::Error, Debug)]
enum UnitTypeError {
    #[error("unknown unit type")]
    UnknownType,
}

#[derive(Debug)]
pub struct Measurement {
    value: usize,
    unit: UnitType,
}

impl std::str::FromStr for Measurement {
    type Err = Box<dyn std::error::Error>;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.ends_with("cm") {
            let value = value
                .replace("cm", "")
                .parse()
                .context("measurement in cm")?;
            return Ok(Self {
                unit: UnitType::Centimeters,
                value,
            });
        } else if value.ends_with("in") {
            let value = value
                .replace("in", "")
                .parse()
                .context("measurement in inches")?;
            return Ok(Self {
                unit: UnitType::Inches,
                value,
            });
        } else {
            return Err(Box::new(UnitTypeError::UnknownType));
        }
    }
}

#[derive(Error, Debug)]
enum PassportError {
    #[error("Invalid value for: {0}")]
    InvalidValue(String),
}

pub struct Passport {
    pub byr: usize,
    pub iyr: usize,
    pub eyr: usize,
    pub hgt: Measurement,
    pub hcl: String,
    pub ecl: Color,
    pub pid: String,
}

impl std::str::FromStr for Passport {
    type Err = Box<dyn std::error::Error>;
    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let mut fields: HashMap<&str, String> = HashMap::new();
        for field in entry.split_whitespace() {
            let mut tokens = field.split(":");

            let key = tokens.next().ok_or("Missing Key")?;
            let value = String::from(tokens.next().ok_or("Missing value")?);

            fields.insert(key, value);
        }

        let byr: usize = fields
            .get("byr")
            .ok_or("Missing birth year")?
            .parse()
            .context("byr")?;
        let iyr: usize = fields
            .get("iyr")
            .ok_or("Missing issue year")?
            .parse()
            .context("iyr")?;
        let eyr: usize = fields
            .get("eyr")
            .ok_or("Missing expiration year")?
            .parse()
            .context("eyr")?;
        let hgt: Measurement = fields.get("hgt").ok_or("Missing height")?.parse()?;
        let hcl = String::from(fields.get("hcl").ok_or("Missing hair color")?);
        let ecl: Color = fields.get("ecl").ok_or("Missing eye color")?.parse()?;
        let pid = String::from(fields.get("pid").ok_or("Missing passport id")?);

        if byr < 1920 || byr > 2002 {
            return Err(Box::new(PassportError::InvalidValue(format!(
                "byr:{}",
                byr
            ))));
        }

        if iyr < 2010 || iyr > 2020 {
            return Err(Box::new(PassportError::InvalidValue(format!(
                "iyr:{}",
                iyr
            ))));
        }

        if eyr < 2020 || eyr > 2030 {
            return Err(Box::new(PassportError::InvalidValue(format!(
                "eyr:{}",
                eyr
            ))));
        }

        if hgt.unit == UnitType::Centimeters {
            if hgt.value < 150 || hgt.value > 193 {
                return Err(Box::new(PassportError::InvalidValue(format!(
                    "hgt:{:?}",
                    hgt
                ))));
            }
        }

        if hgt.unit == UnitType::Inches {
            if hgt.value < 59 || hgt.value > 76 {
                return Err(Box::new(PassportError::InvalidValue(format!(
                    "hgt:{:?}",
                    hgt
                ))));
            }
        }

        let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        if !re.is_match(&hcl) {
            return Err(Box::new(PassportError::InvalidValue(format!(
                "hcl:{}",
                hcl
            ))));
        }

        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        if !re.is_match(&pid) {
            return Err(Box::new(PassportError::InvalidValue(format!(
                "pid:{}",
                pid
            ))));
        }

        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
        })
    }
}

#[cfg(test)]
mod test_passports {
    use super::*;
    use rstest::*;

    #[rstest(
        example,
        case("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
        case("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"),
        case("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
        case("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"),
        case("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:0123456789 hcl:#a97842 hgt:165cm")
    )]
    fn test_provided_invalid_examples(example: &str) {
        assert!(example.parse::<Passport>().is_err());
    }

    #[rstest(
        example,
        case("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"),
        case("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
        case("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"),
        case("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")
    )]
    fn test_provided_valid_examples(example: &str) {
        assert!(example.parse::<Passport>().is_ok());
    }
}

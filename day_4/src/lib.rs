use regex::Regex;

// This solution is way too long, but I wanted to try out some more fanciness than last time
// I'm specifically a bit upset about check_all_fields, as it feels like there should be a better
// type-level way of doing that, instead of parsing the input in two different ways.

fn run(raw: String) -> u32 {
    let mut valid = 0;
    for pass in raw.rsplit("\n\n") {
        let fields: Vec<String> = pass.split(&[' ', '\n'][..]).map(|f| f.to_owned()).collect();

        if check_all_fields(fields.clone())
            && fields.into_iter().map(Field::from).all(|f| f.valid())
        {
            valid += 1;
        }
    }
    valid
}
impl From<String> for Field {
    fn from(input: String) -> Self {
        if input.len() < 4 {
            return Self::Invalid;
        }
        let (field, value) = input.split_at(3);
        let value = value.strip_prefix(':').unwrap().to_owned();
        match field {
            "hgt" => Self::Hgt(Height { value }),
            "hcl" => Self::Hcl(HairColor { value }),
            "byr" => Self::Byr(BirthYear { value }),
            "eyr" => Self::Eyr(ExpirationYear { value }),
            "ecl" => Self::Ecl(EyeColor { value }),
            "pid" => Self::Pid(PassportId { value }),
            "iyr" => Self::Iyr(IssueYear { value }),
            "cid" => Self::Cid(CountryId),
            _ => Self::Invalid,
        }
    }
}

trait Valid {
    fn valid(&self) -> bool;
}

impl Valid for Field {
    fn valid(&self) -> bool {
        match &self {
            Field::Hgt(height) => height.valid(),
            Field::Hcl(hair_color) => hair_color.valid(),
            Field::Byr(birth_year) => birth_year.valid(),
            Field::Ecl(eye_color) => eye_color.valid(),
            Field::Pid(passport_id) => passport_id.valid(),
            Field::Iyr(issue_year) => issue_year.valid(),
            Field::Cid(country_id) => country_id.valid(),
            Field::Eyr(eye_color) => eye_color.valid(),
            Field::Invalid => false,
        }
    }
}

#[derive(Debug, Clone)]
enum Field {
    Byr(BirthYear),
    Iyr(IssueYear),
    Eyr(ExpirationYear),
    Hgt(Height),
    Hcl(HairColor),
    Ecl(EyeColor),
    Pid(PassportId),
    Cid(CountryId),
    Invalid,
}

#[derive(Debug, Clone)]
struct BirthYear {
    value: String,
}
#[derive(Debug, Clone)]
struct IssueYear {
    value: String,
}
#[derive(Debug, Clone)]
struct ExpirationYear {
    value: String,
}
#[derive(Debug, Clone)]
struct HairColor {
    value: String,
}
#[derive(Debug, Clone)]
struct Height {
    value: String,
}

#[derive(Debug, Clone)]
struct EyeColor {
    value: String,
}
#[derive(Debug, Clone)]
struct PassportId {
    value: String,
}

#[derive(Debug, Clone)]
struct CountryId;
fn year_between(year: &str, earliest: u32, latest: u32) -> bool {
    if let Ok(year_int) = year.parse::<u32>() {
        println!("{:?}", year_int);
        return earliest <= year_int && year_int <= latest;
    }
    false
}

impl Valid for BirthYear {
    fn valid(&self) -> bool {
        year_between(self.value.as_str(), 1920, 2002)
    }
}

impl Valid for IssueYear {
    fn valid(&self) -> bool {
        println!("value: {:?}", self.value);
        year_between(self.value.as_str(), 2010, 2020)
    }
}

impl Valid for ExpirationYear {
    fn valid(&self) -> bool {
        year_between(self.value.as_str(), 2020, 2030)
    }
}

impl Valid for Height {
    fn valid(&self) -> bool {
        let (measurment, unit) = self.value.split_at(self.value.len() - 2);
        let measurment_int = measurment.parse::<u32>().unwrap();
        match unit {
            "cm" => 150 <= measurment_int && measurment_int <= 193,
            "in" => 59 <= measurment_int && measurment_int <= 76,
            _ => false,
        }
    }
}

impl Valid for HairColor {
    fn valid(&self) -> bool {
        Regex::new(r"#[a-z0-9]{6}")
            .unwrap()
            .is_match(self.value.as_str())
    }
}

impl Valid for EyeColor {
    fn valid(&self) -> bool {
        EyeColor::valid_colors().contains(&self.value)
    }
}

impl Valid for PassportId {
    fn valid(&self) -> bool {
        Regex::new(r"\d{9}").unwrap().is_match(self.value.as_str())
    }
}

impl Valid for CountryId {
    fn valid(&self) -> bool {
        true
    }
}

impl EyeColor {
    pub fn valid_colors() -> Vec<String> {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
    }
}

fn check_all_fields(input: Vec<String>) -> bool {
    vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
        .into_iter()
        .all(|field| input.clone().into_iter().any(|i| i.contains(field)))
}
#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs::read_to_string;

    #[test]
    fn it_works() {
        let raw = read_to_string("./src/sample_input.txt").unwrap();
        assert_eq!(run(raw), 2);
    }

    #[test]
    fn valid_values_work() {
        let raw = read_to_string("./src/sample_valid.txt").unwrap();
        assert_eq!(run(raw), 4);
    }
    #[test]
    fn full_input() {
        let raw = read_to_string("./src/input.txt").unwrap();
        assert_eq!(run(raw), 182);
    }
    #[test]
    fn valid_passport_id() {
        let pass = PassportId {
            value: "001234567".to_owned(),
        };
        assert_eq!(pass.valid(), true);

        let pass = PassportId {
            value: "00f234567".to_owned(),
        };

        assert_eq!(pass.valid(), false);

        let pass = PassportId {
            value: "55".to_owned(),
        };
        assert_eq!(pass.valid(), false);

        assert!(PassportId {
            value: "087499704".to_owned()
        }
        .valid());
    }

    #[test]
    fn valid_issue_year() {
        assert!(Field::from("iyr:2012".to_owned()).valid());
    }

    #[test]
    fn height_valid() {
        assert!(Height {
            value: "190cm".to_owned(),
        }
        .valid());

        assert!(!Height {
            value: "2cm".to_owned(),
        }
        .valid());

        assert!(Height {
            value: "60in".to_owned()
        }
        .valid());

        assert!(!Height {
            value: "1000in".to_owned()
        }
        .valid());
    }

    #[test]
    fn eye_color_validation() {
        assert!(EyeColor {
            value: "blu".to_owned(),
        }
        .valid());

        assert!(!EyeColor {
            value: "asdf".to_owned()
        }
        .valid());
    }

    #[test]
    fn foo() {
        assert!(!Field::from("".to_owned()).valid());
    }

    #[test]
    fn from() {
        println!("{:?}", Field::from("pid:087499704".to_owned()));
        assert!(Field::from("pid:087499704".to_owned()).valid());
        assert!(Field::from("hgt:74in".to_owned()).valid());
        assert!(Field::from("ecl:grn".to_owned()).valid());
        assert!(Field::from("iyr:2012".to_owned()).valid());
        assert!(Field::from("eyr:2030".to_owned()).valid());
        assert!(Field::from("byr:1980".to_owned()).valid());
        assert!(Field::from("hcl:#623a2f".to_owned()).valid());
    }
}

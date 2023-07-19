use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
struct TryDialect;

impl Dialect for TryDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch.is_ascii_digit()
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!("SELECT location name , total_cases, new_cases, total_deaths, new_deaths \
        FROM {url} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5");
    sql
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;

    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&TryDialect::default(), &example_sql()).is_ok());
    }
}

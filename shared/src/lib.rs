use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive_int() {
        let res = parse_positive_int("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);

        let res = parse_positive_int("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

        let res = parse_positive_int("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "0".to_string());
    }
}

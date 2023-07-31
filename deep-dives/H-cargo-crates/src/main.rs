fn main() {
    println!("back!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn regex_test() {
        let re = regex::Regex::new(
            r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
",
        )
        .unwrap();

        let caps = re.captures("2010-03-14").unwrap();
        assert_eq!("2010", &caps["year"]);
        assert_eq!("03", &caps["month"]);
        assert_eq!("14", &caps["day"]);
    }
}

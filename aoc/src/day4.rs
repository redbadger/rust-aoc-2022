use regex::Regex;

pub fn run() -> anyhow::Result<()> {
    let file = match std::fs::read_to_string("input/day-4.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Could not open the file. Error: {}", error),
    };

    let parser = Parser::new();

    // let mut count = 0;
    //
    // for line in file.lines() {
    //     let (first, second) = parse_line(line)?;

    //     if one_contains_other(first, second) {
    //         count += 1;
    //     }
    // }

    let count = file
        .lines()
        .filter_map(|line| parser.parse_line(line).ok())
        .filter(|&(first, second)| one_contains_other(first, second))
        .count();

    println!("Total overlaps: {count}");

    Ok(())
}

fn one_contains_other(first: (u32, u32), second: (u32, u32)) -> bool {
    is_contained(first, second) || is_contained(second, first)
}

fn is_contained(container: (u32, u32), subrange: (u32, u32)) -> bool {
    container.0 <= subrange.0 && container.1 >= subrange.1
}

struct Parser {
    regex: Regex,
}

impl Parser {
    fn new() -> Self {
        Self {
            regex: Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap(),
        }
    }

    fn parse_line(&self, line: &str) -> anyhow::Result<((u32, u32), (u32, u32))> {
        let Some(caps) = self.regex.captures(line) else {
        anyhow::bail!("Line does not match!");
    };

        let a_low = caps.get(1).expect("expected 1 capture").as_str().parse()?;
        let a_high = caps.get(2).expect("expected 2 captures").as_str().parse()?;
        let b_low = caps.get(3).expect("expected 3 captures").as_str().parse()?;
        let b_high = caps.get(4).expect("expected 4 captures").as_str().parse()?;

        Ok(((a_low, a_high), (b_low, b_high)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let parser = Parser::new();

        let actual = parser.parse_line("23-27,13-67").unwrap();
        let expected = ((23, 27), (13, 67));

        assert_eq!(actual, expected);
    }

    #[test]
    fn bad_line() {
        let parser = Parser::new();

        assert!(parser.parse_line("23-x,13-67").is_err());
    }

    #[test]
    fn no_overlap() {
        let overlap = is_contained((1, 3), (4, 6));
        let expected = false;

        assert_eq!(overlap, expected);
    }

    #[test]
    fn container() {
        let overlap = is_contained((1, 5), (2, 4));
        let expected = true;

        assert_eq!(overlap, expected);
    }

    #[test]
    fn bottom_edge() {
        let overlap = is_contained((1, 5), (1, 4));
        let expected = true;

        assert_eq!(overlap, expected);
    }

    #[test]
    fn top_edge() {
        let overlap = is_contained((1, 5), (3, 5));
        let expected = true;

        assert_eq!(overlap, expected);
    }

    #[test]
    fn cross_over() {
        let overlap = is_contained((1, 5), (3, 6));
        let expected = false;

        assert_eq!(overlap, expected);
    }

    #[test]
    fn symmetry() {
        let overlap = one_contains_other((3, 5), (1, 5));
        let expected = true;

        assert_eq!(overlap, expected);
    }
}

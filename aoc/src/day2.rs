use std::cmp::Ordering;

type Error = Box<dyn std::error::Error>;

pub fn run() -> Result<(), Error> {
    let file = std::fs::read_to_string("input/day-2.txt")?;
    let mut total: u32 = 0;
    for line in file.lines() {
        total += score_game_string(line)? as u32;
    }
    println!("total: {total}");
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use std::cmp::Ordering::*;
        let cmp = match (self, other) {
            (Play::Rock, Play::Paper) => Less,
            (Play::Rock, Play::Scissors) => Greater,
            (Play::Paper, Play::Rock) => Greater,
            (Play::Paper, Play::Scissors) => Less,
            (Play::Scissors, Play::Rock) => Less,
            (Play::Scissors, Play::Paper) => Greater,
            _ => Equal,
        };
        Some(cmp)
    }
}

fn parse_play(play: &str) -> Result<Play, String> {
    let ok = match play {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => return Err(format!("unknown: {play}")),
    };
    Ok(ok)
}

fn score_game(p1: Play, p2: Play) -> u8 {
    let play_score = match p2 {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };
    let game_score = match p2.partial_cmp(&p1).unwrap() {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    };
    play_score + game_score
}

fn score_game_string(game: &str) -> Result<u8, String> {
    // let (p1, p2) = match game.split_once(' ') {
    //     Some(ps) => ps,
    //     None => return Err(format!("bad game: {game}")),
    // };
    let (p1, p2) = game
        .split_once(' ')
        .ok_or_else(|| format!("bad game: {game}"))?;
    let p1 = parse_play(p1)?;
    let p2 = match parse_play(p2) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    Ok(score_game(p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_move() {
        let actual = parse_play("A").unwrap();
        let expect = Play::Rock;

        assert_eq!(actual, expect);

        let actual = parse_play("Y").unwrap();
        let expect = Play::Paper;

        assert_eq!(actual, expect);
    }

    #[test]
    fn cannot_parse_p1_bad_move() {
        let actual = parse_play("Q");
        assert!(actual.is_err());
    }

    #[test]
    fn can_score_game() {
        let examples = &[
            (Play::Rock, Play::Paper, 8),
            (Play::Scissors, Play::Rock, 7),
            (Play::Rock, Play::Scissors, 3),
        ];
        for &(p1, p2, expect) in examples {
            let actual: u8 = score_game(p1, p2);
            assert_eq!(actual, expect);
        }
    }

    #[test]
    fn can_score_game_string() {
        let actual = score_game_string("A X").unwrap();
        let expect = 4;
        assert_eq!(actual, expect);
    }
}

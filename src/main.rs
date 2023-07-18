fn main() {
    let file = match std::fs::read_to_string("day-1-1.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Could not open the file. Error: {}", error),
    };

    let max = max_elf(file, 3);

    println!("Top 3 elves: {max}");
}

fn max_elf(file: String, n_max: usize) -> usize {
    let mut elves: Vec<usize> = file
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .fold(0, |sum, food| sum + food.parse::<usize>().unwrap_or(0))
        })
        .collect();

    // for line in file.split('\n') {
    //     if line.is_empty() {
    //         elves.push(elf);

    //         elf = 0;
    //     } else {
    //         let calories: usize = line.parse().unwrap();
    //         elf += calories;
    //     }
    // }
    //
    // elves.push(elf);

    elves.sort();
    elves.reverse();

    elves.iter().take(n_max).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file() {
        let actual = max_elf("".to_string(), 1);
        let expected = 0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_snack() {
        let actual = max_elf("5600".to_string(), 1);
        let expected = 5600;

        assert_eq!(actual, expected);
    }

    #[test]
    fn few_snacks() {
        let actual = max_elf("5600\n200\n400".to_string(), 1);
        let expected = 6200;

        assert_eq!(actual, expected);
    }

    #[test]
    fn couple_elves() {
        let actual = max_elf("5600\n200\n400\n\n10000".to_string(), 1);
        let expected = 10000;

        assert_eq!(actual, expected);
    }

    #[test]
    fn couple_elves_top_2() {
        let actual = max_elf("5600\n200\n400\n\n10000".to_string(), 2);
        let expected = 16200;

        assert_eq!(actual, expected);
    }
}

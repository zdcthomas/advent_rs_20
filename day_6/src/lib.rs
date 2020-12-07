fn run_everyone(input: &str) -> u32 {
    input.split("\n\n").map(|g| unique_everyone(g)).sum()
}

fn run(input: &str) -> u32 {
    input.split("\n\n").map(|g| unique(g)).sum()
}

fn unique(input: &str) -> u32 {
    let mut chars = input.replace('\n', "").chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    chars.len() as u32
}

fn unique_everyone(input: &str) -> u32 {
    let mut total = 0;
    for answer in input.split('\n').next().unwrap().chars() {
        if input.trim_end().split('\n').all(|a| a.contains(answer)) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unique_test() {
        assert_eq!(unique("abc"), 3);
        assert_eq!(unique("aaaaaa"), 1);
        assert_eq!(unique("abac"), 3);
        assert_eq!(unique("b"), 1);
        assert_eq!(unique("aba"), 2);
    }

    #[test]
    fn sample_works() {
        let result = run(std::fs::read_to_string("./src/sample_input.txt")
            .unwrap()
            .as_str());
        assert_eq!(result, 11);
    }

    #[test]
    fn full_works() {
        let result = run(std::fs::read_to_string("./src/input.txt").unwrap().as_str());
        assert_eq!(result, 6551);
    }

    #[test]
    fn full_works_part_two_sample() {
        let result = run_everyone(
            std::fs::read_to_string("./src/sample_input.txt")
                .unwrap()
                .as_str(),
        );
        assert_eq!(result, 6);
    }
    #[test]
    fn full_works_part_two() {
        let result = run_everyone(std::fs::read_to_string("./src/input.txt").unwrap().as_str());
        assert_eq!(result, 6);
    }
}

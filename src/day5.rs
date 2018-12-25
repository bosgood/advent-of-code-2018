// #[aoc_generator(day5)]
// pub fn generator(input: &str) -> String {

// }

// Reacts on the given polymer chain and returns the most immediate result,
// and whether a result was necessary
fn polymer_destruction(p: &str) -> (String, bool) {
    if p.len() < 2 {
        return (String::from(p), false);
    }

    let chars: Vec<char> = p.chars().collect();
    let l = p.len();
    for i in 0..l - 1 {
        // println!("index {}, char {}", i, chars[i]);
        let e1 = chars[i];
        let e2 = chars[i + 1];
        if reacts(&e1, &e2) {
            // println!("found reaction: {}{}", e1, e2);
            let prefix: String = chars[0..i].into_iter().collect();
            let suffix: String = chars[i + 2..l].into_iter().collect();
            // println!("prefix: {}, suffix: {}", prefix, suffix);
            return ([prefix, suffix].concat(), true);
        }
    }

    (String::from(p), false)
}

// Determines if the two polymer elements will react
fn reacts(c1: &char, c2: &char) -> bool {
    c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

pub fn polymer_reaction(input: &str) -> String {
    let mut reaction = polymer_destruction(input);
    while reaction.1 {
        reaction = polymer_destruction(&reaction.0);
    }
    reaction.0
}

#[aoc(day5, part1)]
pub fn day5_part1(input: &str) -> usize {
    polymer_reaction(&input).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    // dabAcCaCBAcCcaDA  The first 'cC' is removed.
    // dabAaCBAcCcaDA    This creates 'Aa', which is removed.
    // dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
    // dabCBAcaDA        No further actions can be taken.
    #[test]
    fn test_polymer_destruction_1() {
        assert_eq!(
            polymer_destruction("dabAcCaCBAcCcaDA"),
            (String::from("dabAaCBAcCcaDA"), true),
        );
    }
    #[test]
    fn test_polymer_destruction_2() {
        assert_eq!(
            polymer_destruction("dabAaCBAcCcaDA"),
            (String::from("dabCBAcCcaDA"), true),
        );
    }
    #[test]
    fn test_polymer_destruction_3() {
        assert_eq!(
            polymer_destruction("dabCBAcCcaDA"),
            (String::from("dabCBAcaDA"), true),
        );
    }
    #[test]
    fn test_polymer_destruction_4() {
        assert_eq!(
            polymer_destruction("dabCBAcaDA"),
            (String::from("dabCBAcaDA"), false),
        );
    }

    #[test]
    fn test_day5_part1() {
        assert_eq!(day5_part1("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn test_reacts() {
        assert_eq!(reacts(&'a', &'A'), true);
        assert_eq!(reacts(&'B', &'b'), true);
        assert_eq!(reacts(&'B', &'B'), false);
        assert_eq!(reacts(&'a', &'a'), false);
    }
}

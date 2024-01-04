use std::collections::HashSet;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn contains_three_plus_vowels(chars: &[char]) -> bool {
    chars.iter().filter(|c| VOWELS.contains(c)).count() >= 3
}

fn contains_double(chars: &Vec<char>) -> bool {
    for i in 0..=chars.len() - 2 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn contains_disallowed_pair(chars: &Vec<char>) -> bool {
    for i in 0..=chars.len() - 2 {
        if chars[i] == 'a' && chars[i + 1] == 'b'
            || chars[i] == 'c' && chars[i + 1] == 'd'
            || chars[i] == 'p' && chars[i + 1] == 'q'
            || chars[i] == 'x' && chars[i + 1] == 'y'
        {
            return true;
        }
    }
    false
}

fn is_nice_part_1(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();

    contains_three_plus_vowels(&chars)
        && contains_double(&chars)
        && !contains_disallowed_pair(&chars)
}

fn is_nice_part_2(input: &str) -> bool {
    let mut pairs: HashSet<(char, char)> = HashSet::new();
    let mut skip_next_if_this_pair = (' ', ' ');
    let chars: Vec<char> = input.chars().collect();

    // False match: qpnxkuldeiituggg
    let mut pass_a = false;
    for i in 0..=chars.len() - 2 {
        let this_pair = (chars[i], chars[i + 1]);
        if skip_next_if_this_pair == this_pair {
            skip_next_if_this_pair = (' ', ' ');
            continue;
        }
        skip_next_if_this_pair = this_pair;

        if pairs.contains(&this_pair) {
            pass_a = true;
            break;
        }
        pairs.insert(this_pair);
    }

    let mut pass_b = false;
    for i in 0..=chars.len() - 3 {
        if chars[i] == chars[i + 2] {
            pass_b = true;
            break;
        }
    }

    pass_a && pass_b
}

pub fn solve05(input: &[String]) -> (i128, i128) {
    (
        input.iter().filter(|&s| is_nice_part_1(s)).count() as i128,
        input.iter().filter(|&s| is_nice_part_2(s)).count() as i128,
    )
}

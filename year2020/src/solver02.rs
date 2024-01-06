use regex::Regex;

pub fn solve02(input: &[String]) -> (i128, i128) {
    let mut num_valid_passwords_part_1 = 0;
    let mut num_valid_passwords_part_2 = 0;
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

    for line in input {
        let captures = re.captures(line).unwrap();
        let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let character = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let password = captures.get(4).unwrap().as_str();

        let num_matches = password.matches(character).count();
        if num_matches >= min && num_matches <= max {
            num_valid_passwords_part_1 += 1;
        }

        let mut password_chars: Vec<char> = password.chars().collect();
        password_chars.insert(0, ' '); // Handle 1-based index.
        if password_chars[min] != password_chars[max]
            && (password_chars[min] == character || password_chars[max] == character)
        {
            num_valid_passwords_part_2 += 1;
        }
    }
    (num_valid_passwords_part_1, num_valid_passwords_part_2)
}

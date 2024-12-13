use regex::Regex;

fn get_offset(offset_str: &str) -> i32 {
    offset_str.parse().expect("Not a valid number")
}

fn get_register_index(register_str: &str) -> usize {
    match register_str {
        "a" => 0,
        "b" => 1,
        _ => panic!("Invalid input"),
    }
}

pub fn solve23(input: &[String]) -> (i128, i128) {
    (solve(input, 0) as i128, solve(input, 1) as i128)
}

pub fn solve(input: &[String], initial_a: i32) -> i128 {
    let re_jmp = Regex::new(r"^jmp ([+-]\d+)$").unwrap();
    let re_jie = Regex::new(r"^jie ([a-b]), ([+-]\d+)$").unwrap();
    let re_jio = Regex::new(r"^jio ([a-b]), ([+-]\d+)$").unwrap();

    let mut registers = [initial_a, 0];
    let mut instruction_pointer: i32 = 0;

    while instruction_pointer >= 0 && instruction_pointer < input.len() as i32 {
        let instruction = &input[instruction_pointer as usize] as &str;
        instruction_pointer += 1;
        match instruction {
            "hlf a" => registers[0] /= 2,
            "hlf b" => registers[1] /= 2,
            "tpl a" => registers[0] *= 3,
            "tpl b" => registers[1] *= 3,
            "inc a" => registers[0] += 1,
            "inc b" => registers[1] += 1,

            _ if re_jmp.is_match(instruction) => {
                let caps = re_jmp.captures(instruction).unwrap();
                let offset = get_offset(&caps[1]);
                instruction_pointer += offset - 1;
            }
            _ if re_jie.is_match(instruction) => {
                let caps = re_jie.captures(instruction).unwrap();
                let register_index = get_register_index(&caps[1]);
                let offset = get_offset(&caps[2]);
                if registers[register_index] % 2 == 0 {
                    instruction_pointer += offset - 1;
                }
            }
            _ if re_jio.is_match(instruction) => {
                let caps = re_jio.captures(instruction).unwrap();
                let register_index = get_register_index(&caps[1]);
                let offset = get_offset(&caps[2]);
                if registers[register_index] == 1 {
                    instruction_pointer += offset - 1;
                }
            }
            _ => {
                panic!("NO MATCH on {instruction}");
            }
        }
    }
    registers[1] as i128
}

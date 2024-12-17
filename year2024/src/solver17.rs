#[derive(Clone, Debug)]
struct Computer {
    a: u64,
    program: Vec<u64>,
    is_sample: bool,
    output: Vec<u64>,
}

#[allow(dead_code)]
impl Computer {
    // Inspection of the sample and actual program shows that it:
    // - Manipulates the registers over multiple steps.
    // - Outputs a value.
    // - Jumps to instruction pointer zero if a is non-zero.
    fn execute_program_once(&mut self) -> bool {
        if !self.is_sample {
            let partial = (self.a % 8) ^ 7;
            let c = self.a / (2u64.pow(partial as u32) as u64); // 75
            let output = ((partial ^ c) ^ 7) % 8; // 40 17
            self.a = self.a / 8; // 03

            // let output = self.b % 8;
            self.output.push(output); // 55
        } else {
            // TODO Change back for part_one sample.
            // self.a = self.a / 2; // 01
            self.a = self.a / 8; // 03
            self.output.push(self.a % 8); // 54
            // 30 no-op
        }
        self.a != 0
    }
}

fn parse_input(input: &[String]) -> Computer {
    // TODO: Parse input file.
    // TODO: Replace is_part_two with something better.
    let is_sample = false;
    if is_sample {
        // Part one sample
        // Computer { a: 729, b: 0, c: 0, program: vec![0,1,5,4,3,0], is_sample: true, output: vec![]}
        // Part two sample
        Computer { a: 2024, program: vec![0,3,5,4,3,0], is_sample: true, output: vec![]}
    } else {
        Computer { a: 62769524, program: vec![2,4,1,7,7,5,0,3,4,0,1,7,5,5,3,0], is_sample: false, output: vec![]}
    }
}


fn solve(input: &[String], a: u64) {
    let mut computer = parse_input(input);
    computer.a = a; // Goes wrong!
    computer.output.clear();
    println!("DO: a = {}", computer.a);
    while computer.execute_program_once() {
        println!("    {computer:?}")
    }
    println!("SOLUTION ONE: {computer:?}\n");
}

fn find_a(computer: &mut Computer, min_a: u64, max_a: u64, prev_a: u64, target_index: usize) -> Vec<(u64, u64)> {
    let mut candidates = vec![];
    for a in min_a..=max_a {
        computer.a = a;
        computer.output.clear();
        let _not_halted = computer.execute_program_once();
        if computer.output[0] == computer.program[target_index - 1] {
            // println!("        Index {target_index}    found {a} -> {computer:?}, offset from min_a {}", a - min_a);
            if prev_a == computer.a {
                // There could be multiple solutions here.
                println!("        Index {target_index}    found {a} -> {computer:?}, offset from min_a {}, matches prev_a", a - min_a);
                candidates.push((a * 8, a));
            }
        }
    }
    candidates
}

pub fn solve17(input: &[String]) -> (i128, i128) {
    let mut computer = parse_input(input);
    let solution_one = format!("{:?}", computer.output);
    println!("{solution_one}");

    let mut target_index = computer.program.len();
    let mut candidates = vec![(1, 0)];
    while target_index != 0 {
        println!("\nSearch for index {target_index}, num candidates = {}, candidates: {candidates:?}", candidates.len());
        let mut new_candidates = vec![];
        for (min_a, prev_a) in &candidates {
            // let max_a = min_a * 8;
            let max_a = min_a + 8;
            println!("    Candidate: Search for index {target_index} in range of a {min_a} - {max_a}, prev_a {prev_a}");
            let mut more_a = find_a(&mut computer, *min_a, max_a, *prev_a, target_index);
            new_candidates.append(&mut more_a);
        }

        target_index -= 1;
        candidates = new_candidates;
    }

    // Answer: 258394985014171

    (0_i128, 0_i128)
}

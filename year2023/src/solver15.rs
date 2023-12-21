fn solve_part_1(input: Vec<&str>) -> usize {
    let mut output = 0;
    for s in input {
        output += hash(s);
    }
    output
}

fn solve_part_2(input: Vec<&str>) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for s in &input {
        let chars: Vec<char> = s.chars().collect();
        let (focal_length, label) = if chars[chars.len() - 1] == '-' {
            (1000, &s[0..s.len() - 1])
        } else {
            (
                chars[chars.len() - 1].to_digit(10).unwrap() as usize,
                &s[0..s.len() - 2],
            )
        };

        let current_box = &mut boxes[hash(label)];

        if focal_length == 1000 {
            // The box contains zero or one of label. If present remove it.
            current_box.retain(|&x| x.0 != label);
        } else {
            // The box contains zero or one of label. If present, update the focal length. If not
            // present, add the lens at the back of the list.
            let mut replaced = false;

            for i in 0..current_box.len() {
                if current_box[i].0 == label {
                    current_box[i] = (label, focal_length);
                    replaced = true;
                }
            }
            if !replaced {
                current_box.push((label, focal_length));
            }
        }
    }

    let mut focusing_power = 0;
    for (box_id, current_box) in boxes.iter().enumerate() {
        for (slot_id, lens) in current_box.iter().enumerate() {
            focusing_power += (1 + box_id) * (1 + slot_id) * lens.1;
        }
    }

    focusing_power
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

// fn print_boxes(boxes: &Vec<Vec<(&str, usize)>>) {
//     println!("BOXES");
//     for box_id in 0..256 {
//         if boxes[box_id].len() != 0 {
//             println!("  {} -> {:?}", box_id, boxes[box_id]);
//         }
//     }
// }

pub fn solve15(input: &[String]) -> (i128, i128) {
    let split_1: Vec<&str> = input[0].split(',').collect();

    (
        solve_part_1(split_1.clone()) as i128,
        solve_part_2(split_1) as i128,
    )
}

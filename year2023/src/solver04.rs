pub fn solve04(input: &[String]) -> (i128, i128) {
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let mut num_cards = vec![1; input.len()];

    for (card_id, line) in input.iter().enumerate() {
        let split_1: Vec<&str> = line.split(':').collect();
        let split_2: Vec<&str> = split_1[1].split('|').collect();

        let winning_numbers: Vec<i32> = split_2[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let my_numbers: Vec<i32> = split_2[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut line_points = 0;
        let mut line_total = 0;

        for my_number in my_numbers {
            if winning_numbers.contains(&my_number) {
                line_total += 1;
                if line_points == 0 {
                    line_points = 1;
                } else {
                    line_points *= 2;
                }
            }
        }
        total_part_1 += line_points;

        for c in (card_id + 1)..=(card_id + line_total) {
            num_cards[c] += num_cards[card_id];
        }
        total_part_2 += num_cards[card_id];
    }

    (i128::from(total_part_1), i128::from(total_part_2))
}

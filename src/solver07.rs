use std::collections::HashMap;

fn map_card_to_score(card: char, is_part_1: bool) -> u64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if is_part_1 {
                11
            } else {
                1
            }
        }
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u64,
    }
}

fn decider_hand(hand: &str, is_part_1: bool) -> u64 {
    let hand_chars = hand.chars().collect::<Vec<char>>();
    map_card_to_score(hand_chars[4], is_part_1)
        + map_card_to_score(hand_chars[3], is_part_1) * 100
        + map_card_to_score(hand_chars[2], is_part_1) * 10000
        + map_card_to_score(hand_chars[1], is_part_1) * 1000000
        + map_card_to_score(hand_chars[0], is_part_1) * 100000000
}

fn score_sorted_c(sorted_c: &Vec<u64>) -> u64 {
    println!("INPUT: {:?}", sorted_c);
    if *sorted_c == [5] {
        7
    } else if *sorted_c == [1, 4] {
        6
    } else if *sorted_c == [2, 3] {
        5
    } else if *sorted_c == [1, 1, 3] {
        4
    } else if *sorted_c == [1, 2, 2] {
        3
    } else if *sorted_c == [1, 1, 1, 2] {
        2
    } else if *sorted_c == [1, 1, 1, 1, 1] {
        1
    } else {
        panic!("ERROR");
    }
}

fn score_hand_1(hand: &str, counter: HashMap<char, u64>) -> u64 {
    let mut sorted_c: Vec<u64> = counter.values().cloned().collect();
    sorted_c.sort();
    10000000000 * score_sorted_c(&sorted_c) + decider_hand(hand, true)
}

fn score_hand_2(hand: &str, counter: &mut HashMap<char, u64>) -> u64 {
    // Special case for JJJJJ.
    let sorted_c = if hand == "JJJJJ" {
        let mut output: Vec<u64> = counter.values().cloned().collect();
        output.sort();
        output
    } else {
        let num_jacks = if counter.contains_key(&'J') {
            counter[&'J']
        } else {
            0
        };
        counter.remove(&'J');
        let mut output: Vec<u64> = counter.values().cloned().collect();
        output.sort();
        *output.last_mut().unwrap() += num_jacks;
        output
    };

    10000000000 * score_sorted_c(&sorted_c) + decider_hand(hand, false)
}

pub fn solve07(input: Vec<String>) -> (i128, i128) {
    let mut hands: Vec<(&str, HashMap<char, u64>, u64)> = Vec::new();

    for line in &input {
        let split_1: Vec<&str> = line.split(' ').collect();

        // Count the number of each card.
        let mut counter: HashMap<char, u64> = HashMap::new();
        for c in split_1[0].chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        hands.push((split_1[0], counter, split_1[1].parse().unwrap()));
    }

    let mut scored_hands: Vec<(u64, u64, u64)> = Vec::new();

    for (hand, mut counter, bid) in hands {
        println!("{:?}", hand);
        scored_hands.push((
            bid,
            score_hand_1(hand, counter.clone()),
            score_hand_2(hand, &mut counter),
        ))
    }

    scored_hands.sort_by_key(|k| k.1);
    let mut winnings_1 = 0;
    let mut rank = 0;
    for (bid, _, _) in &scored_hands {
        rank += 1;
        winnings_1 += bid * rank;
    }

    scored_hands.sort_by_key(|k| k.2);
    let mut winnings_2 = 0;
    let mut rank = 0;
    for (bid, _, _) in &scored_hands {
        rank += 1;
        winnings_2 += bid * rank;
    }

    (winnings_1 as i128, winnings_2 as i128)
}

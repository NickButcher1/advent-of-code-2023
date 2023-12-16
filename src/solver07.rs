use std::collections::HashMap;

fn tiebreak_score_for_card(card: char, is_part_1: bool) -> u64 {
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

fn score_for_hand(sorted_c: &Vec<u64>) -> u64 {
    let score = if *sorted_c == [5] {
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
    };
    score * 10_000_000_000
}

fn score_for_tiebreak(hand: &str, is_part_1: bool) -> u64 {
    let hand_chars = hand.chars().collect::<Vec<char>>();
    tiebreak_score_for_card(hand_chars[4], is_part_1)
        + tiebreak_score_for_card(hand_chars[3], is_part_1) * 100
        + tiebreak_score_for_card(hand_chars[2], is_part_1) * 10000
        + tiebreak_score_for_card(hand_chars[1], is_part_1) * 1_000_000
        + tiebreak_score_for_card(hand_chars[0], is_part_1) * 100_000_000
}

fn collect_and_sort(counter: &HashMap<char, u64>) -> Vec<u64> {
    let mut sorted_c: Vec<u64> = counter.values().cloned().collect();
    sorted_c.sort();
    sorted_c
}

fn score_hand_1(hand: &str, counter: &HashMap<char, u64>) -> u64 {
    score_for_hand(&collect_and_sort(counter)) + score_for_tiebreak(hand, true)
}

fn score_hand_2(hand: &str, counter: &mut HashMap<char, u64>) -> u64 {
    let sorted_c = if hand == "JJJJJ" {
        collect_and_sort(counter)
    } else {
        let num_jacks = if counter.contains_key(&'J') {
            counter[&'J']
        } else {
            0
        };
        counter.remove(&'J');
        let mut output: Vec<u64> = collect_and_sort(counter);
        *output.last_mut().unwrap() += num_jacks;
        output
    };

    score_for_hand(&sorted_c) + score_for_tiebreak(hand, false)
}

fn count_winnings(scored_hands: &Vec<(u64, u64, u64)>) -> u64 {
    let mut winnings = 0;
    let mut rank = 0;
    for (bid, _, _) in scored_hands {
        rank += 1;
        winnings += bid * rank;
    }
    winnings
}
pub fn solve07(input: Vec<String>) -> (i128, i128) {
    let mut scored_hands: Vec<(u64, u64, u64)> = Vec::new();

    for line in &input {
        let split_1: Vec<&str> = line.split(' ').collect();

        let mut counter: HashMap<char, u64> = HashMap::new();
        for c in split_1[0].chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        scored_hands.push((
            split_1[1].parse().unwrap(),
            score_hand_1(split_1[0], &counter.clone()),
            score_hand_2(split_1[0], &mut counter),
        ));
    }

    scored_hands.sort_by_key(|k| k.1);
    let winnings_1 = count_winnings(&scored_hands);

    scored_hands.sort_by_key(|k| k.2);
    let winnings_2 = count_winnings(&scored_hands);

    (winnings_1 as i128, winnings_2 as i128)
}

from collections import Counter

f = open("input/input07")
# f = open("input/input07-sample")

map_card_to_score_1 = {
    "A": 14,
    "K": 13,
    "Q": 12,
    "J": 11,
    "T": 10,
    "9": 9,
    "8": 8,
    "7": 7,
    "6": 6,
    "5": 5,
    "4": 4,
    "3": 3,
    "2": 2,
}

map_card_to_score_2 = {
    "A": 14,
    "K": 13,
    "Q": 12,
    "T": 10,
    "9": 9,
    "8": 8,
    "7": 7,
    "6": 6,
    "5": 5,
    "4": 4,
    "3": 3,
    "2": 2,
    "J": 1,
}

def score_sorted_c(sorted_c):
    if sorted_c == [5]:
        score = 7
    elif sorted_c == [1, 4]:
        score = 6
    elif sorted_c == [2, 3]:
        score = 5
    elif sorted_c == [1, 1, 3]:
        score = 4
    elif sorted_c == [1, 2, 2]:
        score = 3
    elif sorted_c == [1, 1, 1, 2]:
        score = 2
    elif sorted_c == [1, 1, 1, 1, 1]:
        score = 1
    else:
        print("ERROR")
    return score

def decider_hand(hand, map_card_to_score):
    return map_card_to_score[hand[4]] + \
            map_card_to_score[hand[3]] * 100 + \
            map_card_to_score[hand[2]] * 10000 + \
            map_card_to_score[hand[1]] * 1000000 + \
            map_card_to_score[hand[0]] * 100000000

def score_hand_1(hand, c):
    sorted_c = sorted(c.values())

    return 10000000000 * score_sorted_c(sorted_c) + decider_hand(hand, map_card_to_score_2)

def score_hand_2(hand, c):
    # Special case for JJJJJ.
    if hand == "JJJJJ":
        sorted_c = sorted(c.values())
    else:
        num_jacks = c['J']
        del(c['J'])
        sorted_c = sorted(c.values())
        sorted_c[-1] += num_jacks

    return 10000000000 * score_sorted_c(sorted_c) + decider_hand(hand, map_card_to_score_2)

hands = []

for line in f.readlines():
    line = line.rstrip().split()
    hand = line[0]
    bid = int(line[1])
    c = Counter(hand)
    hands.append((hand, c, bid))

scored_hands = []
for hand, c, bid in hands:
    score_1 = score_hand_1(hand, c)
    score_2 = score_hand_2(hand, c)
    scored_hands.append((bid, score_1, score_2))

scored_hands.sort(key = lambda x: x[1])

winnings_1 = 0
rank = 0
for bid, _, _ in scored_hands:
    rank += 1
    winnings_1 += (bid * rank)

scored_hands.sort(key = lambda x: x[2])

winnings_2 = 0
rank = 0
for bid, _, _ in scored_hands:
    rank += 1
    winnings_2 += (bid * rank)

print("Part 1: " + str(winnings_1))
print("Part 2: " + str(winnings_2))

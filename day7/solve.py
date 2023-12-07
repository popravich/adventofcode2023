from collections import Counter

_test_data = """
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"""


_weights_part1 = {c: w for w, c in enumerate('23456789TJQKA')}

_weights_part2 = {c: w for w, c in enumerate('J23456789TQKA')}


def main():
    data = _test_data.strip()
    with open('./input.txt', 'rt') as f:
        data = f.read().strip()

    # part 1
    with_weigts = []
    for line in data.splitlines():
        hand, bid = line.split()
        counts = Counter(hand)

        hand_type = [x[1] for x in counts.most_common(2)]
        hand_weights = [_weights_part1[c] for c in hand]
        sort_key = tuple(hand_type + hand_weights)

        with_weigts.append((sort_key, int(bid)))
    result1 = sum(k * bid for k, (_, bid) in enumerate(sorted(with_weigts), 1))
    print("Result 1:", result1)

    # part 2
    with_weigts = []
    for line in data.splitlines():
        hand, bid = line.split()
        hand_type = get_hand_type(hand)
        hand_weights = [_weights_part2[c] for c in hand]
        sort_key = tuple(hand_type + hand_weights)

        with_weigts.append((sort_key, int(bid), hand))
    # for i, (k, bid, hand) in enumerate(sorted(with_weigts), 1):
    #     print(f'{hand:<5} {bid:>5}*{i:<4} {"J" in hand!s:>5}     {k}')
    result1 = sum(k * bid for k, (_, bid, _) in enumerate(sorted(with_weigts), 1))
    print("Result 2:", result1)


def get_hand_type(hand):
    counts = Counter(hand)
    if 'J' in counts and counts['J'] < 5:
        j_count = counts.pop('J')
        new_hand = hand.replace('J', counts.most_common()[0][0])
        assert new_hand != hand, (new_hand, hand)
        counts = Counter(new_hand)
    return [x[1] for x in counts.most_common(2)]



if __name__ == "__main__":
    main()

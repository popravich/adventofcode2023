from collections import defaultdict

_test_data = """
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"""


def main():
    # data = _test_data.strip()
    with open('./input.txt', 'rt') as f:
        data = f.read().strip()

    result1 = result2 = 0
    
    # part 1
    for line in data.splitlines():
        head, _, card_numbers = line.partition('|')
        _, _, winning_numbers = head.partition(':')
        winning = set(map(
            lambda x: int(x), filter(None, winning_numbers.split(' ')),
        ))
        card = set(map(
            lambda x: int(x), filter(None, card_numbers.split(' '))
        ))
        count = len(winning & card)
        if count > 0:
            result1 += 2 ** (count - 1)

    print("Result 1:", result1)

    # part 2 
    copies = defaultdict(lambda: 1)
    for idx, line in enumerate(data.splitlines(), 1):
        head, _, card_numbers = line.partition('|')
        _, _, winning_numbers = head.partition(':')
        winning = set(map(int, filter(None, winning_numbers.split(' '))))
        card = set(map(int, filter(None, card_numbers.split(' '))))
        count = len(winning & card)
        for _ in range(copies[idx]):
            for x, _ in enumerate(range(count), idx + 1):
                copies[x] += 1

    result2 = sum(copies.values())
    print("Result 2:", result2)


if __name__ == '__main__':
    main()

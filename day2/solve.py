test_data = """
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
""".strip()


def main():
    # lines = test_data.splitlines()
    with open('./input.txt', 'rt') as f:
        lines = list(f)

    data = {}
    for line in lines:
        game, rolls = line.strip().split(':')
        game_id = int(game[5:])
        for roll in rolls.split(';'):
            out = {}
            for item in roll.strip().split(','):
                count, dice = item.strip().split(' ')
                out[dice] = int(count)
            data.setdefault(game_id, []).append(out)

    # part 1
    limits = {'red': 12, 'green': 13, 'blue': 14}
    result1 = 0
    for game_id, rolls in data.items():
        if all(is_valid(roll, limits) for roll in rolls):
            result1 += game_id

    print("Result 1:", result1)

    # part 2
    result2 = 0
    for rolls in data.values():
        max_counts = dict.fromkeys(['red', 'green', 'blue'], 0)
        for roll in rolls:
            for dice, count in roll.items():
                max_counts[dice] = max(count, max_counts[dice])
        power = 1
        for count in max_counts.values():
            power = power * count
        result2 += power

    print("Result 2:", result2)


def is_valid(roll, limits):
    for dice in limits:
        if roll.get(dice, 0) > limits[dice]:
            return False
    return True


if __name__ == '__main__':
    main()

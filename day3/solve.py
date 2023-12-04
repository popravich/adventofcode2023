_test_data = """
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

def main():
    # data = _test_data.strip()
    with open('./input.txt', 'rt') as f:
        data = f.read().strip()
    result1 = result2 = 0

    lines = data.splitlines()
    width = len(lines[0])
    height = len(data) // width

    # part 1
    for y in range(height):
        x = 0
        while x < width:
            c = lines[y][x]
            if '0' <= c <= '9':
                num, l = read_num(lines[y][x:])
                if has_adjecent(x, y, l, lines):
                    result1 += num
                x += l
            x += 1

    print("Result 1:", result1)

    # part 2
    asterisks_numbers = {}
    for y in range(height):
        x = 0
        while x < width:
            c = lines[y][x]
            if '0' <= c <= '9':
                num, l = read_num(lines[y][x:])
                for coord in get_asterisks(x, y, l, lines):
                    asterisks_numbers.setdefault(coord, []).append(num)
                x += l
            x += 1
    for nums in asterisks_numbers.values():
        if len(nums) == 2:
            result2 += nums[0] * nums[1]
    print("Result 2:", result2)


def read_num(line):
    l = len(line)
    result = int(line[0]) 
    i = 1
    while i < l and '0' <= (c := line[i]) <= '9':
        i += 1
        result = result * 10 + int(c)
    return result, i


_non_symbol = set('.0123456789')


def has_adjecent(x, y, width, lines):
    max_w = len(lines[0])
    max_h = len(lines)
    for k in range(max(0, y - 1), min(max_h, y + 2)):
        line = lines[k][max(0, x - 1):min(max_w, x + width + 1)]
        if set(line) - _non_symbol:
            return True
    return False


def get_asterisks(x, y, width, lines):
    max_w = len(lines[0])
    max_h = len(lines)
    coords = set()
    for k in range(max(0, y - 1), min(max_h, y + 2)):
        for i in range(max(0, x - 1), min(max_w, x + width + 1)):
            if lines[k][i] == '*':
                coords.add((i, k))
    return coords


if __name__ == '__main__':
    main()

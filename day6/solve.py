import math

_test_data = """
Time:      7  15   30
Distance:  9  40  200
"""

def main():
    # data = _test_data.strip()
    with open('./input.txt', 'rt') as f:
        data = f.read().strip()

    time, distance = data.splitlines()
    times = list(map(int, filter(None, time[5:].split())))
    distances = list(map(int, filter(None, distance[9:].split())))

    races = list(zip(times, distances))
    print("Races:", races)

    result1 = 1

    # part 1

    for max_time, max_distance in races:
        t1, t2 = normalize(*solve_quadratic_eq(1, -max_time, max_distance))
        num_winning = abs(t1 - t2) + 1
        result1 *= num_winning

    print("Result 1:", result1)

    # part 2

    time, distance = data.splitlines()
    max_time = int(''.join(filter(None, time[5:].split())))
    max_distance = int(''.join(filter(None, distance[9:].split())))
    result2 = 1

    t1, t2 = normalize(*solve_quadratic_eq(1, -max_time, max_distance))
    num_winning = abs(t1 - t2) + 1
    result2 *= num_winning

    print("Result 2:", result2)


def solve_quadratic_eq(a, b, c):
    x1 = (-b - math.sqrt(b * b - (4 * a * c))) / (2 * a)
    x2 = (-b + math.sqrt(b * b - (4 * a * c))) / (2 * a)
    return x1, x2


def normalize(x1, x2):
    x1, x2 = min(x1, x2), max(x1, x2)
    x1_n = math.ceil(x1)
    if x1_n == x1:
        x1_n = x1_n + 1
    x2_n = math.floor(x2)
    if x2_n == x2:
        x2_n = x2_n - 1
    return x1_n, x2_n



if __name__ == '__main__':
    main()

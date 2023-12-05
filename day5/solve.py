import time

_test_data = """
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"""

def main():
    # data = _test_data.strip()
    with open('./input.txt', 'rt') as f:
        data = f.read().strip()

    lines = data.split('\n\n')

    result1 = 0

    # part 1
    instructions = []
    assert lines[0].startswith("seeds: ")
    for seed in map(int, lines[0][7:].split()):
        instructions.append({'seed': seed})

    for line in lines[1:]:
        title, *mapping = line.splitlines()
        assert title.endswith(' map:')
        src, dst = title[:-5].split('-to-')
        for ranges in mapping:
            dst_start, src_start, length = list(map(int, ranges.split(' ')))
            for item in instructions:
                assert src in item, ("item is missing src:", item, src)
                idx = item[src]
                item.setdefault(dst, idx)
                if src_start <= idx < src_start + length:
                    item[dst] = dst_start + (idx - src_start)
    result1 = min(i['location'] for i in instructions)
    print("Result 1:", result1)

    # part 2
    tmp = map(int, lines[0][7:].split())
    seed_ranges = list(zip(tmp, tmp))
    in_ = seed_ranges[:]
    out = []
    for line in lines[1:]:
        title, *mapping = line.splitlines()
        assert title.endswith(' map:')
        src, dst = title[:-5].split('-to-')
        out.clear()
        while in_:
            seed_start, seed_length = in_.pop(0)
            has_mapping = False
            for ranges in mapping:
                dst_start, src_start, length = list(map(int, ranges.split(' ')))
                tmp = range_match_and_split(
                    seed_start, seed_length,
                    src_start, length,
                )
                if tmp is None:
                    continue
                has_mapping = True
                match, splits = tmp
                in_.extend(splits)
                out.append((
                    dst_start + match[0] - src_start,
                    match[1],
                ))
            if not has_mapping:
                out.append((seed_start, seed_length))
        in_ = out[:]

    result2 = min(x[0] for x in out)
    print("Result 2:", result2)



def range_match_and_split(s1, l1, s2, l2):
    start = max(s1, s2)
    end = min(s1 + l1, s2 + l2)
    if start >= end:
        return
    splits = []
    if start > s1:
        splits.append((s1, start - s1))
    if end < s1 + l1:
        splits.append((end, s1 + l1 - end))
    return (start, end - start), splits


if __name__ == '__main__':
    main()

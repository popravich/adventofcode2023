import string

def main():
    result1 = result2 = 0
    
    words = [
        'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine',
    ]

    with open('./input.txt', 'rt') as f:
        for line in f:
            line = line.strip()  # \n
            # part1
            digits = list(filter(lambda c: '0' <= c <= '9', line))
            a, b = digits[0], digits[-1]
            x = int(a + b)
            result1 += x

            # part2
            digits = []
            for x in range(len(line)):
                window = line[x:x+5]
                if window.startswith(tuple(string.digits)):
                    digits.append(int(window[0]))
                else:
                    for n, word in enumerate(words, 1):
                        if window.startswith(word):
                            digits.append(n)
                            break
            a, b = digits[0], digits[-1]
            result2 += a * 10 + b

    print("Result 1:", result1)
    print("Result 2:", result2)

if __name__ == '__main__':
    main()

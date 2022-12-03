import os
from collections import defaultdict, Counter

# testinput= """
# vJrwpWtwJgWrhcsFMMfFFhFp
# jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
# PmmdzqPrVvPwwTWBwg
# """

def part2():
    count = 0
    c = defaultdict(lambda:0)
    with open('input.txt') as f:
        for i, line in enumerate(f.readlines()):
            seen = Counter(line)
            for ch in seen:
                if ch not in c:
                    c[ch] = 0
                c[ch] += 1
            print(i)
            if i % 3 == 2:
                # find the duplicate and then clear the counter out
                for ch, occurrences in c.items():
                    if occurrences == 3:
                        print('common char:', ch, get_points(ch))
                        count += get_points(ch)
                        break
                c = defaultdict(lambda:0)
        return count

def part1():
    with open('input.txt') as f:
        count = 0
        for line in f.readline():
            num_items = len(line) // 2
            left, right = line[:num_items], line[num_items:]

            seen = Counter(left)
            for char in right:
                if char in seen:
                    earned = get_points(char)
                    count += earned
                    break
        return count

def get_points(ch):
    ordinal = ord(ch)
    if ordinal >= 97 and ordinal <=122:
        return ordinal - 96

    if ordinal >= 65 and ordinal <= 90:
        return ordinal - 38

    print('unknown boi', ch)

if __name__ == "__main__":
    # print('part1:', part1())
    print('part2:', part2())

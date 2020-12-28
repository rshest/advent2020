from functools import reduce
import operator
import math


def count_trees(rows, dx, dy):
    h = len(rows)
    if h == 0:
        return 0
    w = len(rows[0])
    x, y = 0, 0
    res = 0
    while y < h:
        res += rows[y][x % w] == '#'
        x += dx
        y += dy
    return res


def solution():
    rows = [x.strip() for x in open('../data/03.txt', 'r').readlines()]
    res1 = count_trees(rows, 3, 1)
    print(f'Answer 1: {res1}')

    SLOPES = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    res2 = reduce(operator.mul, (count_trees(rows, x, y)
                                 for (x, y) in SLOPES), 1)
    print(f'Answer 2: {res2}')

from functools import reduce


def get_id(seat):
    return reduce(lambda res, c: res * 2 + (c in 'BR'), seat, 0)


def solution():
    lines = [x.strip() for x in open('../data/05.txt', 'r').readlines()]
    ids = [get_id(line) for line in lines]

    idset = set(ids)
    seats = [id for id in range(1 << len(lines[0]))
             if id not in idset and id + 1 in idset and id - 1 in idset]

    print(f'Answer 1: {max(ids)}')
    print(f'Answer 2: {seats[0]}')

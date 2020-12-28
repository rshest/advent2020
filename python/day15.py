def eval(seed, stop_k):
    positions = {}
    next_num = 0
    k = 1
    while True:
        num = seed[k - 1] if k <= len(seed) else next_num
        if k == stop_k:
            return num

        if num in positions:
            next_num = k - positions[num]
        else:
            next_num = 0

        positions[num] = k
        k += 1


def solution():
    seed = [int(s) for s in open('../data/15.txt', 'r').read().split(',')]

    print(f'Answer 1: {eval(seed, 2020)}')
    print(f'Answer 2: {eval(seed, 30000000)}')

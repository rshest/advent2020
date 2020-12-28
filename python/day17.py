import itertools


def count_neighbors(field, pos):
    if len(field) == 0:
        return 0
    res = 0
    for p in itertools.product(*[range(x - 1, x + 2) for x in pos]):
        if p != pos and p in field:
            res += 1
    return res


def get_bounds(field):
    if len(field) == 0:
        return None
    val = next(iter(field))
    res = [[x, x] for x in val]
    for pos in field:
        for i, x in enumerate(pos):
            res[i][0] = min(res[i][0], x)
            res[i][1] = max(res[i][1], x)
    return res


def step(field):
    bounds = get_bounds(field)
    res = set()
    for pos in itertools.product(*[range(b[0] - 1, b[1] + 2) for b in bounds]):
        n = count_neighbors(field, pos)
        if pos in field:
            if n == 2 or n == 3:
                res.add(pos)
        else:
            if n == 3:
                res.add(pos)
    return res


def init_field(seed, ndim=3):
    field = set()
    for y in range(len(seed)):
        for x in range(len(seed[y])):
            if seed[y][x] == '#':
                field.add(tuple([x, y] + [0] * (ndim - 2)))
    return field


NUM_STEPS = 6


def run(seed, ndim, num_steps=NUM_STEPS):
    field = init_field(seed, ndim)
    for i in range(num_steps):
        field = step(field)
    return len(field)


def solution():
    seed = [line.strip() for line in open('../data/17.txt', 'r').readlines()]
    print(f'Answer 1: {run(seed, 3)}')
    print(f'Answer 2: {run(seed, 4)}')

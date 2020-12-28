def part1(periods, ts):
    min_diff = None
    min_p = 0
    res = 0
    for p in periods:
        if p is None:
            continue
        d = p - ts % p
        if min_diff == None or d < min_diff:
            min_diff = d
            res = d * p
            min_p = p
    return res


def part2(periods):
    stops = sorted(((p, i) for (i, p) in enumerate(periods) if p is not None))
    p0, i0 = stops[0]
    step = p0
    k = 0
    ts = i0
    while True:
        p1, i1 = stops[k + 1]
        if (ts + i1) % p1 == 0:
            step *= p1
            k += 1
            if k == len(stops) - 1:
                return ts
        ts += step


def solution():
    lines = open('../data/13.txt', 'r').readlines()
    ts = int(lines[0])
    periods = [int(x) if x.isdigit()
               else None for x in lines[1].strip().split(',')]

    print(f'Answer 1: {part1(periods, ts)}')
    print(f'Answer 2: {part2(periods)}')

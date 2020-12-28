from functools import reduce


def parse_line(line):
    pos = 0
    while pos < len(line):
        if line[pos] in ['e', 'w']:
            yield line[pos]
            pos += 1
        elif line[pos:pos + 2] in ['se', 'sw', 'nw', 'ne']:
            yield line[pos:pos + 2]
            pos += 2
        else:
            return


def eval_step(pos, step):
    x, y = pos
    if len(step) == 1:
        x = x + 1 if step == 'e' else x - 1
    else:
        vdir, hdir = step[0], step[1]
        if y % 2 == 0:
            if hdir == 'w':
                x -= 1
        else:
            if hdir == 'e':
                x += 1
        y = y + 1 if vdir == 's' else y - 1
    return x, y


def get_neighbor_coord(pos):
    return (eval_step(pos, s) for s in
            ['e', 'w', 'se', 'sw', 'nw', 'ne'])


def eval_path(steps):
    return reduce(eval_step, steps, (0, 0))


def eval_paths(paths):
    res = set()
    for path in paths:
        pos = eval_path(path)
        if pos in res:
            res.remove(pos)
        else:
            res.add(pos)
    return res


def step_day(tiles):
    res = set()
    minx = min(tiles, key=lambda p: p[0])[0]
    maxx = max(tiles, key=lambda p: p[0])[0]
    miny = min(tiles, key=lambda p: p[1])[1]
    maxy = max(tiles, key=lambda p: p[1])[1]

    for x in range(minx - 1, maxx + 2):
        for y in range(miny - 1, maxy + 2):
            t = x, y
            num_neighbors = sum(
                npos in tiles for npos in get_neighbor_coord(t))
            if t in tiles:
                if (num_neighbors == 0 or num_neighbors > 2):
                    # flip to white
                    pass
                else:
                    res.add(t)
            elif num_neighbors == 2:
                res.add(t)
    return res


def solution():
    paths = [list(parse_line(line))
             for line in open('../data/24.txt', 'r').readlines()]

    tiles = eval_paths(paths)
    print(f'Answer 1: {len(tiles)}')

    for i in range(100):
        tiles = step_day(tiles)
    print(f'Answer 2: {len(tiles)}')

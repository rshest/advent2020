OFFS = [[-1, -1], [0, -1], [1, -1], [1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0]]


def get_neighbor_counts(seats):
    h, w = len(seats), len(seats[0])
    res = [[0]*w for i in range(h)]
    for y in range(h):
        for x in range(w):
            n = 0
            for dx, dy in OFFS:
                x1, y1 = x + dx, y + dy
                n += x1 >= 0 and x1 < w and y1 >= 0 and y1 < h \
                    and seats[y1][x1] == '#'
            res[y][x] = n
    return res


def get_visible_neighbor_counts(seats):
    h, w = len(seats), len(seats[0])
    cansee = [[0]*w for i in range(h)]
    res = [[0]*w for i in range(h)]

    for dx, dy in OFFS:
        for j in range(h):
            for i in range(w):
                x = i if dx <= 0 else w - i - 1
                y = j if dy <= 0 else h - j - 1
                x1, y1 = x + dx, y + dy
                if x1 >= 0 and x1 < w and y1 >= 0 and y1 < h:
                    s = seats[y1][x1]
                    if s == '.':
                        cansee[y][x] = cansee[y1][x1]
                    elif s == '#':
                        cansee[y][x] = True
                    elif s == 'L':
                        cansee[y][x] = False
                else:
                    cansee[y][x] = False
                res[y][x] += cansee[y][x]
    return res


def step(seats, new_seats, neighbors_fn, max_occupied):
    neighbor_counts = neighbors_fn(seats)

    num_changed = 0
    total_occupied = 0
    for y in range(len(seats)):
        for x in range(len(seats[y])):
            status = seats[y][x]
            n = neighbor_counts[y][x]
            if status == 'L' and n == 0:
                status = '#'
            elif status == '#' and n >= max_occupied:
                status = 'L'

            new_seats[y][x] = status
            num_changed += status != seats[y][x]
            total_occupied += status == '#'
    return (num_changed, total_occupied)


def iter_seats(seats, neighbors_fn, max_occupied):
    s1 = [line.copy() for line in seats]
    s2 = [line.copy() for line in seats]
    s = [s1, s2]

    idx = 0
    while True:
        (changed, occ) = step(s[idx], s[1 - idx], neighbors_fn, max_occupied)
        if changed == 0:
            return occ
        idx = 1 - idx


def solution():
    seats = [list(line.strip())
             for line in open('../data/11.txt', 'r').readlines()]
    print(f'Answer 1: {iter_seats(seats, get_neighbor_counts, 4)}')
    print(f'Answer 2: {iter_seats(seats, get_visible_neighbor_counts, 5)}')

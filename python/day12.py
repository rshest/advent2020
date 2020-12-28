import math
from functools import reduce

OFFS = {
    'N': (0, -1),
    'E': (1, 0),
    'S': (0, 1),
    'W': (-1, 0),
}


def step1(pos, command):
    x, y, angle = pos
    c, val = command
    if c == 'F':
        c = {270: 'N', 0: 'E', 90: 'S', 180: 'W'}[angle]
    if c in OFFS:
        offs = OFFS[c]
        x += offs[0] * val
        y += offs[1] * val
    else:
        if c == 'R':
            angle += val
        elif c == 'L':
            angle -= val
        angle = (angle + 360) % 360
    return (x, y, angle)


def step2(pos, command):
    x, y, wx, wy = pos
    c, val = command
    if c in OFFS:
        offs = OFFS[c]
        wx += offs[0] * val
        wy += offs[1] * val
    elif c == 'F':
        x += wx * val
        y += wy * val
    elif c in ['R', 'L']:
        angle = val
        if c == 'L':
            angle = -angle
        ar = math.radians(angle)
        ca, sa = math.cos(ar), math.sin(ar)
        wx1 = ca * wx - sa * wy
        wy1 = sa * wx + ca * wy
        wx, wy = round(wx1), round(wy1)
    return (x, y, wx, wy)


def solution():
    commands = [(line[0], int(line[1:]))
                for line in open('../data/12.txt', 'r').readlines()]

    x1, y1, angle1 = reduce(step1, commands, (0, 0, 0))
    print(f'Answer 1: {abs(x1) + abs(y1)}')

    x2, y2, wx2, wy2 = reduce(step2, commands, (0, 0, 10, -1))
    print(f'Answer 2: {abs(x2) + abs(y2)}')

import math
from functools import reduce
import operator

OPPOSITE_DIR = [2, 3, 0, 1]
OFFS = [(0, -1), (1, 0), (0, 1), (-1, 0)]
N = 10


def parse_tile(data):
    lines = [line.strip() for line in data.split('\n')]
    try:
        tile_id = int(lines[0].split()[1][:-1])
        return (tile_id, lines[1:])
    except:
        return None


def get_bit_mask(seq):
    res = 0
    for c in seq:
        res = (res << 1) | (c == '#')
    return res


def reverse_bits(val):
    res = 0
    for i in range(N):
        res = res << 1 | (val & 1)
        val = val >> 1
    return res


def get_borders(tile):
    (tile_id, lines) = tile
    top = get_bit_mask(lines[0])
    bottom = get_bit_mask(lines[-1])
    left = get_bit_mask(line[0] for line in lines)
    right = get_bit_mask(line[-1] for line in lines)
    return (top, right, bottom, left)


def rotate(borders):
    (t, r, b, l) = borders
    return (reverse_bits(l), t, reverse_bits(r), b)


def get_transforms(borders):
    for i in range(4):
        yield borders
        (t, r, b, l) = borders
        # flip horizontal/vertical
        yield (reverse_bits(t), l, reverse_bits(b), r)
        yield (b, reverse_bits(r), t, reverse_bits(l))
        borders = rotate(borders)
    pass


def create_mappings(tiles):
    mappings = [{}, {}, {}, {}]
    for (i, tile) in enumerate(tiles):
        borders = get_borders(tile)
        for tm in set(get_transforms(borders)):
            for side in range(4):
                mask = tm[side]
                if mask not in mappings[side]:
                    mappings[side][mask] = []
                mappings[side][mask].append((i, tm))
    return mappings


def find_corners(tiles, mapping):
    res = []
    for (tile_idx, t) in enumerate(tiles):
        borders = get_borders(t)
        k = 0
        for i, mask in enumerate(borders):
            m = set(x[0] for x in mapping[OPPOSITE_DIR[i]][mask])
            if len(m) == 1 and tile_idx in m:
                k += 1
        if k == 2:
            res.append(tile_idx)
    return res


def get_layout(tiles):
    mappings = create_mappings(tiles)
    n = len(tiles)

    used = [False] * n
    num_used = 0
    res = {}

    def layout(tile_idx, orientation, x, y):
        nonlocal num_used
        if used[tile_idx]:
            return False

        for (i, mask) in enumerate(orientation):
            x1, y1 = x + OFFS[i][0], y + OFFS[i][1]
            if (x1, y1) in res and res[(x1, y1)][1][OPPOSITE_DIR[i]] != mask:
                return False

        used[tile_idx] = True
        res[(x, y)] = (tile_idx, orientation)
        num_used += 1
        if num_used == n:
            return True

        found_fit = False
        for i, mask in enumerate(orientation):
            x1, y1 = x + OFFS[i][0], y + OFFS[i][1]
            if (x1, y1) in res:
                continue
            for (ci, co) in mappings[OPPOSITE_DIR[i]][mask]:
                if layout(ci, co, x1, y1):
                    found_fit = True
                    break
            if found_fit:
                break
        if not found_fit:
            used[tile_idx] = False
            num_used -= 1
            res.pop((x, y), None)
        return found_fit

    corners = find_corners(tiles, mappings)
    start_idx = corners[0]
    layout(start_idx, get_borders(tiles[start_idx]), 0, 0)
    return res


def build_image(tiles, layout):
    minx = min(x[0] for x in layout.keys())
    miny = min(x[1] for x in layout.keys())
    maxx = max(x[0] for x in layout.keys())
    maxy = max(x[1] for x in layout.keys())
    res = [['.' for j in range((maxx - minx + 1) * (N - 2))]
           for i in range((maxy - miny + 1) * (N - 2))]
    for pos, (tile_idx, b) in layout.items():
        borders = list(get_transforms(get_borders(tiles[tile_idx])))
        tm = borders.index(b)
        ang = tm // 3
        flip_x = (tm % 3) == 1
        flip_y = (tm % 3) == 2
        x = (pos[0] - minx) * (N - 2)
        y = (pos[1] - miny) * (N - 2)
        img = tiles[tile_idx][1]
        for i in range(N - 2):
            for j in range(N - 2):
                c = img[j][i]
                cx, cy = i + 1, j + 1
                if flip_x:
                    cx = N - cx - 1
                if flip_y:
                    cy = N - cy - 1

                if ang == 1:
                    cx, cy = cy, N - cx - 1
                elif ang == 2:
                    cx, cy = N - cx - 1, N - cy - 1
                elif ang == 3:
                    cx, cy = N - cy - 1, cx

                res[y + j][x + i] = img[cy][cx]
    return [''.join(s) for s in res]


def count_hashes(image, pattern):
    w, h = len(image[0]), len(image)

    def matches_sub(x, y):
        checked = 0
        for j, p in enumerate(pattern):
            for i, c in enumerate(p):
                x1, y1 = x + i, y + j
                if x1 < 0 or y1 < 0 or x1 >= w or y1 >= h:
                    continue
                if pattern[j][i] == '#' and image[y1][x1] != '#':
                    return False
                checked += 1
        return checked > 0
    res = [list(line) for line in image]
    for y in range(h - len(pattern) + 1):
        for x in range(w - len(pattern[0]) + 1):
            if matches_sub(x, y):
                for j, p in enumerate(pattern):
                    for i, c in enumerate(p):
                        x1, y1 = x + i, y + j
                        if x1 < 0 or y1 < 0 or x1 >= w or y1 >= h:
                            continue
                        if pattern[j][i] == '#':
                            res[y + j][x + i] = 'O'
    num_o, num_x = 0, 0
    for y in res:
        for c in y:
            num_o += c == 'O'
            num_x += c == '#'

    return (num_o, num_x)


def rotate_img(image):
    res = [list(line) for line in image]
    n = len(image)
    for y in range(n):
        for x in range(n):
            res[y][x] = image[x][n - y - 1]
    return [''.join(s) for s in res]


def fliph(image):
    return [line[::-1] for line in image]


def flipw(image):
    return [line for line in image[::-1]]


def get_img_transforms(image):
    for i in range(4):
        yield image
        yield fliph(image)
        yield flipw(image)
        image = rotate_img(image)


def compute_roughness(tiles, pattern):
    layout = get_layout(tiles)
    image = build_image(tiles, layout)
    for img in get_img_transforms(image):
        (co, ch) = count_hashes(img, pattern)
        if co != 0:
            return ch


def solution():
    chunks = open('../data/20.txt', 'r').read().split('\n\n')
    tiles = [t for t in (parse_tile(chunk) for chunk in chunks) if t]
    res1 = reduce(operator.mul, (tiles[t][0]
                                 for t in find_corners(tiles, create_mappings(tiles))), 1)
    print(f'Answer 1: {res1}')

    pattern = open('../data/20.pattern.txt', 'r').read().split('\n')
    res2 = compute_roughness(tiles, pattern)
    print(f'Answer 2: {res2}')

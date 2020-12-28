from collections import namedtuple

Notes = namedtuple('Notes', 'categories own_ticket tickets')


def parse_notes(data):
    def parse_category(line):
        parts = line.split(': ')
        ranges = [tuple(map(int, r.split('-')))
                  for r in parts[1].split(' or ')]
        return (parts[0], ranges)

    def parse_ticket(line):
        return [int(x) for x in line.split(',')]

    parts = [part.strip().split('\n') for part in data.split('\n\n')]
    categories = dict(parse_category(line) for line in parts[0])
    own_ticket = parse_ticket(parts[1][1])
    tickets = [parse_ticket(t) for t in parts[2][1:]]
    return Notes(categories, own_ticket, tickets)


def is_valid_category(cat_ranges, val):
    for r in cat_ranges:
        if r[0] <= val <= r[1]:
            return True
    return False


def get_invalid_entries(notes):
    for (i, t) in enumerate(notes.tickets):
        for val in t:
            valid = False
            for cat_ranges in notes.categories.values():
                if is_valid_category(cat_ranges, val):
                    valid = True
                    break
            if not valid:
                yield (i, val)


def get_valid_tickets(notes, invalid_entries):
    res = []
    invalid_tickets = set(x[0] for x in invalid_entries)
    for (i, t) in enumerate(notes.tickets):
        if i not in invalid_tickets:
            res.append(t)
    return res


def get_category_ordering(notes):
    cat_ranges = list(notes.categories.values())
    cat_names = list(notes.categories.keys())

    n = len(cat_names)
    cands_left = n * n
    candidates = [set(range(n)) for i in range(n)]

    # sift out invalid candidates
    for t in notes.tickets:
        for (i, x) in enumerate(t):
            for cand in candidates[i].copy():
                if not is_valid_category(cat_ranges[cand], x):
                    candidates[i].remove(cand)
                    cands_left -= 1
            if cands_left == n:
                break

    # iteratively subtract single-element candidate sets from others
    while cands_left > n:
        for c in candidates:
            if len(c) == 1:
                val = next(iter(c))
                for (i, c1) in enumerate(candidates):
                    if len(c1) > 1 and val in c1:
                        candidates[i].remove(val)
                        cands_left -= 1
    return [cat_names[next(iter(x))] for x in candidates]


def part2(notes,):
    ordering = get_category_ordering(notes)
    res = 1
    for (i, cat_name) in enumerate(ordering):
        if cat_name.startswith('departure'):
            res *= notes.own_ticket[i]
    return res


def solution():
    notes = parse_notes(open('../data/16.txt', 'r').read())

    invalid_entries = list(get_invalid_entries(notes))
    print(f'Answer 1: {sum(x[1] for x in invalid_entries)}')

    valid_notes = Notes(notes.categories, notes.own_ticket,
                        get_valid_tickets(notes, invalid_entries))
    print(f'Answer 2: {part2(valid_notes)}')

# DAY07
def parse_mapping(line):
    parts = line.split()
    bag_color = ' '.join(parts[0:2])
    res = {}
    for i in range(4, len(parts), 4):
        if parts[i] != 'no':
            res[' '.join(parts[i + 1:i + 3])] = int(parts[i])
    return (bag_color, res)


def build_back_mapping(mapping):
    res = {}
    for (bag, children) in mapping.items():
        for child in children.keys():
            if child in res:
                res[child].add(bag)
            else:
                res[child] = set([bag])
    return res


def count_reachable(root, back_mapping):
    visited = set()

    def rec(node):
        if node in visited:
            return 0
        res = 1
        if node in back_mapping:
            for child in back_mapping[node]:
                res += rec(child)
        visited.add(node)
        return res
    return rec(root)


def count_contains(root, mapping):
    res = 1
    for (child, count) in mapping[root].items():
        res += count * count_contains(child, mapping)
    return res


ROOT = "shiny gold"


def solution():
    lines = [g.strip() for g in open('../data/07.txt', 'r').readlines()]
    mapping = dict(parse_mapping(line) for line in lines)
    back_mapping = build_back_mapping(mapping)

    print(f'Answer 1: {count_reachable(ROOT, back_mapping) - 1}')
    print(f'Answer 2: {count_contains(ROOT, mapping) - 1}')

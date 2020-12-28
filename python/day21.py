def parse_rule(line):
    parts = line[:-1].split(' (contains ')
    return (set(parts[0].split(' ')), set(parts[1].split(', ')))


def get_mappings(rules):
    mappings = {}
    for i in range(len(rules)):
        c, a = rules[i]
        if len(c) == len(a) == 1:
            mappings[c.pop()] = a.pop()
            continue
        cxc, cxa = c, a
        for j in range(i + 1, len(rules)):
            c1, a1 = rules[j]
            if len(cxa & a1) > 0:
                cxc = cxc & c1
                cxa = cxa & a1
            if len(cxc) == len(cxa) == 1:
                mappings[cxc.pop()] = cxa.pop()
                break
    return mappings


def trim_rules(rules):
    res = {}
    while True:
        mappings = get_mappings(rules)
        if len(mappings) == 0:
            return res
        for (c, a) in mappings.items():
            res[c] = a
            for r in rules:
                if c in r[0]:
                    r[0].remove(c)
                if a in r[1]:
                    r[1].remove(a)


def solution():
    rules = [parse_rule(line.strip())
             for line in open('../data/21.txt', 'r').readlines()]
    mappings = trim_rules(rules)
    res1 = sum(len(r[0]) for r in rules)
    print(f'Answer 1: {res1}')

    res2 = ','.join(kv[0]
                    for kv in sorted(mappings.items(), key=lambda x: x[1]))
    print(f'Answer 2: {res2}')

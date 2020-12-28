def parse_rule(line):
    parts = [s.strip() for s in line.split('|')]
    for part in parts:
        if part.startswith('"'):
            yield (True, part[1:-1])
        else:
            yield (False, part.split(' '))


def parse_input(data):
    chunks = list(map(lambda x: x.strip().split('\n'), data.split('\n\n')))
    rules = {}
    for line in chunks[0]:
        parts = line.split(': ')
        rules[parts[0]] = list(parse_rule(parts[1]))
    return (rules,  chunks[1])


def is_valid(rules, line):
    def match_seq(seq, line, pos):
        if len(seq) > 0:
            for cpos in match(rules[seq[0]], line, pos):
                yield from match_seq(seq[1:], line, cpos)
        else:
            yield pos

    def match(rule, line, pos):
        for leaf, body in rule:
            if leaf:
                if line[pos:].startswith(body):
                    yield pos + len(body)
            else:
                yield from match_seq(body, line, pos)
    matches = list(match(rules['0'], line, 0))
    return 0 if len(matches) == 0 else matches[0] == len(line)


def patch_rules(rules):
    rules['8'] = list(parse_rule('42 | 42 8'))
    rules['11'] = list(parse_rule('42 31 | 42 11 31'))


def solution():
    rules, strings = parse_input(open('../data/19.txt', 'r').read())
    print(f'Answer 1: {sum(is_valid(rules, s) for s in strings)}')

    patch_rules(rules)
    print(f'Answer 2: {sum(is_valid(rules, s) for s in strings)}')

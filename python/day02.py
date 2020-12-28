def parse_line(line):
    parts = line.strip().split()
    counts = list(map(int, parts[0].split('-')))
    return (counts[0], counts[1], parts[1][0], parts[2])


def is_valid1(desc):
    cnt1, cnt2, ch, pwd = desc
    cnt = pwd.count(ch)
    return cnt1 <= cnt <= cnt2


def is_valid2(desc):
    cnt1, cnt2, ch, pwd = desc
    return (pwd[cnt1 - 1] == ch) ^ (pwd[cnt2 - 1] == ch)


def solution():
    passwords = list(map(parse_line, open('../data/02.txt', 'r').readlines()))
    res1 = sum([is_valid1(pwd) for pwd in passwords])
    res2 = sum([is_valid2(pwd) for pwd in passwords])

    print(f'Answer 1: {res1}')
    print(f'Answer 2: {res2}')

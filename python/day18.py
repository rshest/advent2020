ADD = 1
MUL = 2


def apply_op(op, a, b):
    if op == None:
        return a if b == None else b
    if op == ADD:
        return a + b
    elif op == MUL:
        return a * b


def parse_num(exp, pos):
    res = None
    while pos < len(exp) and '0' <= exp[pos] <= '9':
        d = ord(exp[pos]) - ord('0')
        res = res * 10 + d if res != None else d
        pos += 1
    return res, pos


def eval_exp(exp, pos=0, add_precedence=False):
    res = None
    op = None
    muls = []
    while pos < len(exp):
        c = exp[pos]
        if c == ')':
            pos += 1
            break
        elif c == '(':
            val, pos = eval_exp(exp, pos + 1, add_precedence)
            res = apply_op(op, res, val)
            op = None
        elif c == ' ':
            pos += 1
        elif c == '*':
            if add_precedence:
                muls.append(res)
                res = None
            else:
                op = MUL
            pos += 1
        elif c == '+':
            op = ADD
            pos += 1
        else:
            val, pos = parse_num(exp, pos)
            if val != None:
                res = apply_op(op, res, val)
                op = None
    for mul in muls:
        res *= mul
    return res, pos


def solution():
    expressions = [line.strip()
                   for line in open('../data/18.txt', 'r').readlines()]
    print(
        f'Answer 1: {sum(map(lambda x: eval_exp(x, 0, False)[0], expressions))}')
    print(
        f'Answer 2: {sum(map(lambda x: eval_exp(x, 0, True)[0], expressions))}')

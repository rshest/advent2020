import enum


class Op(enum.Enum):
    acc = 1
    nop = 2
    jmp = 3


def parse_op(line):
    parts = line.split()
    return (Op[parts[0]], int(parts[1]))


def eval_ops(ops, acc=0):
    n = len(ops)
    visited = [False]*n
    ip = 0
    while ip >= 0 and ip < n:
        if visited[ip]:
            return (False, acc)
        visited[ip] = True

        (op, val) = ops[ip]
        if op == Op.jmp:
            ip += val - 1
        elif op == Op.acc:
            acc += val
        ip += 1
    return (True, acc)


def try_mutate_program(ops):
    for i, (op, val) in enumerate(ops):
        prevOp = op
        if op == Op.jmp:
            op = Op.nop
        elif op == Op.nop:
            op = Op.jmp
        else:
            continue

        ops[i] = (op, val)
        (terminated, acc) = eval_ops(ops)
        if terminated:
            return acc
        ops[i] = (prevOp, val)


def solution():
    ops = [parse_op(g.strip())
           for g in open('../data/08.txt', 'r').readlines()]

    print(f'Answer 1: {eval_ops(ops)[1]}')
    print(f'Answer 2: {try_mutate_program(ops)}')

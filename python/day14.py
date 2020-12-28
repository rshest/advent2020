import re
from collections import namedtuple

MemOp = namedtuple('MemOp', 'mask ops')


def parse_ops(lines):
    cur_op = None
    for line in lines:
        if line.startswith('mask'):
            if cur_op != None:
                yield cur_op
            cur_op = MemOp(re.search(r'mask = (.*)$', line).group(1), [])
        elif line.startswith('mem'):
            match = re.search(r'mem\[(\d+)\] = (\d+)', line)
            cur_op.ops.append((int(match.group(1)), int(match.group(2))))
    yield cur_op


def apply_mask1(val, mask):
    bit = 1
    for i in range(len(mask) - 1, -1, -1):
        c = mask[i]
        if c == '0':
            val = val & ~bit
        elif c == '1':
            val = val | bit
        bit = bit << 1
    return val


def eval1(mem_ops):
    mem = {}
    for mask, ops in mem_ops:
        for (addr, val) in ops:
            mem[addr] = apply_mask1(val, mask)
    return mem


def apply_mask2(val, mask, mask_pos=0):
    if mask_pos == len(mask):
        yield val
        return
    bit = 1 << (len(mask) - mask_pos - 1)
    c = mask[mask_pos]

    if c == '0':
        yield from apply_mask2(val, mask, mask_pos + 1)
    if c == 'X':
        val = val & ~bit
        yield from apply_mask2(val, mask, mask_pos + 1)
    if c == '1' or c == 'X':
        val = val | bit
        yield from apply_mask2(val, mask, mask_pos + 1)


def eval2(mem_ops):
    mem = {}
    for mask, ops in mem_ops:
        for (addr, val) in ops:
            for masked_addr in apply_mask2(addr, mask):
                mem[masked_addr] = val
    return mem


def solution():
    lines = [line.strip() for line in open('../data/14.txt', 'r').readlines()]
    mem_ops = list(parse_ops(lines))

    print(f'Answer 1: {sum(eval1(mem_ops).values())}')
    print(f'Answer 2: {sum(eval2(mem_ops).values())}')

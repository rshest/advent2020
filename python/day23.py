import itertools


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


class CircularList:
    def __init__(self, elems):
        self.lookup = {}
        self.current = None

        cur_node = None
        for el in elems:
            next_node = Node(el)
            if cur_node is None:
                self.current = next_node
                self.min_val = el
                self.max_val = el
            else:
                cur_node.next = next_node
                self.min_val = min(el, self.min_val)
                self.max_val = max(el, self.max_val)
            cur_node = next_node
            self.lookup[el] = cur_node
        cur_node.next = self.current

    def __repr__(self):
        node = self.current
        res = ""
        while True:
            res += f'({node.val}) ' if node == self.current else f'{node.val} '
            node = node.next
            if node == self.current:
                break
        return res

    def get_values(self, start_val=1):
        start_node = self.lookup[start_val]
        node = start_node.next
        while node != start_node:
            yield node.val
            node = node.next


def step(labels):
    to_move = []
    to_move_vals = []
    node = labels.current.next
    for i in range(3):
        to_move.append(node)
        to_move_vals.append(node.val)
        node = node.next
    labels.current.next = node

    dest = labels.current.val - 1
    while dest in to_move_vals or dest not in labels.lookup:
        dest -= 1
        if dest < labels.min_val:
            dest = labels.max_val

    dest_node = labels.lookup[dest]
    dnext = dest_node.next
    dest_node.next = to_move[0]
    to_move[-1].next = dnext

    labels.current = labels.current.next


def expand_seed(seed, total=1000000):
    val = seed[0]
    for x in seed:
        val = max(x, val)
        yield x
    for i in range(len(seed), total):
        val += 1
        yield val


def eval_game(labels, steps=100):
    for i in range(steps):
        if (i + 1) % 1000000 == 0:
            print(f'Step {i}')
        step(labels)


def solution():
    seed = [int(x) for x in list(open('../data/23.txt', 'r').read().strip())]

    labels1 = CircularList(seed)
    eval_game(labels1, 100)
    res1 = ''.join(str(n) for n in labels1.get_values())
    print(f'Answer 1: {res1}')

    labels2 = CircularList(expand_seed(seed))
    eval_game(labels2, 10000000)

    (a, b) = itertools.islice(labels2.get_values(), 2)
    res2 = a * b
    print(f'Answer 2: {res2}')

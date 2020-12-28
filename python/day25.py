FACTOR = 20201227


def transform(subj_number, loop_size):
    val = 1
    for i in range(loop_size):
        val = (val * subj_number) % FACTOR
    return val


def find_loop_len(pub_key, subj_number=7):
    val = 1
    i = 0
    while val != pub_key:
        val = (val * subj_number) % FACTOR
        i += 1
    return i


def solution():
    pub_keys = [int(x) for x in open('../data/25.txt', 'r').readlines()]
    loop_sizes = [find_loop_len(key) for key in pub_keys]
    res = transform(pub_keys[0], loop_sizes[1])
    print(f'Answer 1: {res}')

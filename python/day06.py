
def solution():
    groups = [g.split()
              for g in open('../data/06.txt', 'r').read().split('\n\n')]

    res1, res2 = 0, 0
    for group in groups:
        counts = {}
        for answers in group:
            for ans in answers:
                if ans in counts:
                    counts[ans] += 1
                else:
                    counts[ans] = 1
        res1 += len(counts)
        res2 += sum(counts[k] == len(group) for k in counts.keys())

    print(f'Answer 1: {res1}')
    print(f'Answer 2: {res2}')

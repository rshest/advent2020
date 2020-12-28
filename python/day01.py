def solution():
    nums = [int(x) for x in open('../data/01.txt', 'r').readlines()]

    reg = {}

    TOTAL = 2020
    for i, n in enumerate(nums):
        rest = TOTAL - n
        if rest in reg:
            print(f'Answer 1: {n * rest}')
        reg[n] = i

    for i, n1 in enumerate(nums):
        for j in range(i + 1, len(nums)):
            n2 = nums[j]
            rest = TOTAL - n1 - n2
            if rest in reg and reg[rest] > j:
                print(f'Answer 2: {n1 * n2 * rest}')

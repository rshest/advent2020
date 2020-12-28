
def solution():
    nums = sorted(int(g) for g in open('../data/10.txt', 'r').readlines())
    nums.append(nums[-1] + 3)
    nums.insert(0, 0)

    diffs = []
    for i, n in enumerate(nums[1:]):
        diffs.append(n - nums[i])

    res1 = sum(map(lambda x: x == 1, diffs)) * \
        sum(map(lambda x: x == 3, diffs))
    print(f'Answer 1: {res1}')

    n = len(nums)
    counts = [-1]*n

    def count_arrangements(idx):
        if idx == n - 1:
            return 1
        if counts[idx] >= 0:
            return counts[idx]
        res = 0
        i = idx
        while i < n - 1:
            i += 1
            diff = nums[i] - nums[idx]
            if diff <= 3:
                res += count_arrangements(i)
            else:
                break
        counts[idx] = res
        return res

    res2 = count_arrangements(0)
    print(f'Answer 2: {res2}')

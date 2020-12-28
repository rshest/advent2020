import bisect


def get_first_non_sum(nums, k):
    window = sorted(nums[:k])
    for i in range(k, len(nums)):
        l, r = 0, k - 1
        s = nums[i]
        while l < r:
            cur_sum = window[l] + window[r]
            if cur_sum < s:
                l += 1
            elif cur_sum > s:
                r -= 1
            else:
                break
        if l == r:
            return s
        bisect.insort(window, s)
        del window[bisect.bisect_left(window, nums[i - k])]


def find_sum_range(nums, s):
    l, r = 0, 0
    cur_sum = nums[0]
    n = len(nums)
    while r < n - 1 and l < n - 1:
        if cur_sum < s:
            r += 1
            cur_sum += nums[r]
        elif cur_sum > s:
            cur_sum -= nums[l]
            l += 1
        else:
            break
    if l != r:
        return (l, r)


def solution():
    nums = [int(g) for g in open('../data/09.txt', 'r').readlines()]

    WINDOW_SIZE = 25

    non_sum = get_first_non_sum(nums, WINDOW_SIZE)
    print(f'Answer 1: {non_sum}')

    (b, e) = find_sum_range(nums, non_sum)
    print(f'Answer 2: {min(nums[b:e + 1]) + max(nums[b:e + 1])}')

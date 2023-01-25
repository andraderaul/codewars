from typing import List


def max_sequence(arr: List[int]):
    ans = 0
    prev = 0

    for num in arr:
        curr = max(num, num + prev)
        ans = max(curr, ans)
        prev = curr

    return ans

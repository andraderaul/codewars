from typing import List


def find_it(seq: List[int]) -> int:
    ans = {}

    for num in seq:
        if num in ans:
            ans[num] += 1
        else:
            ans[num] = 1

    out = -1
    for key in ans:
        if ans[key] % 2 == 1:
            out = key
            break

    return out

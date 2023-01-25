from typing import List


def array_diff(a: List[int], b: List[int]) -> List[int]:
    j = 0
    ans = []
    for num in a:
        if j < len(b):
            if num != b[j]:
                ans.append(num)
            else:
                j += 1
        else:
            ans.append(num)

    return ans

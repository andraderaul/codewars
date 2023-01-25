from typing import List, Tuple


def remov_nb(n: int) -> List[Tuple[int, int]]:

    sumx = (n * (n + 1)) // 2
    ans = []

    for i in range(1, n):
        if (sumx - i) % (i + 1) == 0:
            j = (sumx - i) / (i + 1)
            if j < n:
                ans.append((i, j))

    return ans

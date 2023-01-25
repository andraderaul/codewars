from typing import List


def est_subsets(arr: List[int]) -> int:
    return pow(2, len(set(arr))) - 1

from typing import List


def array_diff(a: List[int], b: List[int]) -> List[int]:
    return [x for x in a if x not in b]

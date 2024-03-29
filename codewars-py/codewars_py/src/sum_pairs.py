# https://www.codewars.com/kata/54d81488b981293527000c8f/train/python

from typing import List, Optional


def sum_pairs(ints: List[int], s: int) -> Optional[List[int]]:
    seen = set()

    for n in ints:
        x = s - n
        if x in seen:
            return [x, n]

        seen.add(n)

    return None

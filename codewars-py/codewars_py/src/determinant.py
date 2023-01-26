from typing import List


def det(m: List[List[int]], n) -> int:

    # base case
    if n == 1:
        return m[0][0]

    # pre
    acc = 0

    # recurse
    for i in range(0, n):
        signal = 1 if i % 2 == 0 else -1
        element = m[0][i]
        minor = [x[0:i] + x[i + 1 :] for x in m][1:]

        acc += signal * element * det(minor, n - 1)

    # post
    return acc


def determinant(matrix: List[List[int]]):
    return det(matrix, len(matrix))

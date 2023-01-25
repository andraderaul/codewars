from typing import List


def move_zeros_2(array: List[int]):
    return [x for x in array if x] + [0] * array.count(0)


def move_zeros(lst: List[int]):
    zeros = []
    new_lst = []

    for num in lst:
        if num == 0:
            zeros.append(num)
        else:
            new_lst.append(num)

    return [*new_lst, *zeros]

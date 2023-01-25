# https://www.codewars.com/kata/54e6533c92449cc251001667/train/python

from typing import Union, List


def unique_in_order(sequence: List[Union[str, int]]):
    res = []
    for item in sequence:
        if len(res) == 0 or item != res[-1]:
            res.append(item)
    return res

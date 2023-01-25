# https://www.codewars.com/kata/546d15cebed2e10334000ed9/train/python

from typing import List
import operator

NUMBERS = [str(x) for x in range(0, 10)]
UNKNOWN = '?'
OP = {"*": operator.mul, "-": operator.sub, "+": operator.add}


def solve_exp(lst: List[str]) -> str:

    op = None
    prev = None
    first = ""
    second = ""
    for x in lst:
        if x in NUMBERS and op is None:
            first += x
        elif prev in NUMBERS and OP.get(x):
            op = x
        elif x in NUMBERS and op is not None:
            second += x
        elif op is None:
            first += x
        else:
            second += x

        prev = x

    return str(OP[op](int(first), int(second)))


def solve_runes(runes: str) -> int:
    out = -1
    for num in NUMBERS:
        if num not in runes:
            curr_runes = runes.replace(UNKNOWN, num)
            exp, res = curr_runes.split("=")
            curr_res = solve_exp(list(exp))
            if res == curr_res:
                out = int(num)
                break

    return out

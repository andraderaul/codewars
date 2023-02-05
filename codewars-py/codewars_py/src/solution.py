# https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/python

from typing import List


def solution(args: List[int]) -> str:
    ans = ''
    i = 0
    c = 0
    while i < len(args):
        if i < len(args) - 1 and args[i] == -1 and args[i + 1] == 0:
            c += 1
        elif (
            i < len(args) - 1
            and args[i] != -1
            and abs(abs(args[i]) - abs(args[i + 1])) == 1
        ):
            c += 1
        else:
            if c >= 2:
                ans += f"{args[i-c]}-{args[i]}"
            elif c > 0:
                ans += f"{args[i-c]},{args[i]}"
            else:
                ans += f"{args[i]}"

            ans += "" if i == len(args) - 1 else ","
            c = 0

        i += 1

    return ans


def solution2(args: List[int]) -> str:
    out = []
    beg = end = args[0]

    for n in args[1:] + [""]:
        if n != end + 1:
            if end == beg:
                out.append(str(beg))
            elif end == beg + 1:
                out.extend([str(beg), str(end)])
            else:
                out.append(str(beg) + "-" + str(end))
            beg = n
        end = n

    return ",".join(out)

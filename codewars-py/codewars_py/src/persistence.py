import functools
import operator


def persistence(n: int):
    count = 0
    aux = n

    while aux > 9:
        aux = int(functools.reduce(operator.mul, [int(x) for x in str(aux)], 1))
        count += 1

    return count

def perfect_square_2(square: str):
    l = len(square.splitlines())
    return all("." * l == x for x in square.splitlines())


def perfect_square(square: str) -> bool:
    lines = square.splitlines()
    lines_len = len(lines)
    for line in lines:
        count = 0
        for dot in line:
            if dot == '.':
                count += 1
            else:
                return False

        if count != lines_len:
            return False

    return lines_len == count

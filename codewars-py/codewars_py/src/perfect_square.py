def perfect_square_2(square):
    l = len(square.split("\n"))
    return all("." * l == x for x in square.split("\n"))


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


perfect_square('...\n.;..\n...')

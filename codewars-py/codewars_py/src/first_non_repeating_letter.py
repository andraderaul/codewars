def first_non_repeating_letter_2(string):
    string_lower = string.lower()
    for i, letter in enumerate(string_lower):
        if string_lower.count(letter) == 1:
            return string[i]

    return ""


def first_non_repeating_letter(string: str):
    seen = {}

    for s in string:
        key = s.lower()
        if key in seen:
            seen[key] += 1
        else:
            seen[key] = 1

    out = ''
    for s in string:
        key = s.lower()
        if seen.get(key) == 1:
            out = s
            break

    return out

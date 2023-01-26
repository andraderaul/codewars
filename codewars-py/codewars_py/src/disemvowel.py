VOWEL = ['a', 'e', 'i', 'o', 'u']


def disemvowel(string_: str) -> str:
    new_str = ""
    for v in string_:
        if v.lower() not in VOWEL:
            new_str += v

    print(new_str)
    return new_str

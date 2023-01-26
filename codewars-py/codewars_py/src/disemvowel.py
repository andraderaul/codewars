def disemvowel(string_: str) -> str:
    return "".join(c for c in string_ if c.lower() not in "aeiou")

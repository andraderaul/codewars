from codewars_py.src.disemvowel import disemvowel


def assert_equals(expected, real):
    assert expected == real


def basic_tests():
    assert_equals(
        disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!"
    )

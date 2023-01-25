from codewars_py.src.solve_runes import solve_runes


def assert_equals(expected, real, msg):
    assert expected == real, msg


def test_sample_tests():

    assert_equals(solve_runes("1+1=?"), 2, "Answer for expression '1+1=?' ")
    assert_equals(
        solve_runes("123*45?=5?088"), 6, "Answer for expression '123*45?=5?088' "
    )
    assert_equals(solve_runes("-5?*-1=5?"), 0, "Answer for expression '-5?*-1=5?' ")
    assert_equals(solve_runes("19--45=5?"), -1, "Answer for expression '19--45=5?' ")
    assert_equals(solve_runes("??*??=302?"), 5, "Answer for expression '??*??=302?' ")
    assert_equals(solve_runes("?*11=??"), 2, "Answer for expression '?*11=??' ")
    assert_equals(solve_runes("??*1=??"), 2, "Answer for expression '??*11=??' ")

from codewars_py.src.max_sequence import max_sequence


def assert_equals(expected, real):
    assert expected == real


def test_empty_array():
    assert_equals(max_sequence([]), 0)


def test_example_array():
    assert_equals(max_sequence([-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6)


def test_negative_array():
    assert_equals(max_sequence([-2, -1, -3, -4, -1, -2, -1, -5, -4]), 0)


def test_complex_array():
    assert_equals(
        max_sequence(
            [7, 4, 11, -11, 39, 36, 10, -6, 37, -10, -32, 44, -26, -34, 43, 43]
        ),
        155,
    )

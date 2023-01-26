from codewars_py.src.array_diff import array_diff


def assert_equals(expected, real, msg):
    assert expected == real, msg


def test_basic_test_cases():
    assert_equals(array_diff([1, 2], [1]), [2], "a was [1,2], b was [1], expected [2]")
    assert_equals(
        array_diff([1, 2, 2], [1]), [2, 2], "a was [1,2,2], b was [1], expected [2,2]"
    )
    assert_equals(
        array_diff([1, 2, 2], [2]), [1], "a was [1,2,2], b was [2], expected [1]"
    )
    assert_equals(
        array_diff([1, 2, 2], []),
        [1, 2, 2],
        "a was [1,2,2], b was [], expected [1,2,2]",
    )
    assert_equals(array_diff([], [1, 2]), [], "a was [], b was [1,2], expected []")
    assert_equals(
        array_diff([1, 2, 3], [1, 2]), [3], "a was [1,2,3], b was [1, 2], expected [3]"
    )

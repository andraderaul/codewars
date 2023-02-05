from codewars_py.src.solution import solution, solution2


from tests.src.utils import assert_equals


def test_simple_test_cases():
    assert_equals(
        solution(
            [-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]
        ),
        '-6,-3-1,3-5,7-11,14,15,17-20',
        "solution",
    )

    assert_equals(
        solution([-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
        '-3--1,2,10,15,16,18-20',
        "solution",
    )

    assert_equals(
        solution2(
            [-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]
        ),
        '-6,-3-1,3-5,7-11,14,15,17-20',
        "solution2",
    )

    assert_equals(
        solution2([-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
        '-3--1,2,10,15,16,18-20',
        "solution2",
    )

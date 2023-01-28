from codewars_py.src.determinant import determinant

from tests.src.utils import assert_equals


def test_basic_tests():
    m1 = [[4, 6], [3, 8]]
    m5 = [[2, 4, 2], [3, 1, 1], [1, 2, 0]]

    assert_equals(
        determinant([[5]]),
        5,
        "Determinant of a 1 x 1 matrix yields the value of the one element",
    )
    assert_equals(determinant(m1), 14, "Should return 4*8 - 3*6, i.e. 14")
    assert_equals(
        determinant(m5),
        10,
        "Should return the determinant of [[2,4,2],[3,1,1],[1,2,0]], i.e. 10",
    )

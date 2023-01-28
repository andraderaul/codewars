from codewars_py.src.unique_in_order import unique_in_order

from tests.src.utils import assert_equals


def test_should_work_with_empty_sequence():
    assert_equals(unique_in_order(""), [])
    assert_equals(unique_in_order([]), [])
    assert_equals(unique_in_order(()), [])


def test_should_work_with_single_element_sequence():
    assert_equals(unique_in_order("A"), ["A"])
    assert_equals(unique_in_order(["A"]), ["A"])
    assert_equals(unique_in_order(("A",)), ["A"])


def test_should_reduce_duplicates():
    assert_equals(unique_in_order("AA"), ["A"])
    assert_equals(unique_in_order("AAAABBBCCDAABBB"), ["A", "B", "C", "D", "A", "B"])


def should_be_case_sensitive():
    assert_equals(unique_in_order("ABBCcA"), ["A", "B", "C", "c", "A"])


def should_work_with_different_element_types():
    assert_equals(unique_in_order([1, 2, 3, 3, -1]), [1, 2, 3, -1])
    assert_equals(unique_in_order(["a", "b", "b", "a"]), ["a", "b", "a"])

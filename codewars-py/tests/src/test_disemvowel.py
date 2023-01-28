from codewars_py.src.disemvowel import disemvowel

from tests.src.utils import assert_equals


def test_basic_tests():
    assert_equals(
        disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!"
    )

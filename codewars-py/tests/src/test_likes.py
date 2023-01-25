from codewars_py.src.likes import likes


import pytest


@pytest.mark.parametrize(
    "param",
    [
        {"input": [], "output": 'no one likes this'},
        {"input": ['Peter'], "output": 'Peter likes this'},
        {"input": ['Jacob', 'Alex'], "output": 'Jacob and Alex like this'},
        {"input": ['Max', 'John', 'Mark'], "output": 'Max, John and Mark like this'},
        {
            "input": ['Alex', 'Jacob', 'Mark', 'Max'],
            "output": 'Alex, Jacob and 2 others like this',
        },
    ],
)
def test_basic(param):
    assert likes(param["input"]) == param["output"]

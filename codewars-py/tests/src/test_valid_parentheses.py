from codewars_py.src.valid_parentheses import valid_parentheses

import pytest


@pytest.mark.parametrize(
    "param",
    [
        {"input": ")", "output": False},
        {"input": ")test", "output": False},
        {"input": "", "output": True},
        {"input": "hi())(", "output": False},
        {
            "input": "hi(hi)()",
            "output": True,
        },
    ],
)
def test_sample_tests(param):
    input_x = param["input"]
    output = param["output"]
    assert valid_parentheses(input_x) == output, f"should work for '{input_x}'"

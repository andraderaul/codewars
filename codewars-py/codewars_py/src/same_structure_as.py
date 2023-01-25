from typing import List, Any, Union


def _is_list(arr: List[Any]) -> bool:
    return isinstance(arr, list)


def _is_int_or_str(value: Union[str, int]) -> bool:
    return isinstance(value, int) or isinstance(value, str)


def same_structure_as(original: List[Any], other: List[Any]) -> bool:
    def same_structure_as_rec(original: List[Any], other: List[Any]) -> bool:
        if not _is_list(original) or not _is_list(other) or len(original) != len(other):
            return False

        for i in range(0, len(original)):
            if _is_list(original[i]) and _is_list(other[i]):
                return same_structure_as_rec(original[i], other[i])

            elif not _is_int_or_str(original[i]) or not _is_int_or_str(other[i]):
                return False

        return True

    return same_structure_as_rec(original, other)

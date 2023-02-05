from typing import Dict


def cakes(recipe: Dict[str, int], available: Dict[str, int]):
    return min(
        [available[item] // recipe[item] if item in available else 0 for item in recipe]
    )

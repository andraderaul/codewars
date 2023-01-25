def cakes(recipe, available):
    return min(
        [available[item] // recipe[item] if item in available else 0 for item in recipe]
    )

def unique_in_order(sequence):
    res = []
    for item in sequence:
        if len(res) == 0 or item != res[-1]:
            res.append(item)
    return res

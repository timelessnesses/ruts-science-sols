N = int(input("Plots: "))
M = int(input("Developments: "))


def clamp(n: int, min_: int, max_: int) -> int:
    if n > max_:
        return max_
    elif n < min_:
        return min_
    else:
        return n


BUILDINGS: list[int] = []

for _ in range(N):
    BUILDINGS.append(0)

for _ in range(M):
    start, end, height_change = [
        int(x)
        for x in input(
            "Input the start, ending and building height change split by space: "
        ).split(" ")
    ]
    for x in range(start - 1, end):
        BUILDINGS[x] += height_change

    max_height = max(BUILDINGS)
    tallest_near_buildings_together_count = 0
    for i, b in enumerate(BUILDINGS):
        if b == max_height and (
            BUILDINGS[clamp(i - 1, 0, N - 1)] == max_height
            or BUILDINGS[clamp(i + 1, 0, N - 1)] == max_height
        ):
            tallest_near_buildings_together_count += 1
    print(
        f"Max height: {max_height}, Tallest near buildings count: {tallest_near_buildings_together_count}"
    )
    print(BUILDINGS)

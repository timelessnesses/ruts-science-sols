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
    current_count = 0
    max_near_buildings_count = 0
    for b in BUILDINGS:
        if b == max_height:
            current_count += 1
        else:
            if (current_count > max_near_buildings_count):
                max_near_buildings_count = current_count
            current_count = 0
    if current_count > max_near_buildings_count:
        max_near_buildings_count = current_count
    print(
        f"Max height: {max_height}, Tallest near buildings count: {max_near_buildings_count}"
    )

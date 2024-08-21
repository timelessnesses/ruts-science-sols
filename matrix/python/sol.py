Matrix = list[list[int]]

M = int(input("Rows: "))
N = int(input("Columns: "))

def setup_matrix(row: int, column: int) -> Matrix:
    a: Matrix = []
    for x in range(row):
        for y in range(column):
            if len(a) != x + 1:
                a.append([])
            if len(a[x]) != y + 1:
                a[x].append(0)
            a[x][y] = int(input(f"Enter number [{x + 1}][{y + 1}]: "))
    return a

print(f"Enter first matrix values ({M}x{N})")
first_matrix: Matrix = setup_matrix(M, N)
print(f"Enter second matrix values ({M}x{N})")
second_matrix: Matrix = setup_matrix(M, N)

result: Matrix = []

for x in range(M):
    for y in range(N):
        if len(result) != x + 1:
            result.append([])
        if len(result[x]) != y + 1:
            result[x].append(0)
        result[x][y] = first_matrix[x][y] + second_matrix[x][y]

print("Result")
for x in result:
    out = "     ".join(str(a) for a in x)
    print(out)
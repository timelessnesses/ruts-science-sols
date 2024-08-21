"""
A proper prime finding solution
"""
def prime(n: int) -> bool:
    if n <= 1:
        return False
    for x in range(2, n): # the end is n - 1
        if n % x == 0:
            return False
    return True

def stupid_timeless_prime(n: int) -> bool:
    if n <= 1:
        return False
    p = []
    for x in range(1, n+1):
        if n % x == 0:
            p.append(x)
            
    return len(p) == 2

def odd(n: int) -> bool:
    return not (n % 2 == 0)

N = int(input("Enter number of times you want to check numbers: "))

a = []

for x in range(N):
    a.append(int(input(f"Enter number {x + 1}: ")))

print("Is Prime     |     Is Odd")

for x in a:
    p = prime(x)
    o = odd(x)
    
    print(f"{p}        |     {o}")
n = 1000

for c in range(1, n):
    for b in range(1, c):
        for a in range(1, b):
            if a + b + c == 1000:
                if (a**2)+(b**2) == (c**2):
                    print(a, b, c)
                    print(a*b*c)

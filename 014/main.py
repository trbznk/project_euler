cache = {}
starting_number = None
best_count = 0

for n in range(1, 1_000_000, 2):
    n0 = n
    count = 1
    while n != 1:
        if n in cache:
            count += cache[n]-1
            break
        elif n % 2 == 0:
            n = n/2
        else:
            n = 3*n+1
        count += 1
    if count > best_count:
        starting_number = n0
        best_count = count
    cache[n0] = count

print(starting_number, best_count)


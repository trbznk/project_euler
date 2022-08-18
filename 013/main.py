with open("input.txt") as f:
    lines = f.read().splitlines()


r = 0
for line in lines:
    r += int(line)

print(r)
print(str(r)[:10])


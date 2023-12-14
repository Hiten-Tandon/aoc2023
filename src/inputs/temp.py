grid = tuple(open('14.txt').read().splitlines())

def cycle():
    global grid
    for _ in range(4):
        grid = tuple(map("".join, zip(*grid)))
        grid = tuple("#".join(["".join(sorted(tuple(group), reverse=True)) for group in row.split("#")]) for row in grid)
        grid = tuple(row[::-1] for row in grid)

seen = {grid}
array = [grid]

iter = 0

while True:
    iter += 1
    cycle()
    if grid in seen:
        break
    seen.add(grid)
    array.append(grid)
    
first = array.index(grid)
    
f_count = (1_000_000_000 - first) % (iter - first) + first
grid = array[f_count]

print(sum(row.count("O") * (len(grid) - r) for r, row in enumerate(grid)))

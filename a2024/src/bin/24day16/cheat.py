from collections import defaultdict
from heapq import heappop, heappush

grid = {i+j*1j: c for i,r in enumerate(open("src/input.txt"))
                  for j,c in enumerate(r) if c != '#'}

start, = (p for p in grid if grid[p] == 'S')

seen = []
best = 1e9
dist = defaultdict(lambda: 1e9)
todo = [(0, t:=0, start, 1j, [start])]

while todo:
    val, _, pos, dir, path = heappop(todo)

    if val > dist[pos, dir]: continue
    else: dist[pos, dir] = val

    if grid[pos] == 'E' and val <= best:
        seen += path
        best = val

    for r, v in (1, 1), (+1j, 1001), (-1j, 1001):
        v, t, p, d = val+v, t+1, pos + dir*r, dir*r
        if p not in grid: continue
        heappush(todo, (v, t, p, d, path + [p]))


print(best, len(set(seen)))
from collections import Counter

codes = open("src/input.txt").read().split("\n")

keyp = {c: (i % 3, i // 3) for i, c in enumerate("789456123 0A")}
dirp = {c: (i % 3, i // 3) for i, c in enumerate(" ^A<v>")}


def steps(G, s, i=1):
    px, py = G["A"]
    bx, by = G[" "]
    res = Counter()
    for c in s:
        npx, npy = G[c]
        f = npx == bx and py == by or npy == by and px == bx
        res[(npx - px, npy - py, f)] += i
        px, py = npx, npy
    return res


def go(n):
    r = 0
    for code in codes:
        res = steps(keyp, code)
        for _ in range(n + 1):
            res = sum((steps(dirp, ("<" * -x + "v" * y + "^" * -y + ">" * x)[:: -1 if f else 1] + "A", res[(x, y, f)]) for x, y, f in res), Counter())
        r += res.total() * int(code[:3])
    return r


print(go(2))
print(go(25))
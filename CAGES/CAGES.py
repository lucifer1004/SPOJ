n = int(input())
nxt = [int(input()) - 1 for _ in range(n)]
colors = ['A', 'B', 'C']
color = [''] * n
vis = [False] * n
loops = [[] for _ in range(3)]
for i in range(n):
    if not vis[i]:
        u = i
        loop = []
        while not vis[u]:
            loop.append(u)
            vis[u] = True
            u = nxt[u]
        loops[len(loop) % 3].append(loop)

for loop in loops[0]:
    for i, u in enumerate(loop):
        color[u] = colors[i % 3]

# For modulo 2 groups, we use a global cursor.
now = 0
for loop in loops[2]:
    for u in loop:
        color[u] = colors[now]
        now = (now + 1) % 3

# For modulo 1 groups, the start and the end are defined by the global cursor.
# GC at 'A' -> 'CABCAB...CABA'
# GC at 'B' -> 'ABCABC...ABCB'
# GC at 'C' -> 'BCABCA...BCAC'
# In this way, we can ensure the balance between different characters.
for j, loop in enumerate(loops[1]):
    cnow = (now + 2) % 3
    for i, u in enumerate(loop):
        if i != len(loop) - 1:
            color[u] = colors[cnow]
            cnow = (cnow + 1) % 3
        else:
            color[u] = colors[now]
            now = (now + 1) % 3

print(''.join(color))

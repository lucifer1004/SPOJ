t = int(input())
order = {'$': -1, '+': 0, '-': 1, '*': 2, '/': 3, '^': 4, '(': 5}

for _ in range(t):
    s = input()
    op = []
    val = []
    for c in s + '$':
        if c == '(':
            op.append(c)
        elif c in order:
            while op and op[-1] != '(' and order[op[-1]] >= order[c]:
                assert(len(val) >= 2)
                r = val.pop()
                l = val.pop()
                val.append(l + r + op[-1])
                op.pop()
            op.append(c)
        elif c == ')':
            while op and op[-1] != '(':
                assert(len(val) >= 2)
                r = val.pop()
                l = val.pop()
                val.append(l + r + op[-1])
                op.pop()
            op.pop()
        else:
            val.append(c)

    assert(len(val) == 1)
    print(val[0])

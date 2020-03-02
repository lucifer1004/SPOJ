t = int(input())
for case_num in range(1, t + 1):
    n = int(input())
    a, b = (2, 3) if n == 3 else (n // 2, n - n % 2)
    print('Case {}: {} {}'.format(case_num, a, b))

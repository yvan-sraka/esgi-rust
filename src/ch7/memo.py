#! /usr/bin/env python3

"""
# How to write an homemade @functools.lru_cache

def memoize(func):
    magic = {}
    def func_wrapper(n):
        if n not in magic:
            magic[n] = func(n)
        return magic[n]
    return func_wrapper
"""

import functools

@functools.lru_cache(maxsize = 3)
def fibo(n):
    if n < 2:
        return 1
    else:
        return fibo(n - 1) + fibo(n - 2)

for i in range(100):
    print("fibo(%s) = %s" % (i, fibo(i)))

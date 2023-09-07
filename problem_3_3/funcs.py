import numpy as np
import math
import scipy


def lg_star(n):
    if isinstance(n, np.ndarray):
        return np.array([lg_star(n_) for n_ in n])
    if n <= 0:
        return 0
        # raise ValueError("lg* is only defined for positive numbers")
    
    count = 0
    while n > 1:
        n = np.log2(n)
        count += 1
    
    return count

# fact = lambda x: math.factorial(x.astype(np.int64))
fact = lambda x: scipy.special.factorial(x.astype(np.int64))

# Define 30 arbitrary functions as label and lambda expression pairs
funcs = [
    ("2^(2^(n+1))", lambda x: 2 ** (2 ** (x+1))),
    ("2^(2^n)", lambda x: 2 ** (2 ** x)),
    ("(n+1)!", lambda x: fact(x+1)),
    ("n!", lambda x: fact(x)),
    ("e^n", lambda x: np.exp(x)),
    ("n*2^n", lambda x: x * (2 ** x)),
    ("2^n", lambda x: 2 ** x),
    ("(3/2)^n", lambda x: (3 / 2) ** x),
    ("lg(n)!", lambda x: fact(np.log2(x))),
    ("lg(n!)", lambda x: np.log2(fact(x))),
    ("n^3", lambda x: x ** 3),
    ("4^lg(n)", lambda x: 4 ** np.log2(x)),
    ("n^2", lambda x: x ** 2),
    ("n*lg(n)", lambda x: x * np.log2(x)),
    ("2^lg(n)", lambda x: 2 ** (np.log2(x))),
    ("n", lambda x: x),
    ("2^sqrt(2*lg(n))", lambda x: 2 ** np.sqrt(2 * lg_star(x))),
    ("sqrt(n)", lambda x: np.sqrt(x)),
    ("lg^2(n)", lambda x: np.log2(x) ** 2),
    ("ln(n)", lambda x: np.log(x)),
    ("sqrt(lg(n))", lambda x: np.sqrt(np.log2(x))),
    ("n^lg(lg(n))", lambda x: x ** (np.log2(np.log2(x)))),
    ("ln(ln(n))", lambda x: np.log(np.log(x))),
    ("lg(n)^lg(n)", lambda x: np.log2(x) ** np.log2(x)),
    ("2^lg*(n)", lambda x: 2 ** (lg_star(x))),
    ("lg*(n)", lambda x: lg_star(x)),
    ("lg(lg*(n))", lambda x: np.log2(lg_star(x))),
    ("lg*(lg(n))", lambda x: lg_star(np.log2(x))),
    ("1", lambda x: np.full_like(x, 1)),
    ("n^(1/lg(n))", lambda x: x ** (1 / np.log2(x))),
]
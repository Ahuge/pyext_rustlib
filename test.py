#!/usr/bin/env python3

import os
import sys
import time

sys.path.append(os.path.abspath("./target/release"))

from librusttest import rustfib

def pyfib(terms):
    if terms <= 1:
        return terms
    return (pyfib(terms-1) + pyfib(terms-2))
    
def timer(cbl, args):
    s = time.time()
    try:
        result = cbl(*args)
    except:
        result = None
    print("Time calling %s took %s" % (cbl.__name__, (time.time() - s)))
    return result


for term in [10, 20, 30, 35, 40, 45, 50]:
    msg = "Testing %d..." % term
    print("=" * len(msg))
    print(msg)
    print("  PyFib:")
    r = timer(pyfib, [term])
    print("  RustFib:")
    r2 = timer(rustfib, [term])
    if r != r2:
        print("---ERROR---")
        print("  PyFib: %d   !=   RustFib: %d" % (r, r2))

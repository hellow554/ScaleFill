#!/usr/bin/env python3

import math
from time import sleep


def easeInOutSine(x):
    return -(math.cos(math.pi * x) - 1) / 2


def upDown(start, end):
    while True:
        for x in range(start, end):
            yield x
        for x in range(end, start, -1):
            yield x


if __name__ == "__main__":
    import sys

    path = sys.argv[1]
    with open(path, "w") as f:
        for x in upDown(0, 100):
            val = easeInOutSine(x / 100) * 8
            val = abs(val)
            f.write(f"+{val:9.2f}kg\n")
            sleep(0.1)

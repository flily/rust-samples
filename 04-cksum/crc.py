#!/usr/bin/env python
# coding: utf-8


def bit(x):
    return 1 << x

def gen():
    return bit(32) | bit(26) | bit(23) | bit(22) | bit(16) | bit(12) | bit(11) | bit(10) | bit(8) | bit(7) | bit(5) | bit(4) | bit(2) | bit(1) | bit(0)

def gen_r():
    r = [gen()]
    for x in range(1, 8):
        r0 = r[-1]
        rr = 0
        if r0 & bit(31):
            rr = gen()
        r.append((r0 << 1) ^ rr)

    return r

def crc_table(r, x):
    rem = 0
    for i in range(8):
        if x & bit(i):
            rem ^= r[i]

    return rem & 0xffffffff

def main():
    r = gen_r()
    print(r)
    for x in range(256):
        c = crc_table(r, x)
        print("[%3d] 0x%08x %d" % (x, c, c))

if __name__ == '__main__':
    main()

#!/usr/bin/python3

class Grid:
    def __init__(self, dims):
        self.dims = dims
        self.values = [ False ]
        self.min = [0] * dims
        self.max = [0] * dims

    def add(self, x, y, z):
        coord = (x, y, z)

        for i, v in enumerate(coord):
            if v < self.min[i]:
                self.min[i] = v
            if v > self.max[i]:
                self.max[i] = v

        self.bits.add(coord)

    def reset(self):
        self.bits.clear()
        for i in range(3):
            self.min[i] = 0
            self.max[i] = 0

    def load(self, input):
        self.reset()

        for y, line in enumerate(input):
            for x, ch in enumerate(line.rstrip()):
                if ch != "#":
                    continue

                self.add(x, y, 0)

    def step(self):
        t = self.bits
        self.bits = self.next_bits
        self.next_bits = t

        mn = list(self.min)
        mx = list(self.mx)
        self.reset()

        for x in range(mn[0] - 1, mx[0] + 2):
            for y in range(mn[1] - 1, mx[1] + 2):
                for z in range(mn[2] - 1, mx[2] + 2):
                    neighbours = self.count_neighbours(x, y, z)

                    if (x, y, z) in t:
                        


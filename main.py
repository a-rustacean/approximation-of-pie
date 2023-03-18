from math import sqrt
from random import random

totatPoints = 10 ** 7
pointsInCircle = 0

def length(x, y):
    return sqrt(x * x + y * y)


for i in range(totatPoints):
    x = random()
    y = random()

    dist = length(x, y)
    if (dist < 1):
        pointsInCircle += 1

print(pointsInCircle * 4 / totatPoints)

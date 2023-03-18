"use strict";
function len(x, y) {
  return Math.sqrt(x * x + y * y);
}

const totalPoints = 10 ** 7;
let pointsInCircle = 0;

for (let i = 0; i < totalPoints; i++) {
  const x = Math.random();
  const y = Math.random();

  const dist = len(x, y);
  if (dist < 1) pointsInCircle++;
}

console.log(pointsInCircle * 4 / totalPoints);

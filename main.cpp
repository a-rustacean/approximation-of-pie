#include <cstdlib>
#include <iostream>

using namespace std;

double len(double x, double y) {
  return sqrt(x * x + y * y);
}

int main() {
  int totalPoints = 10000000;
  int pointsInCircle = 0;

  srand(time(0));
  for (int i = 0; i < totalPoints; i++) {
    int x = rand();
    int y = rand();
    double dist = len(x, y);
    if (dist < RAND_MAX) {
      pointsInCircle++;
    }
  }
  cout << (double) pointsInCircle * 4.0 / (double) totalPoints << endl;
  return 0;
}

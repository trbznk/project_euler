#include <iostream>
using namespace std;

#include <math.h>


double gauss(double divisor, double limit) {
    double n = floor(limit/divisor);
    double nsum = (n+1)*divisor;
    return n*nsum/2;
}

int main() {
    double a = 3;
    double b = 5;
    double limit = 1000;
    limit = limit-1;

    cout << gauss(a, limit) << endl;
    cout << gauss(b, limit) << endl;
    cout << gauss(a, limit)+gauss(b, limit) << endl;
    cout << gauss(a, limit)+gauss(b, limit)-gauss(a*b, limit) << endl;

    return 0; 
}

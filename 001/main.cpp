/* 
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

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

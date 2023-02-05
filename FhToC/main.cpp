#include <iostream>
using namespace std;
int main() {
    cout << "Say temperature in Fahrenheit:";
    int fahrenheit;
    cin >> fahrenheit;
    double celcius = (fahrenheit - 32) / 1.8;
    cout << "Temperature in Celsius is: " << celcius;
    return 0;
}

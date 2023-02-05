#include <iostream>

using namespace std;

int main() {
    double sales = 95000;
    cout << "Your sales: $" << sales << endl;

    const double stateTaxRate = .04;
    double stateTax = sales * stateTaxRate;
    cout << "State Tax: $" << stateTax << endl;

    const double countyTaxRate = .02;
    double countyTax = sales * stateTaxRate;
    cout << "County Tax: $" << stateTax << endl;

    cout << "You paid: $" << total << " in taxes " << endl;

    return 0;
}

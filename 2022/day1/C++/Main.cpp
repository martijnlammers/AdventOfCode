#include <iostream>
#include <fstream>
#include <string>
using namespace std;
int main()
{
    ifstream file(".\\..\\input.txt");
    if (!file.is_open())
        throw runtime_error("could not open file");
    string fileLine;
    int highestCals, calories;
    while (getline(file, fileLine)){
        if(calories > highestCals) highestCals = calories;
        calories = (fileLine.empty()) ? 0 : calories + stoi(fileLine);
    }
    file.close();
    cout << highestCals << endl;
    return 0;
}
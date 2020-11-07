#include <bits/stdc++.h> //all standard libraries in cpp
using namespace std;

std::ifstream infile("2sum.txt");
long long a[1000005] = {0}; //initialize array to all zeros

int main(){
    long long in; //input number
    int i = 0;
    while (infile >> in){
        a[i++] = in;
    }

    int size = i;
    sort (a, a+size); // sort array first
    std::map<long long, bool> mp;
    int ans = 0;
    for (int i = 0; i < size; ++i){
        // get smallest and largerst value of a[i]
        long long l = (-10000) - a[i];
        long long r = (10000) - a[i];


        // int j = lower_bound(a, a+size, l) - a     --->   get first element not less than l     
        // j >=0 && j < size && a[j] <= r            --->   keep within bounds 

        for (int j = lower_bound(a, a+size, l) - a; j >=0 && j < size && a[j] <= r; j++){
            
            long long sum = a[i] + a[j];
            if (sum <-10000 or sum > 10000){
                return 0;
            }

            //set mp as registering a distinct x and y
            if (mp[sum] == 0){
                mp[sum] = 1;
                ++ans;
            }
        }

    }
    std::cout << ans << std::endl;

}
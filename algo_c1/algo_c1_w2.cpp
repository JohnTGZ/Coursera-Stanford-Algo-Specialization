#include <iostream>
#include <fstream>
#include <string>

using namespace std;
// struct int_array {
//     int array[];
//     int num;
// };


int *mergesort(int *A, int *B, int n)
{
    
    int i = 0; int j = 0; int C[n];
    for(int k=0; k<n; k++) {
        if(A[i] < B[j]) {
            C[k] = A[i];
            i++;
        }
        else
        {
            C[k] = B[j];
            j++;
        }
    }
    return C;
}

int *divide(int *A, int n) 
{
    if (n==1) {
        return A;
    }
    else {
       //split A into 2
        int A1[n/2];
        int A2[n/2];
        for (int i= 0; i<n/2; i++) {
            A1[i] = A[i];}
        for (int i= n/2; i<n; i++) {
            A2[i] = A[i];}

        //recursive calls
        int *X; int X_array[n/2];
        X = X_array;
        X = divide(A1,n/2);

        int *Y; int Y_array[n/2];
        Y = Y_array;
        Y = divide(A2,n/2);

        //merge sort 
        int *Z = mergesort(X,Y,n);
        return Z;
    }
}


// long count(int A[], int n) {
//     if (n=1) return 0;
//     else {
//         //split A into 2 halves A1 and A2
//         int A1[n/2];
//         int A2[n/2];
//         for (int i= 0; i<n/2; i++) {
//             A1[i] = A[i];}
//         for (int i= n/2; i<n; i++) {
//             A2[i] = A[i];}

//         //recursive calls  
//         int_array X = sortandcount(A1, n/2);
//         int_array Y = sortandcount(A2, n/2);
//         int_array Z = countsplitinv(A, X.num_array, Y.num_array, n);
//         return X.num + Y.num + Z.num;
//     }
// }

int main() 
{
    //read file
    std::string line;
    std::ofstream file_array;
    file_array.open("IntegerArray.txt");
    if (file_array.is_open()) 
    {
        while (
        {
            cout << line << '\n';
        }
        file_array.close();
    }
    else cout << "unable to open file";
    return 0;

    // int A[4] = {1,2,3,4};
    // int n = 4;
    // int *result = divide( A,  n);  
    // std::cout << result << std::endl;
}
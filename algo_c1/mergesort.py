#Written by JohnTGZ
#The classic mergesort algorithm

import numpy as np
import math 

def mergesort(A, B, n):
    i = 0; j = 0; inv_count = 0
    C = np.zeros([n])
    for k in range(0,n):
        if i<len(A) and j<len(B):
            if(A[i] < B[j]):
                C[k] = A[i]
                i +=1
            else: #inverse case
                C[k] = B[j]
                j += 1
                inv_count += len(A)-(i)
        elif i<len(A) and j==len(B):
            C[k] = A[i]
            i +=1
        elif i == len(A) and j<len(B):
            C[k] = B[j]
            j +=1
    return C,inv_count
    
def splitandsort(A):
    n=len(A)
    if n==1:
        return A, 0
    else:
        split_idx = int(math.floor(n/2))
        #split array into 2
        A_x = A[ 0 :  split_idx ]
        A_y = A[  split_idx : n ]

        #recursive calls
        B, B_count = splitandsort(A_x)
        C, C_count = splitandsort(A_y)

        #mergesort
        D,D_count = mergesort(B, C, len(B)+len(C))
        total_inv_counts = B_count + C_count + D_count
        return D, total_inv_counts

if __name__ == '__main__':
    file = open("data/IntegerArray.txt",'r')

    array = np.zeros([100000])
    i=0
    for line in file:
        print(line, i)
        array[i] = line
        i+=1

    A = array
    B,total_inv_counts = splitandsort(A)
    print("Length of array:",len(array))
    print("Original A",A)
    print("Sorted A:",B)
    print("Total Inv Count", total_inv_counts)
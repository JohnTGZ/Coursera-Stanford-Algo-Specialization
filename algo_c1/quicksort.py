#Written by John TGZ

#This script compares 3 different approaches to quicksort and prints the number of comparisms from each approach
#quicksort1 sorts by choosing the first element as the pivot when performing partitioning
#quicksort2 sorts by choosing the last element as the pivot when  partitioning
#quicksort3 sorts by the 'Median of 3' pivot rule when partitioning

import numpy as np


def ChoosePivot1(A, n):
    #select first element as pivot
    return A[0], 0

def ChoosePivot2(A, n):
    #Select final element as pivot
    return A[n-1], n-1

def ChoosePivot3(A, n):
    #'Median of 3' pivot rule
    # med_A, med_B, med_C are the 1st, mid and 3rd index of the arrays respectively
    med_A = 0; med_C = n-1

    if n%2 == 0: #even case
        med_B = int((n/2) - 1)
    else: #odd case
        med_B = int(np.floor(n/2))

    max_val = max(int(A[med_A]), int(A[med_B]), int(A[med_C]))
    min_val = min(int(A[med_A]), int(A[med_B]), int(A[med_C]))

    if A[med_A]!= max_val and A[med_A]!= min_val: #med_A in the middle
        pivot = med_A
    elif A[med_B]!= max_val and A[med_B]!= min_val: #med_B in the middle
        pivot = med_B
    else:
        pivot = med_C

    return A[pivot], pivot

def Partition(A, P, P_idx, n, B):

    #check if pivot is not in first position, and swap if triggered
    if P_idx != 0: 
        temp_var = A[0]
        A[0] = P
        A[P_idx] = temp_var

    i = 1 #On the first largest element
    for k in range(1,n): #k is on the element being compared with Pivot P
        if A[k] < P:
            temp_var = A[i]
            A[i] = A[k]
            A[k] = temp_var
            i+=1
    #Swap pivot(in 1st position) and position of i-1
    temp_var = A[i-1]
    A[i-1] = P
    A[0] = temp_var

    #Splitting array A
    L_array = A[0: i-1] #index 0 to i-2 inclusive
    L_array_len = i-1 
    R_array = A[i: n] #index i to n-1 inclusive
    R_array_len = n - i



    return L_array, R_array, L_array_len, R_array_len


def Quicksort1(A, n, B):
    #Quicksort1 always chooses final element as pivot
    if n<=1:
        return

    #Pivot
    P, P_idx = ChoosePivot1(A,n) 
    B[0] += n-1
    L_array, R_array, L_array_len, R_array_len = Partition(A, P, P_idx, n, B)

    #recursive calls
    Quicksort1(L_array, L_array_len, B)
    Quicksort1(R_array, R_array_len, B)
    
def Quicksort2(A, n, B):
    #Quicksort2 always chooses last element as pivot
    if n<=1:
        return

    #Pivot
    P, P_idx = ChoosePivot2(A,n) 
    B[0] += n-1
    L_array, R_array, L_array_len, R_array_len = Partition(A, P, P_idx, n, B)

    #recursive calls
    Quicksort2(L_array, L_array_len, B)
    Quicksort2(R_array, R_array_len, B)
    
def Quicksort3(A, n, B):
    #sorts based on 'Median of 3' pivot rule
    if n<=1:
        return

    #Pivot
    P, P_idx = ChoosePivot3(A,n) 

    B[0] += n-1

    L_array, R_array, L_array_len, R_array_len = Partition(A, P, P_idx, n, B)

    #recursive calls
    Quicksort3(L_array, L_array_len, B)
    Quicksort3(R_array, R_array_len, B)


if __name__ == '__main__':
    file = open("data/QuickSort.txt",'r')
    n = 10000
    array = np.zeros([n])

    i=0
    for line in file:
        array[i]=line
        i+=1

    A1 = array; A2 = array; A3 = array
    B1 = np.array([0]); B2 = np.array([0]); B3 = np.array([0])
    A_n = len(A1)

    Quicksort1(A1, A_n, B1); print("no of comp 1", B1[0])
    Quicksort2(A2, A_n, B2); print("no of comp 2", B2[0])
    Quicksort3(A3, A_n, B3); print("no of comp 3", B3[0])


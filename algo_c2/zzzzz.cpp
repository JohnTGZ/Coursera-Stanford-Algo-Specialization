// Your task is to compute the number of target values t in the interval 
// [-10000,10000] (inclusive) such that there are distinct numbers x,y 
// in the input file that satisfy x+y=t. 
// (NOTE: ensuring distinctness requires a one-line addition to the algorithm from lecture.)
#include <chrono>
#include <iostream>
#include <fstream>
#include <string>
#include <set>
#include <unordered_set>
#include <map>
#include <algorithm>

class Timer
{
    public:
        Timer(std::string name_): name(name_){
            m_StartTimepoint =  std::chrono::high_resolution_clock::now();
        }
        ~Timer(){
            Stop();
            std::cout << "name: " << name << std::endl;
        }

        void Stop(){
            

            auto endTimePoint = std::chrono::high_resolution_clock::now();

            auto start = std::chrono::time_point_cast<std::chrono::microseconds>(m_StartTimepoint).time_since_epoch().count();
            auto stop = std::chrono::time_point_cast<std::chrono::microseconds>(endTimePoint).time_since_epoch().count();
        
            auto duration = stop - start;
            double ms = duration * 0.001;

            std::cout << duration << "us (" <<ms << " ms)"<< std::endl;
        }
    private:
        std::string name;
        std::chrono::time_point< std::chrono::high_resolution_clock> m_StartTimepoint;
};

void fileToHash_unorderedset(std::string filename_, std::unordered_set<long> &A){
    std::ifstream inFile;
    inFile.open(filename_);
    if (!inFile){
        std::cerr<< "unable to open file"<<
        filename_ << std::endl;
        exit(1);
    }
    std::string line;
    while (std::getline(inFile, line, '\n')){
        long num = std::stol(line);
        A.insert(num);
    }

    std::cout << "completed file to hash with size " << A.size() << std::endl;
}

void fileToHash_set(std::string filename_, std::set<long> &A){
    std::ifstream inFile;
    inFile.open(filename_);
    if (!inFile){
        std::cerr<< "unable to open file"<<
        filename_ << std::endl;
        exit(1);
    }
    std::string line;
    while (std::getline(inFile, line, '\n')){
        long num = std::stol(line);
        A.insert(num);
    }

    std::cout << "completed file to hash with size " << A.size() << std::endl;
}



int main(){
    
    long long A[1000000] = {0};
    std::map<long long, bool> interval;
    long max = 10000;
    long min = -10000;

    std::ifstream INFILE;
    INFILE.open("2sum.txt");
    long long num; int itr = 0;
    while (INFILE >> num){
        A[itr++] = num;
    }
    int size = itr;
    
    std::sort(A, A+size);

    long ans = 0;

    for (int i = 0; i < size; ++i){
        long long lower_lim = -A[i] - 10000;
        long long upper_lim = 10000 - A[i];
        long long lower_pos = std::lower_bound(A, A+size, lower_lim) - A;
        long long upper_pos = std::upper_bound(A, A+size, upper_lim) - A;

        // std::cout << "A[" << i << "]:" << A[i] << 
        // " of limits: " << lower_lim << " and " << upper_lim << 
        // " between " <<  lower_pos << " and " << upper_pos << std::endl;
        
        // std::set<long long> nums;
        for (int j = lower_pos; j>=0 && j<size && A[j] <= upper_lim; j++ ){
            long long sum = A[i] + A[j];
            // if (sum < -10000 or sum > 10000){
            //     std::cout << "continue" << std::endl;
            //     continue;
            // }
            if (interval[sum] == 0){
                interval[sum] = 1;
                ++ans;
            }
        }
    }
    std::cout << "ans: " << ans << std::endl;


}



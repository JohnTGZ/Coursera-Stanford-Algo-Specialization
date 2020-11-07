// Your task is to compute the number of target values t in the interval 
// [-10000,10000] (inclusive) such that there are distinct numbers x,y 
// in the input file that satisfy x+y=t. 
// (NOTE: ensuring distinctness requires a one-line addition to the algorithm from lecture.)

#include <iostream>
#include <ifstream>
#include <string>
#include <unordered_set>

void fileToHash(std::string filename_, std::unordered_set<int> &A){
    std::ifstream inFile;
    inFile.open(filename_);
    if (!inFile){
        std::cerr<< "unable to open file"<<
        filename_ << std::endl;
        exit(1);
    }
    std::string line;
    while (std::getline(inFile, line, '\n')){
        int num = std::stoi(line);
        A.insert(num);
    }

    std::cout << "completed file to hash with size " << A.size() << std::endl;
}


int main(){
    
    std::unordered_set<int> A;

    fileToHash("2sum.txt", A);

}



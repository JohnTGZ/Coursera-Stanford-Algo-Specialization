#include <vector>
#include <string>
#include <cmath>

#include <algorithm>

#include <iostream>

#include <fstream> //theres ifstream and ofstream too
#include <sstream>

#include <chrono>

#define INF 1000000

class Timer
{
    public:
        Timer(){
            m_StartTimepoint =  std::chrono::high_resolution_clock::now();
        }
        ~Timer(){
            Stop();
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
        std::chrono::time_point< std::chrono::high_resolution_clock> m_StartTimepoint;
};


struct greater { // for min heap
    bool operator()(const int a, const int b) const {
        return a > b;
    }
};

struct lesser { // for max heap
    bool operator()(const int a, const int b) const {
        return a < b;
    }
};


class Median{
    public:
        Median(std::string file_name_, int n_): file_path(file_name_), n(n_) {}
        
        void preprocessGraph();
        void testHeaps();


    // private:
        std::string file_path;
        int n;
        std::vector<int> h_low; //max heap
        std::vector<int> h_high; //min heap
};

void Median::preprocessGraph(){

    std::ifstream inFile;
    inFile.open(file_path);

    if (!inFile) {
        std::cerr << "unable to open file at " << file_path << std::endl;
        exit(1);
    }

    std::string line;
    int half;
    half = n/2 + 1; 

    //median k 
    int k = 1;
    int k_;
    int sum = 0;
    int itr = 0;
    bool firsttime_ = true;

    bool both_heaps_filled = false;

    while (std::getline(inFile, line, '\n')){       
        int num = std::stoi(line);  

        //add elements
        if (!both_heaps_filled){
            if (firsttime_){
                h_low.push_back(num);
                std::push_heap(h_low.begin(), h_low.end()); //largest element in front
                
                firsttime_ = false;
            }
            else{
                if (num > h_low.front()){ 
                    h_high.push_back(num);
                    std::push_heap(h_high.begin(), h_high.end(), greater()); 
                    both_heaps_filled = true;
                }
                else{
                    h_low.push_back(num);
                    std::push_heap(h_low.begin(), h_low.end());    

                    if (h_low.size() - 1 > h_high.size()){
                        //remove from h_low and push to h_high
                        std::pop_heap(h_low.begin(), h_low.end());
                        int elem = h_low.back();
                        h_low.pop_back();

                        h_high.push_back(elem);
                        std::push_heap(h_high.begin(),h_high.end(), greater());
                    }  
                }
            }
        }
        else
        {
            //add to heap
            if (num <= h_low.front()){ 
                // std::cout << "itr: (" << itr << "): " << num << " <= " << h_low.front() << std::endl;
                h_low.push_back(num);
                std::push_heap(h_low.begin(), h_low.end());
            }
            else{
                // std::cout << "itr: (" << itr << "): " << num << " > " << h_low.front() << std::endl;
                h_high.push_back(num);
                std::push_heap(h_high.begin(), h_high.end(), greater()); 
            }

            //check if heaps are imbalanced
            if (h_high.size() - 1 > h_low.size() ) //if h_high has at least 2 elements more
            {
                //remove from h_high and push to h_low
                std::pop_heap(h_high.begin(), h_high.end(), greater());
                int elem = h_high.back();
                h_high.pop_back();

                h_low.push_back(elem);
                std::push_heap(h_low.begin(),h_low.end());
                
            }
            else if (h_low.size() - 1 > h_high.size() ){
                //remove from h_low and push to h_high
                std::pop_heap(h_low.begin(), h_low.end());
                int elem = h_low.back();
                h_low.pop_back();

                h_high.push_back(elem);
                std::push_heap(h_high.begin(),h_high.end(), greater());
            }
            
        }


        if (k%2 == 0){
           k_ = k/2;
           
        }
        else {
            k_ = (k+1)/2;
        }
        
        //extract the median
        if (k_ > h_low.size()){ 
            sum += h_high.front();
            // std::pop_heap(h_high.begin(), h_high.end(), greater()); //get minimum element from h_high
            // sum += h_high.back();
            // std::sort_heap(h_high.begin(), h_high.end(), greater());
            itr++;
        }
        else{
            sum += h_low.front();
            // std::pop_heap(h_low.begin(), h_low.end()); //get max element from h_low
            // sum += h_low.back();
            // std::sort_heap(h_low.begin(), h_low.end());
            itr++;
        }

        //print out current vector
        // std::cout<<"__h_low__ (" << itr  << "): ";
        // for (int low : h_low){
        //     std::cout << low << ", ";
        // }
        // std::cout << "\n";

        // std::cout <<"__h_high__ (" << itr  << "): ";
        // for (int high : h_high){
        //     std::cout << high << ", ";
        // }
        // std::cout << "\n";
        
        k++;
    }

    sum = sum % 10000;
    
    std::cout << "result: " << sum << " with itr: " << itr << std::endl;


    // std::cout << h_high.size() << std::endl;
    // std::cout << h_low.size() << std::endl;

    inFile.close();
}

void Median::testHeaps(){
    //test h_low
    int half = (n/2) + 1; 

    std::cout << "h_low: " << std::endl;
    for (int h_low_elem : h_low){
        std::cout << h_low_elem << std::endl;
    }

    std::cout << "h_high: " << std::endl;
    for (int h_high_elem : h_high){
        std::cout << h_high_elem << std::endl;
    }
}

int main(int argc , char *argv[]){

    int n = 10000;
    Median median_("Median.txt", n);

    median_.preprocessGraph();

    // median_.testHeaps();

    return 0;
}   
//scheduling 
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>

struct Job{
  float key;
  int weight;
  int length;
  Job(float key, int weight, int length): 
    key(key), weight(weight), length(length) {}

  //increasing order?
  // bool operator < (const Job& rhs){
  //   if (key == rhs.key){
  //     return (weight < rhs.weight);
  //   }
  //   else{
  //     return (key < rhs.key);
  //   }
  // }
};

struct decreasing_key_order{
  bool operator() (const Job& lhs, const Job& rhs){
    if (lhs.key == rhs.key){
      return lhs.weight > rhs.weight;
    }
    else{
      return (lhs.key > rhs.key);
    }

  }
};


float key_diff(int& weight, int& length);
float key_ratio(int& weight, int& length);
long sumWeightCompletionTime(std::vector<Job>& job_array);

int main(){
  int weight, length;
  int num_jobs;

  std::vector<Job> job_array;

  std::ifstream infile("jobs.txt");
  //starting element
  infile >> num_jobs;
  std::cout << "number of jobs" << num_jobs << std::endl;

  while (infile >> weight >> length){
    // std::cout << weight << ", " << length << std::endl;
    // float key = key_diff(weight, length);
    float key = key_ratio(weight, length);
    job_array.push_back(Job(key, weight, length));
  }

  //sort job_array
  std::sort(job_array.begin(), job_array.end(), decreasing_key_order());

  for (Job job_ : job_array){
    // std::printf("key %f, weight %d, length %d \n", job_.key, job_.weight, job_.length);
  }

  //sum the job array
  long total = sumWeightCompletionTime(job_array);
  std::cout << "total: " << total << std::endl;

  return 0;
}


float key_diff(int& weight, int& length){
  return weight - length;
}

float key_ratio(int& weight, int& length){
  return float(weight) /float(length);
}

//69119377652
long sumWeightCompletionTime(std::vector<Job>& job_array){
  long sum = 0;
  long culm_length = 0;
  for (Job job_ : job_array){
    culm_length += job_.length;
    sum += job_.weight * culm_length;
  }
  return sum;

}
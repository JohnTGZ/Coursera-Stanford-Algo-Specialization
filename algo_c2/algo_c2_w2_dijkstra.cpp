#include <vector>
#include <string>
#include <utility>
#include <algorithm>

#include <unordered_set>

#include <limits.h>
#include <unistd.h>
#include <iostream>

#include <fstream> //theres ifstream and ofstream too
#include <sstream>

#define INF 1000000

class Cell
{
    public:
        Cell(int a, int b): idx(a), f_cost(b) {}
        int idx;
        float f_cost;
        bool operator==(const Cell &_cell) const {
            return idx == _cell.idx;
        }
};

struct greater { //min heap operator overload
    bool operator()(const Cell& a, const Cell& b) const {
        return a.f_cost > b.f_cost;
    }
};

class Dijkstra{
    public:
        Dijkstra(std::string file_name_, int num_vertice_);
        Dijkstra(std::string file_dir_, std::string file_name_, int num_vertice);
        void preprocessGraph();
        bool computeShortestPath(int s_start, int s_goal);
        void clearAll();

        /**debug **/
        void testGraph (int vertex);

    private:
        std::string file_path;
        int num_vertice;

        /** string methods **/
        void eraseSubStr(std::string& mainStr, const std::string& toErase);
        std::string getexedir();

        /** graph **/
        std::vector< std::vector<std::pair<int, int > > > weights;
        std::vector<int> neighbors(int s);
        int cost(int s, int s_nb);
        void updateVertex(int s, int s_nb);
        void removeFromOpenList(int s);

        /** checks **/
        bool inOpenList(int s);
        bool isVisited(int s);
        
        
        /** data structures **/
        std::vector<Cell> OPENLIST;               //open list containing cell idx and f cost
        std::unordered_set<int> visited;
        std::vector<int> g_cost;
};

Dijkstra::Dijkstra(std::string file_name_, int num_vertice_){
    num_vertice = num_vertice_;
    file_path = file_name_;
    weights.resize(num_vertice);
    g_cost.resize(num_vertice);
}

Dijkstra::Dijkstra(std::string file_dir_, std::string file_name_, int num_vertice){
    file_path = file_dir_ + "/" + file_name_;
    weights.resize(num_vertice);
}

void Dijkstra::eraseSubStr(std::string& mainStr, const std::string& toErase)
{
    size_t pos = mainStr.find(toErase);
    
    if (pos != std::string::npos)
    {
        mainStr.erase(pos, toErase.length());
    }
}

std::string Dijkstra::getexedir()
{
    char result[ PATH_MAX ];
    ssize_t count = readlink( "/proc/self/exe", result, PATH_MAX );
    std::string exec_dir = std::string( result, (count > 0) ? count : 0 );
    eraseSubStr(exec_dir, "a.out");
    return exec_dir;
}

void Dijkstra::preprocessGraph(){

    std::ifstream inFile;
    inFile.open(file_path);

    if (!inFile) {
        std::cerr << "unable to open file at " << file_path << std::endl;
        exit(1);
    }

    // std::cout << "//////////////////////////////////" << std::endl;
    // std::cout << "Preprocessing graph from file "  << file_path << std::endl;
    // std::cout << "__________________________________" << std::endl;

    std::string line;
    int source_vertex;

    std::string vertex_weight_pair;
    std::string vertex_scalar;

    while (std::getline(inFile, line, '\n')){                                         //per line of text
            
        std::istringstream source_vertex_line(line);

        //extract first token (the source vertice)
        source_vertex_line >> source_vertex; 
        // std::cout<< "__" << source_vertex << "__" << std::endl;

        while(std::getline(source_vertex_line, vertex_weight_pair, '\t')){   //per vertex and weight pair

            std::stringstream connected_vertex(vertex_weight_pair);

            std::vector<std::string> v_w_pair;

            v_w_pair.clear();

            int itr = 0;
            while (std::getline(connected_vertex, vertex_scalar, ',' )) {    //per vertex_scalar
                v_w_pair.push_back(vertex_scalar);
                
                // std::cout << itr << ": "<< v_w_pair[itr] << std::endl;
                itr ++;
            }

            if(itr == 2){
                weights[source_vertex-1].push_back(std::pair<int, int>(std::stoi(v_w_pair[0]), 
                                                                        std::stoi(v_w_pair[1]) ));
            }
            else{
                // std::cout << "incomplete vertex" << std::endl;
            }
        }
            
    }
    // std::cout << "__________________________________" << std::endl;
    // std::cout << "END" << std::endl;
    // std::cout << "//////////////////////////////////" << std::endl;

    inFile.close();
}

void Dijkstra::testGraph (int vertex){
    // std::cout << "test graph for vertex " << vertex << std::endl;
    for (std::pair<int, int> pair_ : weights[vertex-1]){
        // std::cout << pair_.first << ", " << pair_.second << std::endl;
    }
    
}


void Dijkstra::removeFromOpenList(int s){
    Cell item = Cell(s, g_cost[s-1]);
    std::vector<Cell>::iterator position = std::find(OPENLIST.begin(), OPENLIST.end(), item );
    if(position != OPENLIST.end()){
        OPENLIST.erase(position);
    }
    // std::vector<Cell> &openlist = OPENLIST;
    // openlist.erase(std::remove(openlist.cbegin(), openlist.cend(), item), openlist.end());
}

void Dijkstra::updateVertex(int s, int s_nb){
    // std::cout << "cost(" << s << ", " << s_nb << ")= " << cost(s, s_nb) << std::endl;

    if (g_cost[s-1] + cost(s, s_nb) <= g_cost[s_nb-1]){
        g_cost[s_nb-1] = g_cost[s-1] + cost(s, s_nb);
        // if (inOpenList(s_nb)){
        //     removeFromOpenList(s_nb);
        // }
        // std::cout<<"pushing to open list cell "<< s_nb << std::endl;
        OPENLIST.push_back(Cell(s_nb, g_cost[s_nb-1]) ); 
        std::push_heap(OPENLIST.begin(), OPENLIST.end(), greater()); //sorts last element into the minheap
    }

    // g_cost[s_nb-1] = g_cost[s-1] + cost(s, s_nb);
    // OPENLIST.push_back(Cell(s_nb, g_cost[s_nb-1]) ); 
    // std::push_heap(OPENLIST.begin(), OPENLIST.end(), greater()); //sorts last element into the minheap

}

bool Dijkstra::isVisited(int s){
    if (visited.count(s)) {
        return true;
    }
    else 
        return false;
}

bool Dijkstra::inOpenList(int s){
    for (Cell s_ : OPENLIST ){
        if (s_.idx == s){
            return true;
        } 
    }
    return false;
}

std::vector<int> Dijkstra::neighbors(int s){
    std::vector<int> s_nb_vec;
    for (std::pair<int, int> pair_ : weights[s-1]){
        s_nb_vec.push_back(pair_.first);
    }
    return s_nb_vec;
}

int Dijkstra::cost(int s, int s_nb){
    for (std::pair<int, int> pair_ : weights[s-1]){
        if (pair_.first == s_nb){
            return pair_.second;
        }
    }
    // return INF;
}

void Dijkstra::clearAll(){
    g_cost.clear(); 
    g_cost.resize(num_vertice);
    std::fill(g_cost.begin(), g_cost.end(), INF);
    OPENLIST.clear();
    visited.clear();
}


bool Dijkstra::computeShortestPath(int s_start, int s_goal){
    // std::cout << "__________________________________" << std::endl;
    // std::cout << "Computing shortest path" << std::endl;
    // std::cout << "//////////////////////////////////" << std::endl;
    clearAll();
    g_cost[s_start-1] = 0;
    OPENLIST.push_back(Cell(s_start, 0));

    while (!OPENLIST.empty()){

        std::pop_heap(OPENLIST.begin(), OPENLIST.end(), greater());
        Cell s = OPENLIST.back();
        // std::cout<<"g_cost of "<< s.idx << std::endl;
        OPENLIST.pop_back();

        if (s.idx == s_goal){
            std::cout << "path found from " <<s_start << " to "<< s_goal <<"  with cost " << g_cost[s_goal-1] << std::endl;
            return true;
        }

        visited.insert(s.idx);
        for (int s_nb : neighbors(s.idx)){
            // std::cout << s_nb << " with cost [" << cost(s.idx, s_nb) <<"]"<< std::endl;

            if (!isVisited(s_nb)){
            //     // std::cout << s_nb << "not in closedlist" << std::endl;
                // if (!inOpenList(s_nb)){
                //     // std::cout << s_nb << "not in openlist" << std::endl;
                    updateVertex(s.idx, s_nb);
                // }
            }
        }
            
    }
    std::cerr << "path not found"<< std::endl;
    return false;
}


int main(int argc , char *argv[]){

    Dijkstra planner("dijkstraData.txt", 200);
    // Dijkstra planner("dijkstraData_testcase2.txt", 8);

    planner.preprocessGraph();
    planner.testGraph(4);
    std::vector<int> goals = {7,37,59,82,99,115,133,165,188,197};
    for (int goal : goals){
        planner.computeShortestPath(1, goal);
    }
    // std::vector<int> goals = {1,2,3,4,5,6,7,8};
    // for (int goal : goals){
    //     planner.computeShortestPath(1, goal);
    // }

    // planner.computeShortestPath(1, 6);
    
}
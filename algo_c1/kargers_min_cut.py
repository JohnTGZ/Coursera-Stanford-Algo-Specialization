#Written by JohnTGZ
#Karger's algorithm to determine the minimum cut of a graph

import numpy as np
import random
import time
import copy

def merge(u,v,graph):

    for node in graph[v]:
        if (node != u+1):
            graph[u].append(node) #add node to u if node!= u to prevent self loops
            graph[node-1].append(u+1) #append u to each of the nodes as u is now connected to the nodes
        graph[node-1].remove(v+1) #remove v from each of the nodes as v is being absorbed
        
    return graph

def random_contract_single(graph):
    vertice_list = list(range(0,200))
    random.seed(time.time())

    while len(vertice_list) > 2:
        #returns indexes u and v (not vertices!!)
        u = random.choice( vertice_list )
        v = random.choice( graph[u] ) - 1
        vertice_list.remove(v)
        graph = merge(u,v,graph)

    min_cut_ = len(graph[vertice_list[0]])
    return min_cut_
       
def adj_matrix(file, num_v):
    #construct adjacent matrix
    num_v = 200
    graph_ = []
    temp_list =[]

    vertice_idx = 0
    for line in file:
        for adj_vertice in line.split()[1:]:
            temp_list.append(int(adj_vertice))
        graph_.append(temp_list)
        temp_list =[]
        vertice_idx+=1
    return graph_

if __name__ == "__main__":
    file = open("data/kargerMinCut.txt",'r')
    num_v_ = 200
    graph_ = adj_matrix(file, num_v_)
    min_cut = list()

    #number of trials to carry out: should be (n**2)*ln(n) times to 
    #guarantee a (n-1)/n chance of getting the minimum cut
    num_trials = 200

    for itr in range(0, num_trials):
        graph = copy.deepcopy(graph_)
        cut = random_contract_single(graph)
        min_cut.append(cut)
        print(cut)

    print("min_cut: ", min(min_cut))


        
        



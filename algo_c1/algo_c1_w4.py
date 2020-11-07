import numpy as np
import random

def random_pick(vertice_list):
    choices = random.sample(vertice_list, k = 2)
    vertice_list.remove(choices[0])
    vertice_list.remove(choices[1])
    return choices[0], choices[1], vertice_list

def merge(u,v,graph):
    #take the place of u
    graph[u] = 

def random_contract( graph, num_v, seed):
    vertice_list = range(0,num_v)
    random.seed(seed)
    while num_v > 2:
        #pick an edge uniformly at random
        u, v,vertice_list = random_pick(vertice_list)
        #contract u and v into single
        graph = merge(u,v,graph)
        num_v -=1
        #remove self loops
        

def adj_matrix(file, num_v):
    num_v = 200
    graph = np.zeros([num_v, num_v])
    
    vertice_idx = 0
    for line in file:
        for adj_vertice in line.split()[1:]:
            graph[vertice_idx][int(adj_vertice)-1] += 1
        vertice_idx+=1
    return graph


file = open("/home/john/projects/coursera_algo/kargerMinCut.txt",'r')
num_v = 200

# adj_matrix(file, num_v)
graph = adj_matrix(file, num_v)
print(graph[0])


        
        



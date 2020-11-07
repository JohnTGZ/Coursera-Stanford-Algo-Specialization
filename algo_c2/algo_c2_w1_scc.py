import numpy as np
import os
import sys

def preprocessGraph(file_name, num_nodes):

    file = open(file_name,'r')

    #make a list of different lists
    G = [ [] for i in range(num_nodes)] 
    G_rev = [ [] for i in range(num_nodes)] 

    for line in file:
        vertices = line.split()     #split by whitespace
        root_vertice = int(vertices[0])
        child_vertice = int(vertices[1])
        G[root_vertice]      += [child_vertice]
        G_rev[child_vertice] += [root_vertice]

    return G,G_rev

if __name__ == "__main__":
    dirname = os.path.dirname(__file__)
    filename = os.path.join(dirname, 'scc_test1.txt')

    num_nodes = 10

    print("Preprocessing graph...")
    G, G_rev = preprocessGraph(filename, num_nodes)

    #data structures
    stack = []
    order = [] #from first to last
    f = [0] * num_nodes
    parent = [0] * num_nodes
    visited = [False] * num_nodes #represents the node. If node i is unvisited then visited[i] == False
    leader = [0] * num_nodes 
    scc = [0] * num_nodes #The index is the scc leader and the value is the size of the scc.

    #DFS on reverse graph
    print("Starting DFS on reverse graph...")

    print("G_rev", G_rev)


    t = 0

    for i in range(num_nodes-1, 0, -1):
        if not ( visited[i] ):  # if i not yet visited
            pred = 0
            s = i
            stack.append(i)

            #START OF LOOP
            while stack:
                i = stack.pop() 
                print("LOOP 1, Currently visiting: ", i)
                
                if (visited[i]):
                    continue
                visited[i] = True  

                # t += 1

                if (not G_rev[i]): 
                    order.append(i)
                    # f[i] = t
                    print("___LOOP 1, Appended to order: ", i)
                    
                else: 
                    for j in G_rev[i]:
                        if (visited[j]): 
                            # f[i] = t
                            
                            order.append(i)
                            stack.append(parent[i])
                            print("______LOOP 1, Appended to order: ", i)
                        else: 
                            stack.append(j) 
                            print("______LOOP 1, Appended to stack: ", j)

    print("order:", order)

    # #DFS on forward graph
    # print("Starting DFS on forward graph...")

    # visited = [False] * len(visited) #reset visited cells
    # order.reverse() #reverse the order so that now the last node to finish starts first

    # for i in order:
    #     if not ( visited[i] ):  # if i not yet visited
    #         s = i
    #         stack.append(i)

    #         #START OF LOOP
    #         while stack:
    #             i = stack.pop()

    #             leader[i] = s

    #             if (visited[i]):
    #                 continue
                
    #             visited[i] = True  # mark i as explored
    
    #             if (not G[i]): # if i is sink vertex, then i will finish here
    #                 continue
    #             else:  #else continue on to child vertices
    #                 for j in G[i]:
    #                     if not(visited[j]): #if j not visited
    #                         stack.append(j) 

    # print("number of zeros in leader: ", leader.count(0))
    
    # print("Starting to get SCCs...")
    # #get the SCCs
    # for leader_node in leader:
    #     if leader_node == 0:
    #         continue
    #     else:
    #         scc[leader_node] += 1

    # print("Starting to sort SCCs...")
    # scc.sort(reverse=True) #sort from smallest to largerst
    # print(scc[:5]) #get the five biggest SCCs





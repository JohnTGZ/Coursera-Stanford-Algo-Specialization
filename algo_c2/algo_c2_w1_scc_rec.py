import sys, threading
import os
sys.setrecursionlimit(800000)
threading.stack_size(67108864)

num_nodes = 875714 + 1 #875714
t = 0

visited = [False] * num_nodes #represents the node. If node i is unvisited then visited[i] == False
f = [0] * num_nodes #represents the node. If node i is unvisited then visited[i] == False
leader = [0] * num_nodes
order = []
scc = [0] * num_nodes

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

def dfs1(G_rev, i):
    global order
    # global t
    visited[i] = True
    for j in G_rev[i]:
        if (not visited[j]):
            dfs1(G_rev, j)
    # t += 1
    # f[i] = t
    order.append(i)

def dfs_loop1(G_rev):
    for i in range(num_nodes-1, 0, -1):
        if not visited[i]:
            dfs1(G_rev, i)
    

def dfs2(G, i, s):
    visited[i] = True
    leader[i] = s
    for j in G[i]:
        if (not visited[j]):
            dfs2(G, j, s)

def dfs_loop2(G, order):
    for i in order:
        if not visited[i]:
            s = i
            dfs2(G, i, s)

def main():
    dirname = os.path.dirname(__file__)
    filename = os.path.join(dirname, 'SCC.txt')

    print("Preprocessing graph...")
    G, G_rev = preprocessGraph(filename, num_nodes)

    #first loop
    dfs_loop1(G_rev)

    order.reverse()
    # print(order)

    global visited
    visited = [False] * num_nodes 
    #second loop
    dfs_loop2(G, order)

    for leader_vertex in leader:
        if (leader_vertex != 0):
            scc[leader_vertex] += 1

    scc.sort(reverse=True)
    print(scc[:5])

thread = threading.Thread(target=main)
thread.start()
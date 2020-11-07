import numpy as np
import os
import sys


def preprocessGraph(file_name, num_v):

    file = open(file_name,'r')

    #make a list of different lists
    G = [ [] for i in range(num_v)] 
    G_rev = [ [] for i in range(num_v)] 

    for line in file:
        vertices = line.split()     #split by whitespace
        root_vertice = int(vertices[0])
        child_vertice = int(vertices[1])
        G[root_vertice]      += [child_vertice]
        G_rev[child_vertice] += [root_vertice]

    return G,G_rev

def writeGraph(output_filename, G, G_rev):
    with open(output_filename + '_forward.txt', 'w') as filehandle:
        for list in G:
            filehandle.write('%s\n' % list)


if __name__ == "__main__":
    dirname = os.path.dirname(__file__)
    filename = os.path.join(dirname, 'SCC.txt')

    num_v = 875715

    G, G_rev = preprocessGraph(filename, num_v)

    writeGraph('SCC_preprocessed', G, G_rev)






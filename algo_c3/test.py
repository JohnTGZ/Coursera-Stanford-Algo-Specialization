f = open('clustering_big_test.txt', 'r')

f.readline()
ls = f.readline()
graph_bin = []
while ls:
    graph_bin += [ls[:-1].replace(' ', '')]
    ls = f.readline()
f.close()

graph_bin = list(set(graph_bin))

num_nodes = len(graph_bin)
print("num of nodes: ", num_nodes)

print("graph_bin type: ",type(graph_bin))
print("graph_bin ", graph_bin)

graph_dec = [int(i, 2) for i in graph_bin]

print("graph_dec type: ",type(graph_dec))
print("graph_dec ", graph_dec)

#dictionary mapping each node id to a distance
uf = {i : i for i in graph_dec}
rank = {i : 0 for i in graph_dec}

print("uf", uf)
print("rank", rank)

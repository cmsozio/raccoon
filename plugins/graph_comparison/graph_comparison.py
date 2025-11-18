import igraph as ig

def compare(graph1: ig.Graph, graph2: ig.Graph) -> None:
    bfs1 = graph1.bfsiter(5, mode="out")
    bfs2 = graph2.bfsiter(5, mode="out")
    for i in range(0, 4):
        print("Graph 1: {}".format(next(bfs1)))
        print("Graph 2: {}".format(next(bfs2)))
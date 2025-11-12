import argparse
import raccoon
import networkx as nx
from networkx.drawing.nx_agraph import read_dot
from plugins.cotd.src.cotd import COTD, COTDGML
import igraph as ig
import shutil

if __name__ == "__main__":
    print("Setting up...")
    arg_parser = argparse.ArgumentParser()

    arg_parser.add_argument("-n", "--netlist", type=str, help="Path to the netlist file")
    arg_parser.add_argument("-o", "--output", type=str, help="Output file format (json or dot)")
    arg_parser.add_argument("--second", type=str, help="Other netlist to compare against")
    arg_parser.add_argument("--cotd", action="store_true", help="Run COTD analysis")
    arg_parser.add_argument("--comparison", action="store_true", help="Run a comparison between two graphs")
    arg_parser.add_argument("-g", "--graph", type=str, help="Path to the graph file (e.g. .dot, .json, .gml)")

    args = arg_parser.parse_args()

    graph = None

    if args.graph == None:
        print("Parsing netlist...")
        raccoon.parse_netlist(args.netlist, args.output)
        graph = ig.Graph.Read("output.gml", format="graphml")
        if args.second != None:
            shutil.move("output.gml", "first_output.gml")
            print("Parsing second netlist...")
            raccoon.parse_netlist(args.second, args.output)
            shutil.move("output.gml", "second_output.gml")
            graph2 = ig.Graph.Read("second_output.gml", format="graphml")
            shutil.move("first_output.gml", "output.gml")
    else:
        graph = ig.Graph.Read(args.graph, format="graphml")

    
    if args.comparison:
        bfs = graph2.bfsiter(5, mode="out")
        for i in range(0, 4):
            print(next(bfs))

    #print(graph.vs[3])
    #print(graph.incident(3, mode='all'))
    #print(graph.es[0])
    #for i in range(0, graph.vcount()):
    #    if graph.vs[i].predecessors() == []:
    #        print("No predecessors for node {}".format(graph.vs[i]["weight"]))

    if args.cotd:
        print("Running COTD...")
    
        if args.output == "dot":
            cotd = COTD(dot="output.dot")
            cotd.cc()
            cotd.co()

            cc_points = cotd.compute_cc_points()

            kmeans = cotd.k_means_algo()
            cotd.draw_scatter(kmeans, "COTD K-Means Clustering")
            cotd.draw()

        elif args.output == "json":
            cotd = COTD(json_file="dump.json")

            cotd.cc()
            cotd.co()

            cc_points = cotd.compute_cc_points()

            kmeans = cotd.k_means_algo()

            #title = input("Enter title for the clustering plot: ")
            title = "blah"
            cotd.draw_scatter(kmeans, "COTD K-Means Clustering for {}".format(title))
            cotd.print_combinational()

        elif args.output == "gml":
            cotd = COTDGML(graph_file="output.gml", cell_list_file="cells.json")
            cotd.cc()
            cotd.co()
            kmeans = cotd.k_means_algo()

            cotd.draw_scatter(kmeans, "Test")
            #for pair in cotd.combinational:
            #    print("{}: {}".format(pair, cotd.combinational[pair]))
            
        else:
            if args.graph:
                cotd = COTDGML(graph_file=args.graph, cell_list_file="cells.json")
                cotd.cc()
                cotd.co()
                kmeans = cotd.k_means_algo()

                cotd.draw_scatter(kmeans, "Test")
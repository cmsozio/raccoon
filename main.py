import argparse
import raccoon
import networkx as nx
from networkx.drawing.nx_agraph import read_dot
from plugins.cotd.src.cotd import COTD
import igraph as ig

if __name__ == "__main__":
    print("Setting up...")
    arg_parser = argparse.ArgumentParser()

    arg_parser.add_argument("netlist", type=str, help="Path to the netlist file")
    arg_parser.add_argument("-o", "--output", type=str, help="Output file format (json or dot)")
    arg_parser.add_argument("--cotd", action="store_true", help="Run COTD analysis")

    args = arg_parser.parse_args()

    print("Parsing netlist...")
    raccoon.parse_netlist(args.netlist, args.output)

    graph = ig.Graph.Read("output.gml", format="graphml")
    print(graph)

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

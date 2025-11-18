import argparse
import raccoon
from plugins.cotd.src.cotd import COTD, COTDGML
from plugins.graph_comparison.graph_comparison import compare
import igraph as ig

def parse_netlist(netlist_path: str, output_format: str) -> None:
    print("Parsing netlist {} ...".format(netlist_path))
    raccoon.parse_netlist(netlist_path, output_format)

def run_cotd(output: str, filename: str) -> None:
    print("Running COTD analysis...")
    cotd = None

    if output == "dot":
        cotd = COTD(dot=filename + ".dot")
    elif output == "json":
        cotd = COTD(json_file=filename + ".json")
    elif output == "gml":
        cotd = COTDGML(graph_file=filename + ".gml", cell_list_file="cells.json")
    
    cotd.cc()
    cotd.co()

    cc_points = cotd.compute_cc_points()

    kmeans = cotd.k_means_algo()

    title = "COTD Analysis On " + filename + " Netlist"
    cotd.draw_scatter(kmeans, title)

def handle_args(args: argparse.Namespace) -> None:
    filename = None
    graph = None
    graph2 = None

    if args.netlist != None:
        filename = args.netlist.split("/")[-1].split(".")[0]
    elif args.graph == None:
        print("ERROR: No netlist or graph file provided.")
        return

    format = {"dot": "dot", "json": "json", "gml": "graphml"}

    if args.graph == None:
        parse_netlist(args.netlist, args.output)
        if args.second != None:
            parse_netlist(args.second, args.output)
            second_filename = args.second.split("/")[-1].split(".")[0]
            graph2 = ig.Graph.Read(second_filename + "." + args.output, format=format[args.output])
        graph = ig.Graph.Read(filename + "." + args.output, format=format[args.output])
    else:
        graph = ig.Graph.Read(args.graph, format=format[args.output])

    
    if args.comparison:
        compare(graph, graph2)

    if args.cotd:
        run_cotd(args.output, filename)

if __name__ == "__main__":
    print("Setting up...")
    arg_parser = argparse.ArgumentParser()

    arg_parser.add_argument("-n", "--netlist", type=str, help="Path to the netlist file")
    arg_parser.add_argument("-o", "--output", type=str, help="Output file format (json, dot, or gml)")
    arg_parser.add_argument("-g", "--graph", type=str, help="Path to the graph file (e.g. .dot, .json, .gml)")
    arg_parser.add_argument("-s", "--second", type=str, help="Other netlist to compare against")
    arg_parser.add_argument("--cotd", action="store_true", help="Run COTD analysis")
    arg_parser.add_argument("--comparison", action="store_true", help="Run a comparison between two graphs.\n Requires two netlists to be provided and both in the format graphml (.gml).")

    args = arg_parser.parse_args()

    graph = None

    handle_args(args)
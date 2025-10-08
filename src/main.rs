/*
MIT License

Copyright (c) 2025 Christopher Sozio

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::env;
use std::process;
use parrot;
use petgraph::{dot::Dot, graph::NodeIndex, Graph};
use pyo3::prelude::*;

use crate::graph_analysis::GraphAnalysis;

mod graph_analysis;


fn main() {

    // Default output format is DOT
    let mut option = "dot";

    let args= env::args();
    let args: Vec<String> = args.collect();
    if args.len() < 2 {
        eprintln!("\nNeed to provide gate-level netlist.\n");
        process::exit(1);
    } else if args.len() == 3 {
        option = &args[2];
    }

    let file = &args[1];

    let mut parser = parrot::VerilogParser::new(file);
    parser.parse();
    
    let verbosity: u8 = 0;
    let include_sum: bool = true;
    let mut netlist_graph = parrot::NetlistGraph::new(parser, verbosity, include_sum);
    netlist_graph.setup();

    if option == "dot" {
        netlist_graph.write_dot();
    } else if option == "json" {
        netlist_graph.vp.top_module_jsonify();
    } else if option == "analyze" {
        let mut ga = GraphAnalysis::new(netlist_graph.netgraph);
        ga.setup_node_map();
        let list = ga.user_get_neighbors("g193333");
        println!("\nNeighbors of 'g193333': {:?}", list);
        let list = ga.user_logic_cone("g193333", 1, "in");
        println!("\nLogic cone of 'g193333': {:?}", list);
        //ga.print_node_map();
        //ga.analyze();
    } else {
        eprintln!("\nUnsupported option format: {}\n", option);
        process::exit(1);
    }


}

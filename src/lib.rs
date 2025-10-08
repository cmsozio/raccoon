use std::env;
use std::process;
use parrot;
use parrot::VerilogParser;
use petgraph::{dot::Dot, graph::NodeIndex, Graph};
use pyo3::prelude::*;

use crate::graph_analysis::GraphAnalysis;

mod graph_analysis;

#[pyfunction]
fn parse_netlist(path: String, option: String) -> PyResult<()> {
    let mut parser = parrot::VerilogParser::new(&path);
    parser.parse();

    let verbosity: u8 = 0;
    let include_sum: bool = true;
    let mut netlist_graph = parrot::NetlistGraph::new(parser, verbosity, include_sum);
    netlist_graph.setup();

    if option == "dot" {
        netlist_graph.write_dot();
    } else if option == "gml" {
        netlist_graph.write_gml();
    } else if option == "json" {
        netlist_graph.vp.top_module_jsonify();
    } else {
        eprintln!("\nUnsupported option format: {}\n", option);
        process::exit(1);
    }

    Ok(())
}

#[pymodule]
fn raccoon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_netlist, m)?)?;
    Ok(())
}
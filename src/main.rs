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

fn main() {

    // Default output format is DOT
    let mut output = "dot";

    let args= env::args();
    let args: Vec<String> = args.collect();
    if args.len() < 2 {
        eprintln!("\nNeed to provide gate-level netlist.\n");
        process::exit(1);
    } else if args.len() == 3 {
        output = &args[2];
    }

    let file = &args[1];

    let mut parser = parrot::VerilogParser::new(file);
    parser.parse();

    if output == "dot" {
        let verbosity: u8 = 0;
        let include_sum: bool = true;
        let mut netlist_graph = parrot::NetlistGraph::new(parser, verbosity, include_sum);
        netlist_graph.setup();
        netlist_graph.write_dot();
    } else if output == "json" {
        parser.top_module_jsonify();
    } else {
        eprintln!("\nUnsupported output format: {}\n", output);
        process::exit(1);
    }

}

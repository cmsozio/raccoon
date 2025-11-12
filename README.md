# Rapid Analysis of Circuit Cells and Observation of Obscure Nets (RACCOON)

The Rapid Analysis of Circuit Cells and Observation of Obscure Nets (RACCOON) tool provides analysis for gate-level netlists to help identify obscure or stealthy circuits embedded into the netlist. RACCOON includes a number of plugins to allow a thorough analysis of the provided netlist. From some known hardware Trojan detection algorithms like COTD, FANCI, and UCI to more general netlist analysis.

The RACCOON tool utilizes the Parrot netlist parser to create a graph representation of the netlist used for most analysis algorithms.

The tool accepts only Verilog gate-level netlists. It cannot handle netlists at the register transfer level. The only active plugin for the RACCOON tool is the COTD algorithm.

## Environment Setup

### Submodules

First you will need to pull in the necessary submodules `parrot` and `cotd` to be able to use them appropriately. This can be achieved through the use of the make tag `submodule` as seen below.

```bash
make submodule
```

### Python Virtual Environment

After pulling in the submodules, then you can setup the local Python virtual environment through the use of the make tag `setup` as seen below. This should create a Python virtual environment `.venv`, as well as create Python bindings for the Parrot Rust codebase.

```bash
make setup
```

To then run the RACCOON interface ensure that you have sourced the virtual environment.

```bash
source .venv/bin/activate
```

## Running RACCOON

Currently the tool is ran through the command line interface by calling on the `main.py` file.

```bash
python3 main.py <args>
```

- `-n` : the target gate-level netlist. Verilog is the only supported language
- `-o` : output file format for parsed netlist (`dot`, `gml`, and `json`)
- `--cotd` : Run the COTD plugin on the parsed netlist

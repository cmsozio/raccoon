submodule:
	git submodule update --init --recursive

build:
	cargo build

develop:
	maturin develop

setup: submodule
	python3 -m venv .venv
	. .venv/bin/activate; pip install -r requirements.txt
	. .venv/bin/activate; make develop

clean:
	rm -rf target
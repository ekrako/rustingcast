all: build install

build:
	maturin build

install:
	pip3 install --user --force-reinstall `ls ./target/wheels/fibrs*.whl`
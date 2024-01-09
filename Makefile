.SILENT:
.DEFAULT_GOAL:=run
SHELL:=/usr/bin/bash

.PHONY: run clean build

# for dev only
run:
	python usage.py

build:
	maturin develop

clean:
	echo 'nothing to clean yet'

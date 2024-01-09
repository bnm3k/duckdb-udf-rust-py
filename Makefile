.SILENT:
.DEFAULT_GOAL:=dev
SHELL:=/usr/bin/bash

.PHONY: dev clean build

# for dev only
dev: .dev
	python example.py

.dev: $(shell find src -type f)
	maturin develop
	touch .dev

clean:
	rm .build

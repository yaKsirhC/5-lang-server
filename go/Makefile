###############################################
#
# Makefile
#
###############################################

.DEFAULT_GOAL := run

.PHONY: dist

run:
	go run .

build:
	go build -ldflags="-s -w" -o main .

dist: clean lint format

clean:
	rm -f main

lint:
	go vet

format:
	go fmt

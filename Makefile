RUSTC ?= rustc

SRC = $(shell find src -name '*.rs')

all: clean interpreter

interpreter: $(SRC)
	mkdir -p target
	$(RUSTC) --out-dir target $(SRC)

test: $(SRC)
	mkdir -p target
	$(RUSTC) --test --out-dir target $(SRC)
	./target/interpreter

run:
	./target/interpreter

clean:
	@rm -rf target

.PHONY: clean
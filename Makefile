# https://docs.rs/crate/neon-sys/0.1.2/source/build/binding.Makefile
export builddir_name ?= ./build/.

.PHONY: all

all:
	$(MAKE) neon

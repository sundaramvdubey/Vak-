.PHONY: help fmt test check examples build install clean

help:
	@printf '%s\n' \
	  'make test      Run formatting checks and all Rust tests' \
	  'make check     Check the hello example with Vāk' \
	  'make examples  Build the runnable example programs' \
	  'make build     Build the optimized Vāk compiler' \
	  'make install   Install Vāk into ~/.local/bin'

fmt:
	cargo fmt -- --check

test: fmt
	cargo test

check: build
	./target/release/vak check examples/hello.vak

examples: build
	./target/release/vak build examples/native.vak /tmp/vak-native
	./target/release/vak build examples/control.vak /tmp/vak-control
	./target/release/vak build examples/aggregates.vak /tmp/vak-aggregates
	./target/release/vak build examples/for_range.vak /tmp/vak-for-range
	./target/release/vak build examples/strings.vak /tmp/vak-strings

build:
	cargo build --release

install: build
	mkdir -p "$(HOME)/.local/bin"
	cp target/release/vak "$(HOME)/.local/bin/vak"
	@printf '%s\n' 'Installed Vāk to $(HOME)/.local/bin/vak'
	@printf '%s\n' 'Add $(HOME)/.local/bin to PATH if the vak command is not found.'

clean:
	cargo clean

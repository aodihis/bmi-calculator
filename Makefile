CARGO = cargo
CARGO_WATCH = cargo watch
APP_NAME = bmi_calculator

run:
	$(CARGO) run

release:
	$(CARGO) run --release

build:
	$(CARGO) build

build-release:
	$(CARGO) build --release

watch:
	$(CARGO_WATCH) \
		-w src/ \
		-w .env \
		-x clippy \
		-x 'test -- --nocapture' \
		-x 'run -- --color=always'

clippy:
	$(CARGO) clippy -- -D warnings

fmt:
	$(CARGO) fmt --all -- --check

lint: fmt clippy

test:
	$(CARGO) test -- --nocapture

install-tools:
	$(CARGO) install cargo-watch
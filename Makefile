include .env

.PHONY: codeline
codeline:
	@tokei .

.PHONY: initdb
initdb:
	@docker run \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_DB=filebox \
    -p 5432:5432 \
    -d \
    --name=postgres-docker \
    postgres -N 1000

.PHONY: test 
test: fmt
	@cargo nextest run

.PHONY: fmt
fmt:
	@cargo fmt && cargo fmt -- --check && cargo clippy --all-targets --all-features --tests --benches -- -D warnings

.PHONY: run
run:
	@cargo run -p server --bin filebox-server


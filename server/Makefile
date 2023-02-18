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

.PHONY: test/integration
test/integration:
	@cargo test --lib -- tests::filebox::tests::test_filebox_lifecycle --exact --nocapture
	@cargo test --lib -- tests::general::tests::test_health --exact --nocapture

.PHONY: test 
test: fmt
	@cargo nextest run

.PHONY: fmt
fmt:
	@cargo fmt && cargo fmt -- --check && cargo clippy --all-targets --all-features --tests --benches -- -D warnings

.PHONY: run
run:
	@cargo run --bin filebox-server

.PHONY: watch
watch: fmt
	@RUST_BACKTRACE=1 cargo watch -x run

build-dev: clear-none-docker-image test
	@docker-compose build 

up-dev: test
	@docker-compose up server

down-dev:
	@docker-compose down server

clear-none-docker-image:
	@docker rm $(docker ps -a -q) 
	@docker rmi $(docker images -f "dangling=true" -q)

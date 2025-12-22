# make run year=2019 day=4

download:
	cargo run -- download --year $(year) --day $(day)

run:
	cargo run -- run --year $(year) --day $(day)

test:
	cargo test --package aoc$(year) --lib -- day$(day)::tests --nocapture

gen:
	cargo run -- gen --year $(year) --day $(day)
	cargo run -- download --year $(year) --day $(day)

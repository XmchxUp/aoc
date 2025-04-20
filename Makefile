# make run year=2019 day=4

run:
	cargo run -- run --year $(year) --day $(day)

gen:
	cargo run -- gen --year $(year) --day $(day)
	cargo run -- download --year $(year) --day $(day)



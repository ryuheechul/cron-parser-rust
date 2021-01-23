.PHONY: run
run: build
	docker-compose run rust

.PHONY: test
test: build
	docker-compose run test

.PHONY: build
build:
	docker-compose build

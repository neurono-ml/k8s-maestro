BINARY_NAME := $$(cat Cargo.toml | grep name | head -n 1 | awk '{print $$3}' | sed -r 's/^"|"$$//g')
PROJECT_VERSION := $$(cat Cargo.toml | grep version | head -n 1 | awk '{print $$3}' | sed -r 's/^"|"$$//g')
GIT_REFERENCE := $$(git log -1 --pretty=%h)

REGISTRY=ghcr.io
NAMESPACE=kognitalab
IMAGE_REPOSITORY=$(REGISTRY)/$(NAMESPACE)/$(BINARY_NAME)
IMAGE_NAME=$(IMAGE_REPOSITORY):$(PROJECT_VERSION)


target/release:
	git push
	git tag v$(PROJECT_VERSION) --force
	git push --tags --force

clean:
	rm -rf target/release

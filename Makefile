# Only the autonomous Mac worker invokes this versioned compilation contract.
SHELL := /bin/bash
.PHONY: build
build:
	@/bin/bash tools/build/recipe.sh

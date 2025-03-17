# Compiler and flags
CARGO := cargo
RELEASE_FLAGS := --release
TARGET_DIR := target

# Main commands
.PHONY: all build clean test run check fmt lint help

all: build

# Build the project
build:
	$(CARGO) build $(RELEASE_FLAGS)

# Build with release optimizations
release:
	$(CARGO) build $(RELEASE_FLAGS)

# Clean build artifacts
clean:
	$(CARGO) clean
	rm -rf $(TARGET_DIR)

# Run tests
test:
	$(CARGO) test

# Run the project
run:
	$(CARGO) run $(RELEASE_FLAGS)

# Check for compilation errors without building
check:
	$(CARGO) check

# Format code using rustfmt
fmt:
	$(CARGO) fmt

# Run clippy for linting
lint:
	$(CARGO) clippy -- -D warnings

# Install dependencies
deps:
	$(CARGO) fetch

# Update dependencies
update:
	$(CARGO) update

# Show help
help:
	@echo "Available targets:"
	@echo "  all         - Build the project (default)"
	@echo "  build       - Build the project in debug mode"
	@echo "  release     - Build the project with optimizations"
	@echo "  clean       - Remove build artifacts"
	@echo "  test        - Run tests"
	@echo "  run         - Run the project"
	@echo "  run-release - Run the project with optimizations"
	@echo "  check       - Check for compilation errors"
	@echo "  fmt         - Format code using rustfmt"
	@echo "  lint        - Run clippy linter"
	@echo "  deps        - Install dependencies"
	@echo "  update      - Update dependencies"
	@echo "  help        - Show this help message"
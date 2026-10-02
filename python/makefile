VENV := .venv
PYTHON := python3
BIN := $(VENV)/bin

.PHONY: setup install clean

setup-python: install
	@echo "Python environment ready."

setup:
	@if [ ! -d "$(VENV)" ]; then \
		echo "Creating virtual environment..."; \
		$(PYTHON) -m venv $(VENV); \
	fi

install: setup
	@if [ ! -x "$(BIN)/ipython" ]; then \
		echo "Installing ipykernel..."; \
		$(BIN)/python -m pip install --upgrade pip ipykernel; \
	fi

clean:
	rm -rf $(VENV)
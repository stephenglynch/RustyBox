CARGO = cargo
VENV = venv
PYTHON = $(VENV)/bin/python3
PIP = $(VENV)/bin/pip


all: rustybox

rustybox: 
	$(eval FEATURES = $(shell $(PYTHON) genfeatures.py))
	$(CARGO) build --features --no-default-features '$(FEATURES)'

$(VENV)/bin/activate: requirements.txt
	python3 -m venv $(VENV)
	$(PIP) install -r requirements.txt

menuconfig: $(VENV)/bin/activate
	$(PYTHON) -m menuconfig

clean:
	$(CARGO) clean
	rm -rf __pycache__
	rm -rf $(VENV)
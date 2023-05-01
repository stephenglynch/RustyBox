CARGO ?= cargo
VENV ?= venv
PYTHON ?= $(VENV)/bin/python3
PIP ?= $(VENV)/bin/pip
DOCKER ?= docker
KCONFIG_CONFIG ?= .config

.PHONY: clean install menuconfig

all: rustybox

rustybox: $(KCONFIG_CONFIG)
	$(eval FEATURES = $(shell $(PYTHON) genfeatures.py))
	$(CARGO) build --no-default-features --features '$(FEATURES)'

$(KCONFIG_CONFIG):
	echo "Not configured, run 'make menuconfig'";\
	exit 1;

$(VENV)/bin/activate: requirements.txt
	python3 -m venv $(VENV)
	$(PIP) install -r requirements.txt

menuconfig: $(VENV)/bin/activate
	$(PYTHON) -m menuconfig

install: rustybox
	$(shell ./build_rootfs.sh)

clean:
	$(CARGO) clean
	rm -rf __pycache__
	rm -rf $(VENV)

docker_sh: install
	$(DOCKER) run -it --rm $(shell $(DOCKER) build . -q) /bin/sh
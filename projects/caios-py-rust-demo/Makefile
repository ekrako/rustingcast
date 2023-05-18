component := $(notdir $(PWD))

install:
	sudo yum install -y rustc cargo
	sudo pip3 install maturin

test_local: build_lib
	caios test run local

build_lib:
	(cd fibrs;$(MAKE))

test_remote: package upload deploy
	python3 -m unittest discover ./test/integrated/remote/

package:
	caios service package $(component) dev ./src/python/service.py ./output

copy_lib:
	cp -R ~/.local/lib/python$(PYTHON_VERSION)/site-packages/* ./output/python/

upload: copy_lib 
	caios service upload ./output $(CAIOS_ENV) $(component)

deploy:
	caios service deploy $(CAIOS_ENV) $(component) ./output/stack.json

shutdown: cleanup_lib
	caios test shutdown	

cleanup_lib:
	rm -fR ~/.local/lib/
	



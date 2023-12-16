TARANTOOL ?= /usr/bin/tarantool

procedures.so: lib.c
	cc -shared -o procedures.so -fPIC -O3 -std=c11 lib.c

clean:
	rm -rf data *.so

run: procedures.so
	$(TARANTOOL) -i init.lua

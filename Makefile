.PHONY: clean all

all: test.out

libtest.c.a:
	gcc -c src/lib.c
	ar -rc libtest.c.a lib.o
	rm lib.o

libtest.rs.a:
	rustc --crate-type=staticlib src/lib.rs -o libtest.rs.a


test.out: libtest.c.a libtest.rs.a
	gcc src/main.c -ltest.rs -L ./ -ltest.c -L ./ -o test.out -ldl -lpthread

clean: 
	rm test.out libtest.c.a libtest.rs.a
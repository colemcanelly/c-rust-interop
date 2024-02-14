LIBS = libtest_c.a libtest_rs.a
RUST_TARGET_DIR = ./target/debug

.PHONY: all clean
.DEFAULT: all

all: test.out

# General Rule for all object file creation
%.o: %.c
	gcc -c $^ -o $@


libtest_c.a: src/lib.o			# Rule for C library creation
	ar -rc $@ $^

libtest_rs.a:					# Rule for Rust library creation
	cargo build


test.out: $(LIBS)				# Final executable compilation
	gcc src/main.c -o $@ -L$(RUST_TARGET_DIR) -L./ -ltest_rs -ltest_c -ldl -lpthread

clean: 
	-rm $(LIBS)
	-rm src/*.o
	-rm test.out
.DEFAULT_GOAL := all

TARGETS = hello part1 part2

all: $(TARGETS)

$(TARGETS): %: %.o
	ld -s -o $@ $^

%.o: %.asm
	nasm -f elf64 -o $@ $^

.PHONY: clean
clean:
	rm -f *.o $(TARGETS)

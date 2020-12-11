FILE2UEF = $(HOME)/c/eek/src/eek-file2uef
XA = xa

all : day11.uef

day11.bin : electron.asm

%.bin : %.asm
	$(XA) -o $@ $<

day11.uef : loader.txt day11.bin input.txt
	$(FILE2UEF) -o $@ \
	-n day11 -t loader.txt \
	-n day11prog -l 0x1910 -e 0x1910 day11.bin \
	-n data input.txt

.PHONY : all clean

clean :
	rm -f *.uef *.bin

FILE2UEF = $(HOME)/c/eek/src/eek-file2uef
XA = xa

all : day6.uef

day6.bin : electron.asm

%.bin : %.asm
	$(XA) -o $@ $<

day6.uef : loader.txt day6.bin input.txt
	$(FILE2UEF) -o $@ \
	-n day6 -t loader.txt \
	-n day6prog -l 0x1910 -e 0x1910 day6.bin \
	-n data input.txt

.PHONY : all clean

clean :
	rm -f *.uef *.bin

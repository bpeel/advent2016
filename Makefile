CFLAGS = -Wall -g
PROGS = day5

all : $(PROGS)

day5 : day5.o
	$(CC) -o day5 $^ -lcrypto

.c.o :
	$(CC) -o $@ -c $< $(CFLAGS)

clean :
	rm -f $(PROGS) *.o

.PHONY : clean all

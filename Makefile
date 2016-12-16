CFLAGS = -Wall -g
PROGS = day5 day8-video day9 day11 day13 day14 day16

all : $(PROGS)

day5 : day5.o
	$(CC) -o day5 $^ -lcrypto

day8 : day8-video.o
	$(CC) -o day8 $^

day9 : day9.o
	$(CC) -o day9 $^

day11 : day11.o
	$(CC) -o day11 $^

day13 : day13.o
	$(CC) -o day13 $^

day14 : day14.o
	$(CC) -o day14 $^ -lcrypto

day16 : day16.o
	$(CC) -o day16 $^

.c.o :
	$(CC) -o $@ -c $< $(CFLAGS)

clean :
	rm -f $(PROGS) *.o

.PHONY : clean all

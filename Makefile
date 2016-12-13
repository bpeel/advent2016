CFLAGS = -Wall -g
PROGS = day5 day8-video day9 day11 day13 day13-part2

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

day13-part2 : day13-part2.o
	$(CC) -o day13-part2 $^

.c.o :
	$(CC) -o $@ -c $< $(CFLAGS)

clean :
	rm -f $(PROGS) *.o

.PHONY : clean all

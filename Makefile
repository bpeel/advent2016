CFLAGS = -Wall -g
PROGS = day5 day8-video

all : $(PROGS)

day5 : day5.o
	$(CC) -o day5 $^ -lcrypto

day8 : day8-video.o
	$(CC) -o day8 $^

.c.o :
	$(CC) -o $@ -c $< $(CFLAGS)

clean :
	rm -f $(PROGS) *.o

.PHONY : clean all

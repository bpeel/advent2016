CFLAGS = -Wall -g
CXXFLAGS = -Wall -g
PROGS = day1 day1-part2 day6 day7 day21

all : $(PROGS)

day1 : day1.o
	$(CC) -o day1 $^

day1-part2 : day1-part2.o
	$(CXX) -o day1-part2 $^

day6 : day6.o
	$(CXX) -o day6 $^

day7 : day7.o
	$(CC) -o day7 $^

day21 : day21.o
	$(CC) -o day21 $^

clean :
	rm -f $(PROGS) *.o

.PHONY : clean all

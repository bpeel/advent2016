CFLAGS = -Wall -g
CXXFLAGS = -Wall -g
PROGS = day1 day1-part2 day6 day7

all : $(PROGS)

day1 : day1.o
	$(CC) -o day1 $^

day1-part2 : day1-part2.o
	$(CXX) -o day1-part2 $^

day6 : day6.o
	$(CXX) -o day6 $^

day7 : day7.o fv-util.o fv-list.o
	$(CC) -o day7 $^

clean :
	rm -f $(PROGS) *.o

.PHONY : clean all

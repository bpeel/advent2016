#include <iostream>
#include <vector>
#include <cstdint>
#include <cstdlib>
#include <iterator>

static int
sum(const std::vector<int8_t> &digits,
    int offset)
{
        int sum = 0;

        for (size_t i = 0; i < digits.size(); i++) {
                int next = digits[(i + offset) % digits.size()];
                if (digits[i] == next)
                        sum += next;
        }

        return sum;
}

int
main(int argc, char **argv)
{
        typedef std::istream_iterator<char> iterator;
        std::vector<int8_t> digits;

        for (iterator it = std::cin; it != iterator(); ++it) {
                if (*it < '0' || *it > '9')
                        continue;

                digits.push_back(*it - '0');
        }

        std::cout << "Part 1 : " << sum(digits, 1) << std::endl;
        std::cout << "Part 2 : " << sum(digits, digits.size() / 2) << std::endl;

        return EXIT_SUCCESS;
}

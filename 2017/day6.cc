#include <iostream>
#include <map>
#include <algorithm>
#include <array>

#define N_BANKS 16

struct Configuration {
        std::array<int, N_BANKS> banks;

        bool operator<(const Configuration& other) const;
        bool operator==(const Configuration& other) const;
        void step();

        friend std::ostream& operator<<(std::ostream& os,
                                        const Configuration& dt);
};

static Configuration
initial_configuration = {
        { 4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5 }
};

bool
Configuration::operator<(const Configuration& other) const
{
        return std::lexicographical_compare(banks.begin(), banks.end(),
                                            other.banks.begin(),
                                            other.banks.end());
}

bool
Configuration::operator==(const Configuration& other) const
{
        return std::equal(banks.begin(), banks.end(), other.banks.begin());
}

std::ostream&
operator<<(std::ostream& os, const Configuration& conf)
{
        for (auto it = conf.banks.begin(); it != conf.banks.end(); ++it) {
                os << *it;
                if (it + 1 != conf.banks.end())
                        os << ", ";
        }

        return os;
}

void
Configuration::step()
{
        auto max = std::max_element(banks.begin(), banks.end());
        auto count = *max;

        *max = 0;

        for (auto it = ++max; count > 0; count--, ++it) {
                if (it == banks.end())
                        it = banks.begin();

                (*it)++;
        }
}

int
main()
{
        std::map<Configuration, int> history;
        auto found_pos = history.begin();
        Configuration conf = initial_configuration;
        int count = 0;

        while (true) {
                std::cout << conf << std::endl;

                found_pos = history.find(conf);
                if (found_pos != history.end())
                        break;

                history.insert(std::make_pair(conf, count));
                count++;

                conf.step();
        }

        std::cout << "Part 1: " << count << std::endl
                  << "Part 2: " << count - found_pos->second << std::endl;

        return 0;
}

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <errno.h>
#include <limits.h>

#include "pcx-util.h"

struct deck {
        size_t n_cards;
        int *cards;
        int *buf;
};

static struct deck *
create_deck(size_t n_cards)
{
        struct deck *deck = pcx_alloc(sizeof *deck);

        deck->n_cards = n_cards;
        deck->cards = pcx_alloc(n_cards * sizeof *deck->cards);
        deck->buf = pcx_alloc(n_cards * sizeof *deck->buf);

        for (unsigned i = 0; i < n_cards; i++)
                deck->cards[i] = i;

        return deck;
}

static void
free_deck(struct deck *deck)
{
        pcx_free(deck->cards);
        pcx_free(deck->buf);
        pcx_free(deck);
}

static void
reverse_deck(struct deck *deck)
{
        for (unsigned i = 0; i < deck->n_cards / 2; i++) {
                int tmp = deck->cards[i];
                deck->cards[i] = deck->cards[deck->n_cards - i - 1];
                deck->cards[deck->n_cards - i - 1] = tmp;
        }
}

static void
cut_deck(struct deck *deck,
         int amount)
{
        if (amount < 0)
                amount += deck->n_cards;

        memcpy(deck->buf, deck->cards, amount * sizeof deck->cards[0]);
        memmove(deck->cards,
                deck->cards + amount,
                (deck->n_cards - amount) * sizeof deck->cards[0]);
        memcpy(deck->cards + deck->n_cards - amount,
               deck->buf,
               amount * sizeof deck->cards[0]);
}

static void
deal_with_increment(struct deck *deck,
                    int increment)
{
        for (unsigned i = 0; i < deck->n_cards; i++)
                deck->buf[i * increment % deck->n_cards] = deck->cards[i];

        memcpy(deck->cards, deck->buf, deck->n_cards * sizeof deck->cards[0]);
}

static bool
is_string_with_integer(const char *str,
                       const char *line,
                       int *result)
{
        int str_len = strlen(str);
        int line_len = strlen(line);

        if (line_len <= str_len)
                return false;

        if (memcmp(str, line, str_len))
                return false;

        errno = 0;
        char *tail;
        long value = strtol(line + str_len, &tail, 10);
        if (errno || value < INT_MIN || value > INT_MAX)
                return false;
        while (*tail) {
                if (!isspace(*(tail++)))
                        return false;
        }

        *result = value;

        return true;
}

static bool
is_string(const char *str,
          const char *line)
{
        int str_len = strlen(str);
        int line_len = strlen(line);

        if (line_len <= str_len)
                return false;

        if (memcmp(str, line, str_len))
                return false;

        for (const char *p = line + str_len; *p; p++) {
                if (!isspace(*p))
                        return false;
        }

        return true;
}

static bool
process_commands(struct deck *deck,
                 FILE *in)
{
        char line[512];
        int line_num = 1;

        while (fgets(line, sizeof line, in)) {
                int value;

                if (is_string_with_integer("deal with increment ",
                                           line,
                                           &value)) {
                        deal_with_increment(deck, value);
                } else if (is_string_with_integer("cut ",
                                                  line,
                                                  &value)) {
                        cut_deck(deck, value);
                } else if (is_string("deal into new stack", line)) {
                        reverse_deck(deck);
                } else {
                        fprintf(stderr, "Invalid line number %i", line_num);
                        return false;
                }

                line_num++;
        }

        return true;
}

int
main(int argc, char **argv)
{
        int deck_size = 10007;

        if (argc > 1) {
                deck_size = strtol(argv[1], NULL, 10);
                if (deck_size < 1) {
                        fprintf(stderr, "Invalid deck size\n");
                        return EXIT_FAILURE;
                }
        }

        struct deck *deck = create_deck(deck_size);
        int ret = EXIT_SUCCESS;

        if (!process_commands(deck, stdin))
                ret = EXIT_FAILURE;

        for (unsigned i = 0; i < deck->n_cards; i++) {
                if (deck->cards[i] == 2019) {
                        printf("Part 1: %u\n", i);
                        goto found_card;
                }
        }

        printf("Part 1: no card found\n");
found_card:

        free_deck(deck);

        return ret;
}

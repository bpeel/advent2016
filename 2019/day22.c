#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <errno.h>
#include <limits.h>
#include <inttypes.h>

#include "pcx-util.h"
#include "pcx-buffer.h"

struct deck {
        size_t n_cards;
        int *cards;
        int *buf;
};

enum command_type {
        COMMAND_TYPE_CUT,
        COMMAND_TYPE_DEAL_WITH_INCREMENT,
        COMMAND_TYPE_REVERSE,
};

struct command {
        enum command_type command;
        int value;
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

static void
run_commands(struct deck *deck,
             size_t n_commands,
             const struct command *commands)
{
        for (unsigned i = 0; i < n_commands; i++) {
                const struct command *command = commands + i;

                switch (command->command) {
                case COMMAND_TYPE_REVERSE:
                        reverse_deck(deck);
                        break;
                case COMMAND_TYPE_CUT:
                        cut_deck(deck, command->value);
                        break;
                case COMMAND_TYPE_DEAL_WITH_INCREMENT:
                        deal_with_increment(deck, command->value);
                        break;
                }
        }
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
parse_commands(FILE *in,
               size_t *n_commands_out,
               struct command **commands_out)
{
        char line[512];
        int line_num = 1;
        struct pcx_buffer buf = PCX_BUFFER_STATIC_INIT;

        while (fgets(line, sizeof line, in)) {
                pcx_buffer_set_length(&buf,
                                      buf.length + sizeof (struct command));

                struct command *command =
                        ((struct command *) (buf.data + buf.length)) - 1;

                if (is_string_with_integer("deal with increment ",
                                           line,
                                           &command->value)) {
                        command->command = COMMAND_TYPE_DEAL_WITH_INCREMENT;
                } else if (is_string_with_integer("cut ",
                                                  line,
                                                  &command->value)) {
                        command->command = COMMAND_TYPE_CUT;
                } else if (is_string("deal into new stack", line)) {
                        command->command = COMMAND_TYPE_REVERSE;
                } else {
                        fprintf(stderr, "Invalid line number %i\n", line_num);
                        goto error;
                }

                line_num++;
        }

        *n_commands_out = buf.length / sizeof (struct command);
        *commands_out = (struct command *) buf.data;

        return true;

error:
        pcx_buffer_destroy(&buf);
        return false;
}

static int64_t
part2(size_t n_commands,
      const struct command *commands,
      int64_t pos,
      int64_t n_cards)
{
        for (unsigned i = 0; i < n_commands; i++) {
                const struct command *command = commands + i;

                switch (command->command) {
                case COMMAND_TYPE_REVERSE:
                        pos = n_cards - 1 - pos;
                        break;
                case COMMAND_TYPE_CUT: {
                        int64_t amount = command->value;

                        if (amount < 0)
                                amount += n_cards;

                        pos = (pos + amount) % n_cards;
                        break;
                }
                case COMMAND_TYPE_DEAL_WITH_INCREMENT: {
                        pos = (pos * command->value) % n_cards;
                        break;
                }
                }
        }

        return pos;
}

int
main(int argc, char **argv)
{
        int deck1_size = 10007;
        int64_t deck2_size = INT64_C(119315717514047);
        int64_t pos = 2020;
        int64_t repeats = INT64_C(101741582076661);

        if (argc > 1) {
                deck1_size = strtol(argv[1], NULL, 10);
                if (deck1_size < 1) {
                        fprintf(stderr, "Invalid deck size\n");
                        return EXIT_FAILURE;
                }
                deck2_size = deck1_size;

                if (argc > 2) {
                        pos = strtol(argv[2], NULL, 10);
                        if (pos < 1 || pos >= deck2_size) {
                                fprintf(stderr, "Invalid pos\n");
                                return EXIT_FAILURE;
                        }

                        if (argc > 3)
                                repeats = strtol(argv[3], NULL, 10);
                }
        }

        size_t n_commands;
        struct command *commands;

        if (!parse_commands(stdin, &n_commands, &commands))
                return EXIT_FAILURE;

        struct deck *deck = create_deck(deck1_size);

        run_commands(deck, n_commands, commands);

        for (unsigned i = 0; i < deck->n_cards; i++) {
                if (deck->cards[i] == 2019) {
                        printf("Part 1: %u\n", i);
                        goto found_card;
                }
        }

        printf("Part 1: no card found\n");

found_card: (void) 0;

        for (int64_t i = 0; i < repeats; i++) {
                pos = part2(n_commands,
                            commands,
                            pos,
                            deck2_size);
        }

        printf("Part 2: %" PRIi64 "\n", pos);

        free_deck(deck);
        pcx_free(commands);

        return EXIT_SUCCESS;
}

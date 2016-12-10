#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdbool.h>
#include <ctype.h>

struct stack_entry {
        size_t length;
        size_t remaining;
        int multiplier;
        struct stack_entry *next;
};

struct stack {
        struct stack_entry *top;
};

static int
get_character(void)
{
        int ch;

        do
                ch = fgetc(stdin);
        while (ch != EOF && isspace(ch));

        return ch;
}

static int
parse_number(int *result)
{
        int value = 0;
        int count = 0;
        int ch;

        while (true) {
                ch = get_character();
                if (ch == EOF)
                        break;
                if (ch < '0' || ch > '9') {
                        ungetc(ch, stdin);
                        break;
                }
                value = value * 10 + ch - '0';
                count++;
        }

        *result = value;

        return count;
}

static ssize_t
process_bracket(int *str_len,
                int *repeat_count)
{
        int count = 0;
        int ch;

        ch = get_character();
        if (ch != '(')
                return count;
        count++;

        count += parse_number(str_len);

        ch = get_character();
        if (ch != 'x')
                return count;
        count++;

        count += parse_number(repeat_count);

        ch = get_character();
        if (ch != ')')
                return count;
        count++;

        return count;
}

static void
stack_push(struct stack *stack,
           size_t bracket_length,
           size_t length,
           size_t multiplier)
{
        struct stack_entry *entry = malloc(sizeof (struct stack_entry));

        entry->length = bracket_length + length;
        entry->remaining = length;
        entry->multiplier = multiplier;

        entry->next = stack->top;
        stack->top = entry;
}

static void
stack_pop(struct stack *stack)
{
        struct stack_entry *next = stack->top->next;

        free(stack->top);
        stack->top = next;
}

int
main(int argc, char **argv)
{
        struct stack stack = { .top = NULL };
        int multiplier = 1;
        size_t count = 0;
        int str_len, repeat_count;
        int got;
        int ch;

        stack_push(&stack, 0, SIZE_MAX, 1);

        while (true) {
                ch = get_character();

                if (ch == EOF)
                        break;

                if (ch == '(') {
                        ungetc(ch, stdin);
                        got = process_bracket(&str_len, &repeat_count);
                        stack_push(&stack, got, str_len, repeat_count);
                        multiplier *= repeat_count;
                } else {
                        count += multiplier;

                        got = 1;

                        while (true) {
                                stack.top->remaining -= got;

                                if (stack.top->remaining > 0)
                                        break;

                                got = stack.top->length;
                                multiplier /= stack.top->multiplier;
                                stack_pop(&stack);
                        }
                }
        }

        stack_pop(&stack);

        printf("%li\n", count);

        return 0;
}

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>

struct firewall {
        struct range *ranges;
};

struct range {
        uint32_t min, max;
        struct range *next;
        struct range *prev;
};

static void
firewall_init(struct firewall *firewall)
{
        firewall->ranges = NULL;
}

static void
firewall_destroy(struct firewall *firewall)
{
        struct range *range, *next;

        for (range = firewall->ranges; range; range = next) {
                next = range->next;
                free(range);
        }
}

static void
firewall_block_range(struct firewall *firewall,
                     uint32_t block_min,
                     uint32_t block_max)
{
        struct range *range, *next;

        for (range = firewall->ranges; range; range = next) {
                next = range->next;

                /* Look for ranges that overlap or are next to each other */
                if (range->min > block_max + UINT64_C(1) ||
                    range->max + UINT64_C(1) < block_min)
                        continue;

                /* Expand the blocked range to include the union with
                 * this range */
                if (range->min < block_min)
                        block_min = range->min;
                if (range->max > block_max)
                        block_max = range->max;

                /* Remove the range because it will be included again
                 * later when the union is added */
                if (range->prev)
                        range->prev->next = range->next;
                else
                        firewall->ranges = range->next;

                if (range->next)
                        range->next->prev = range->prev;

                free(range);
        }

        range = malloc(sizeof (struct range));
        range->min = block_min;
        range->max = block_max;
        range->next = firewall->ranges;
        range->prev = NULL;
        if (firewall->ranges)
                firewall->ranges->prev = range;
        firewall->ranges = range;
}

static uint32_t
firewall_find_first_free_address(struct firewall *firewall)
{
        struct range *range, *smallest = firewall->ranges;

        /* None of the ranges are overlapping or touching so there is
         * always an IP address free after each range. Therefore it’s
         * just a case of finding the smallest one. */

        for (range = firewall->ranges; range; range = range->next) {
                if (range->max < smallest->max)
                        smallest = range;
        }

        if (smallest == NULL || smallest->min > 0)
                return 0;
        else
                return smallest->max + 1;
}

static uint32_t
firewall_count_free_addresses(struct firewall *firewall)
{
        uint32_t blocked_addresses = 0;
        struct range *range;

        for (range = firewall->ranges; range; range = range->next) {
                blocked_addresses += range->max - range->min;
                blocked_addresses++;
        }

        return ~blocked_addresses + 1;
}

int
main(int argc, char **argv)
{
        struct firewall firewall;
        uint32_t block_min, block_max;

        firewall_init(&firewall);

        while (scanf("%" PRIu32 "-%" PRIu32 "\n",
                     &block_min,
                     &block_max) == 2) {
                firewall_block_range(&firewall, block_min, block_max);
        }

        printf("Part 1: %" PRIu32 "\n",
               firewall_find_first_free_address(&firewall));
        printf("Part 2: %" PRIu32 "\n",
               firewall_count_free_addresses(&firewall));

        firewall_destroy(&firewall);

        return EXIT_SUCCESS;
}

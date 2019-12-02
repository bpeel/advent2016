#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "intcode.h"
#include "pcx-error.h"

int
main(int argc, char **argv)
{
        int ret = EXIT_SUCCESS;
        struct intcode *machine =
                intcode_new(5,
                            (int64_t[]) { 2, 0, 0, 0, 99 });
        struct pcx_error *error = NULL;

        if (intcode_run(machine, &error)) {
                int64_t value;
                intcode_read(machine, 0, &value, &error);
                printf("%" PRIi64 "\n", value);
        } else {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                ret = EXIT_FAILURE;
        }

        return ret;
}

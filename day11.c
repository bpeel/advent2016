#include <stdio.h>
#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <limits.h>
#include <stdint.h>
#include <string.h>

#define N_OBJECTS 10
#define N_FLOORS 4

enum state_result {
        STATE_RESULT_INSTABLE,
        STATE_RESULT_STABLE,
        STATE_RESULT_WIN,
};

/* The state is stored in a 32-bit int where two bits are used to
 * store the floor number of each object as well as the lift. The
 * even-numbered objects are the generators and the odd ones are the
 * microchips
 */
typedef uint32_t state_t;

#define STATE_OBJECT_FLOOR(state, obj_num) \
        ((((state) >> ((obj_num) * 2)) & 0x3) + 1)
#define STATE_LIFT_FLOOR(state) STATE_OBJECT_FLOOR(state, N_OBJECTS)

#define STATE_SET_OBJECT_FLOOR(state, obj_num, floor)   \
        (((state) & ~(3 << ((obj_num) * 2))) | \
         (((floor) - 1) << ((obj_num) * 2)))
#define STATE_SET_LIFT_FLOOR(state, floor) \
        STATE_SET_OBJECT_FLOOR(state, N_OBJECTS, floor)

struct move {
        int direction;
        int obj_a;
        int obj_b;
};

struct stack_entry {
        struct move move;
        state_t state;
};

struct history_entry {
        int depth;
        state_t state;
};

struct solver {
        int stack_size;
        int stack_length;
        struct stack_entry *stack_entries;

        int history_size;
        int history_length;
        struct history_entry *history_entries;
};

static const char
object_names[] = "SPTRC";

static const state_t
initial_state =
        STATE_SET_OBJECT_FLOOR(0, 0, 1) |
        STATE_SET_OBJECT_FLOOR(0, 1, 1) |
        STATE_SET_OBJECT_FLOOR(0, 2, 1) |
        STATE_SET_OBJECT_FLOOR(0, 3, 1) |
        STATE_SET_OBJECT_FLOOR(0, 4, 2) |
        STATE_SET_OBJECT_FLOOR(0, 5, 3) |
        STATE_SET_OBJECT_FLOOR(0, 6, 2) |
        STATE_SET_OBJECT_FLOOR(0, 7, 2) |
        STATE_SET_OBJECT_FLOOR(0, 8, 2) |
        STATE_SET_OBJECT_FLOOR(0, 9, 2);

static bool
floor_contains_generator(state_t state,
                         int floor)
{
        int i;

        for (i = 0; i < N_OBJECTS; i += 2) {
                if (STATE_OBJECT_FLOOR(state, i) == floor)
                        return true;
        }

        return false;
}

static enum state_result
analyse_state(state_t state)
{
        int floor, object_num;

        /* Check for a win condition (everything on the top floor) */
        for (object_num = 0; object_num < N_OBJECTS; object_num++) {
                if (STATE_OBJECT_FLOOR(state, object_num) != N_FLOORS)
                        goto not_win;
        }

        return STATE_RESULT_WIN;

not_win:

        /* Check if any of the floors are instable */
        for (floor = 0; floor < N_FLOORS; floor++) {
                /* The floor is always stable if it doesn’t contain a
                 * generator */
                if (!floor_contains_generator(state, floor))
                        continue;

                /* Check if all of the chips have their corresponding
                 * generator */
                for (object_num = 1; object_num < N_OBJECTS; object_num += 2) {
                        if (STATE_OBJECT_FLOOR(state, object_num) != floor)
                                continue;

                        if (STATE_OBJECT_FLOOR(state, object_num - 1) != floor)
                                return STATE_RESULT_INSTABLE;
                }
        }

        return STATE_RESULT_STABLE;
}

static bool
move_valid(state_t state,
           const struct move *move)
{
        /* Can’t move the lift outside the building */
        if (move->direction) {
                if (STATE_LIFT_FLOOR(state) >= N_FLOORS)
                        return false;
        } else {
                if (STATE_LIFT_FLOOR(state) <= 1)
                        return false;
        }

        /* Can’t take the same object twice (or no objects) */
        if (move->obj_a == move->obj_b)
                return false;

        /* Objects taken must be on the same floor as the lift */
        if ((move->obj_a >= 0 &&
             STATE_OBJECT_FLOOR(state, move->obj_a) !=
             STATE_LIFT_FLOOR(state)) ||
            (move->obj_b >= 0 &&
             STATE_OBJECT_FLOOR(state, move->obj_b) !=
             STATE_LIFT_FLOOR(state)))
                return false;

        return true;
}

static bool
next_move(struct move *move)
{
        if (++move->obj_a < N_OBJECTS)
                return true;
        move->obj_a = -1;

        if (++move->obj_b < N_OBJECTS)
                return true;
        move->obj_b = -1;

        if (++move->direction < 2)
                return true;

        return false;
}

static state_t
apply_move(state_t state,
           const struct move *move)
{
        int floor_offset = move->direction ? 1 : -1;
        int floor;

        if (move->obj_a > -1) {
                floor = STATE_OBJECT_FLOOR(state, move->obj_a) + floor_offset;
                state = STATE_SET_OBJECT_FLOOR(state, move->obj_a, floor);
        }
        if (move->obj_b > -1) {
                floor = STATE_OBJECT_FLOOR(state, move->obj_b) + floor_offset;
                state = STATE_SET_OBJECT_FLOOR(state, move->obj_b, floor);
        }

        floor = STATE_LIFT_FLOOR(state) + floor_offset;
        state = STATE_SET_LIFT_FLOOR(state, floor);

        return state;
}

static bool
add_state_to_history(struct solver *solver,
                     state_t state)
{
        struct history_entry *entry;
        int min = 0, max = solver->history_length, mid;

        while (min < max) {
                mid = (min + max) / 2;
                entry = solver->history_entries + mid;

                if (entry->state == state) {
                        if (entry->depth > solver->stack_length) {
                                entry->depth = solver->stack_length;
                                return true;
                        } else {
                                return false;
                        }
                } else if (entry->state < state) {
                        min = mid + 1;
                } else {
                        max = mid;
                }
        }

        if (solver->history_length >= solver->history_size) {
                solver->history_size *= 2;
                solver->history_entries =
                        realloc(solver->history_entries,
                                sizeof (struct history_entry) *
                                solver->history_size);
        }

        memmove(solver->history_entries + min + 1,
                solver->history_entries + min,
                (solver->history_length - min) * sizeof (struct history_entry));

        entry = solver->history_entries + min;
        entry->state = state;
        entry->depth = solver->stack_length;
        solver->history_length++;

        return true;
}

static bool
find_next_move(struct solver *solver,
               state_t state,
               struct move *move)
{
        state_t new_state;

        while (true) {
                if (!next_move(move))
                        return false;

                /* Skip moves that don’t make sense */
                if (!move_valid(state, move))
                        continue;

                new_state = apply_move(state, move);

                /* Skip invalid states */
                if (analyse_state(new_state) == STATE_RESULT_INSTABLE)
                        continue;

                /* Skip states that we’ve already visited earlier in
                 * the stack */
                if (!add_state_to_history(solver, new_state))
                        continue;

                return true;
        }
}

static void
stack_push(struct solver *solver,
           state_t state)
{
        struct stack_entry *entry;

        if (solver->stack_length >= solver->stack_size) {
                solver->stack_size *= 2;
                solver->stack_entries = realloc(solver->stack_entries,
                                                sizeof *solver->stack_entries *
                                                solver->stack_size);
        }

        entry = solver->stack_entries + solver->stack_length++;
        entry->state = state;
        entry->move.direction = 0;
        entry->move.obj_a = -1;
        entry->move.obj_b = -1;
}

static void
stack_pop(struct solver *solver)
{
        assert(solver->stack_length >= 1);
        solver->stack_length--;
}

static struct stack_entry *
stack_top(struct solver *solver)
{
        assert(solver->stack_length >= 1);

        return solver->stack_entries + solver->stack_length - 1;
}

static void
solver_init(struct solver *solver)
{
        solver->stack_length = 0;
        solver->stack_size = 8;
        solver->stack_entries = malloc(sizeof (struct stack_entry) *
                                       solver->stack_size);

        solver->history_length = 0;
        solver->history_size = 8;
        solver->history_entries = malloc(sizeof (struct history_entry) *
                                         solver->history_size);
}

static void
solver_destroy(struct solver *solver)
{
        free(solver->stack_entries);
        free(solver->history_entries);
}

static void
print_object(int obj_num)
{
        printf("%c%c",
               object_names[obj_num / 2],
               (obj_num & 1) ? 'M' : 'G');
}

static void
print_state(state_t state)
{
        int floor, object_num;

        for (floor = N_FLOORS; floor >= 1; floor--) {
                printf("F%i %c",
                       floor,
                       STATE_LIFT_FLOOR(state) == floor ? 'E' : '.');

                for (object_num = 0; object_num < N_OBJECTS; object_num++) {
                        if (STATE_OBJECT_FLOOR(state, object_num) != floor)
                                continue;

                        fputc(' ', stdout);
                        print_object(object_num);
                }

                fputc('\n', stdout);
        }

        fputc('\n', stdout);
}

static void
print_move(const struct move *move)
{
        printf("%c", move->direction ? 'U' : 'D');

        if (move->obj_a != -1) {
                fputc(' ', stdout);
                print_object(move->obj_a);
        }

        if (move->obj_b != -1) {
                fputc(' ', stdout);
                print_object(move->obj_b);
        }

        fputc('\n', stdout);
}

static void
print_solution(const struct solver *solver)
{
        int i;

        for (i = 0; i < solver->stack_length; i++) {
                print_state(solver->stack_entries[i].state);

                if (i < solver->stack_length - 1)
                        print_move(&solver->stack_entries[i].move);
        }

        printf("Total moves: %i\n\n", solver->stack_length - 1);
}

static void
solve(void)
{
        struct solver solver;
        struct stack_entry *top;
        int best_solution = INT_MAX;
        state_t next_state;

        solver_init(&solver);
        stack_push(&solver, initial_state);

        while (solver.stack_length > 0) {
                top = stack_top(&solver);

                if (solver.stack_length > best_solution) {
                        stack_pop(&solver);
                } else if (analyse_state(top->state) == STATE_RESULT_WIN) {
                        print_solution(&solver);
                        best_solution = solver.stack_length;
                        stack_pop(&solver);
                } else if (find_next_move(&solver, top->state, &top->move)) {
                        next_state = apply_move(top->state, &top->move);
                        stack_push(&solver, next_state);
                } else {
                        stack_pop(&solver);
                }
        }

        solver_destroy(&solver);
}

int
main(int argc, char **argv)
{
        solve();

        return 0;
}

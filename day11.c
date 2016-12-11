#include <stdio.h>
#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <limits.h>

#define N_OBJECTS 10
#define N_FLOORS 4

enum object_type {
        GENERATOR,
        MICROCHIP
};

enum state_result {
        STATE_RESULT_INSTABLE,
        STATE_RESULT_STABLE,
        STATE_RESULT_WIN,
};

struct object {
        enum object_type type;
        char material;
        int floor;
};

struct state {
        int lift_floor;
        struct object objects[N_OBJECTS];
};

struct move {
        int direction;
        int obj_a;
        int obj_b;
};

struct stack_entry {
        struct move move;
        struct state state;
};

struct history_entry {
        int depth;
        struct state state;
};

struct solver {
        int stack_size;
        int stack_length;
        struct stack_entry *stack_entries;

        int history_size;
        int history_length;
        struct history_entry *history_entries;
};

const struct state
initial_state = {
        .lift_floor = 1,
        .objects = {
                { .type = GENERATOR, .material = 'S', .floor = 1 },
                { .type = MICROCHIP, .material = 'S', .floor = 1 },
                { .type = GENERATOR, .material = 'P', .floor = 1 },
                { .type = MICROCHIP, .material = 'P', .floor = 1 },

                { .type = GENERATOR, .material = 'T', .floor = 2 },
                { .type = GENERATOR, .material = 'R', .floor = 2 },
                { .type = MICROCHIP, .material = 'R', .floor = 2 },
                { .type = GENERATOR, .material = 'C', .floor = 2 },
                { .type = MICROCHIP, .material = 'C', .floor = 2 },

                { .type = MICROCHIP, .material = 'T', .floor = 3 },
        }
};

static bool
floor_contains_generator(const struct state *state,
                         int floor)
{
        int i;

        for (i = 0; i < N_OBJECTS; i++) {
                if (state->objects[i].floor == floor &&
                    state->objects[i].type == GENERATOR)
                        return true;
        }

        return false;
}

static bool
floor_has_generator_for_material(const struct state *state,
                                 int floor,
                                 int material)
{
        int i;

        for (i = 0; i < N_OBJECTS; i++) {
                if (state->objects[i].floor == floor &&
                    state->objects[i].type == GENERATOR &&
                    state->objects[i].material == material)
                        return true;
        }

        return false;
}

static enum state_result
analyse_state(const struct state *state)
{
        int floor, object_num;
        const struct object *object;

        /* Check for a win condition (everything on the top floor) */
        for (object_num = 0; object_num < N_OBJECTS; object_num++) {
                if (state->objects[object_num].floor != N_FLOORS)
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
                for (object_num = 0; object_num < N_OBJECTS; object_num++) {
                        object = state->objects + object_num;

                        if (object->floor != floor ||
                            object->type != MICROCHIP)
                                continue;

                        if (!floor_has_generator_for_material(state,
                                                              floor,
                                                              object->material))
                                return STATE_RESULT_INSTABLE;
                }
        }

        return STATE_RESULT_STABLE;
}

static bool
state_equal(const struct state *a, const struct state *b)
{
        const struct object *obj_a, *obj_b;
        int i;

        for (i = 0; i < N_OBJECTS; i++) {
                obj_a = a->objects + i;
                obj_b = b->objects + i;

                if (obj_a->type != obj_b->type ||
                    obj_a->material != obj_b->material ||
                    obj_a->floor != obj_b->floor)
                        return false;
        }

        if (a->lift_floor != b->lift_floor)
                return false;

        return true;
}

static bool
move_valid(const struct state *state,
           const struct move *move)
{
        /* Can’t move the lift outside the building */
        if (move->direction) {
                if (state->lift_floor >= N_FLOORS)
                        return false;
        } else {
                if (state->lift_floor <= 1)
                        return false;
        }

        /* Can’t take the same object twice (or no objects) */
        if (move->obj_a == move->obj_b)
                return false;

        /* Objects taken must be on the same floor as the lift */
        if ((move->obj_a >= 0 &&
             state->objects[move->obj_a].floor != state->lift_floor) ||
            (move->obj_b >= 0 &&
             state->objects[move->obj_b].floor != state->lift_floor))
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

static void
apply_move(struct state *state,
           const struct move *move)
{
        int floor_offset = move->direction ? 1 : -1;

        if (move->obj_a > -1)
                state->objects[move->obj_a].floor += floor_offset;
        if (move->obj_b > -1)
                state->objects[move->obj_b].floor += floor_offset;

        state->lift_floor += floor_offset;
}

static bool
add_state_to_history(struct solver *solver,
                     const struct state *state)
{
        struct history_entry *entry;
        int i;

        for (i = 0; i < solver->history_length; i++) {
                entry = solver->history_entries + i;

                if (state_equal(&entry->state, state)) {
                        if (entry->depth > solver->stack_length) {
                                entry->depth = solver->stack_length;
                                return true;
                        } else {
                                return false;
                        }
                }
        }

        if (solver->history_length >= solver->history_size) {
                solver->history_size *= 2;
                solver->history_entries =
                        realloc(solver->history_entries,
                                sizeof (struct history_entry) *
                                solver->history_size);
        }

        entry = solver->history_entries + solver->history_length++;
        entry->state = *state;
        entry->depth = solver->stack_length;

        return true;
}

static bool
find_next_move(struct solver *solver,
               const struct state *state,
               struct move *move)
{
        struct state new_state;

        while (true) {
                if (!next_move(move))
                        return false;

                /* Skip moves that don’t make sense */
                if (!move_valid(state, move))
                        continue;

                new_state = *state;
                apply_move(&new_state, move);

                /* Skip invalid states */
                if (analyse_state(&new_state) == STATE_RESULT_INSTABLE)
                        continue;

                /* Skip states that we’ve already visited earlier in
                 * the stack */
                if (!add_state_to_history(solver, &new_state))
                        continue;

                return true;
        }
}

static void
stack_push(struct solver *solver,
           const struct state *state)
{
        struct stack_entry *entry;

        if (solver->stack_length >= solver->stack_size) {
                solver->stack_size *= 2;
                solver->stack_entries = realloc(solver->stack_entries,
                                                sizeof *solver->stack_entries *
                                                solver->stack_size);
        }

        entry = solver->stack_entries + solver->stack_length++;
        entry->state = *state;
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
print_object(const struct object *object)
{
        printf("%c%c",
               object->material,
               object->type == GENERATOR ? 'G' :
               object->type == MICROCHIP ? 'M' :
               '?');
}

static void
print_state(const struct state *state)
{
        int floor, object_num;
        const struct object *object;

        for (floor = N_FLOORS; floor >= 1; floor--) {
                printf("F%i %c",
                       floor,
                       state->lift_floor == floor ? 'E' : '.');

                for (object_num = 0; object_num < N_OBJECTS; object_num++) {
                        object = state->objects + object_num;

                        if (object->floor != floor)
                                continue;

                        fputc(' ', stdout);
                        print_object(object);
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
                print_object(initial_state.objects + move->obj_a);
        }

        if (move->obj_b != -1) {
                fputc(' ', stdout);
                print_object(initial_state.objects + move->obj_b);
        }

        fputc('\n', stdout);
}

static void
print_solution(const struct solver *solver)
{
        int i;

        for (i = 0; i < solver->stack_length; i++) {
                print_state(&solver->stack_entries[i].state);

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
        struct state next_state;

        solver_init(&solver);
        stack_push(&solver, &initial_state);

        while (solver.stack_length > 0) {
                top = stack_top(&solver);

                if (analyse_state(&top->state) == STATE_RESULT_WIN &&
                    solver.stack_length - 1 < best_solution) {
                        print_solution(&solver);
                        best_solution = solver.stack_length - 1;
                        stack_pop(&solver);
                } else if (find_next_move(&solver, &top->state, &top->move)) {
                        next_state = top->state;
                        apply_move(&next_state, &top->move);
                        stack_push(&solver, &next_state);
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

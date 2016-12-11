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

struct stack {
        int size;
        int length;
        struct stack_entry *entries;
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
state_in_stack(const struct stack *stack,
               const struct state *state)
{
        int i;

        for (i = 0; i < stack->length; i++) {
                if (state_equal(&stack->entries[i].state, state))
                        return true;
        }

        return false;
}

static bool
find_next_move(const struct stack *stack,
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
                if (state_in_stack(stack, &new_state))
                        continue;

                return true;
        }
}

static void
stack_push(struct stack *stack,
           const struct state *state)
{
        struct stack_entry *entry;

        if (stack->length >= stack->size) {
                stack->size *= 2;
                stack->entries = realloc(stack->entries,
                                         sizeof *stack->entries * stack->size);
        }

        entry = stack->entries + stack->length++;
        entry->state = *state;
        entry->move.direction = 0;
        entry->move.obj_a = -1;
        entry->move.obj_b = -1;
}

static void
stack_pop(struct stack *stack)
{
        assert(stack->length >= 1);
        stack->length--;
}

static struct stack_entry *
stack_top(struct stack *stack)
{
        assert(stack->length >= 1);

        return stack->entries + stack->length - 1;
}

static void
stack_init(struct stack *stack)
{
        stack->length = 0;
        stack->size = 8;
        stack->entries = malloc(sizeof (struct stack_entry) * stack->size);
}

static void
stack_destroy(struct stack *stack)
{
        free(stack->entries);
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
print_solution(const struct stack *stack)
{
        int i;

        for (i = 0; i < stack->length; i++) {
                print_state(&stack->entries[i].state);

                if (i < stack->length - 1)
                        print_move(&stack->entries[i].move);
        }

        printf("Total moves: %i\n\n", stack->length - 1);
}

static void
solve(void)
{
        struct stack stack;
        struct stack_entry *top;
        int best_solution = INT_MAX;
        struct state next_state;

        stack_init(&stack);
        stack_push(&stack, &initial_state);

        while (stack.length > 0) {
                top = stack_top(&stack);

                if (analyse_state(&top->state) == STATE_RESULT_WIN &&
                    stack.length - 1 < best_solution) {
                        print_solution(&stack);
                        best_solution = stack.length - 1;
                }

                if (find_next_move(&stack, &top->state, &top->move)) {
                        next_state = top->state;
                        apply_move(&next_state, &top->move);
                        stack_push(&stack, &next_state);
                } else {
                        stack_pop(&stack);
                }
        }

        stack_destroy(&stack);
}

int
main(int argc, char **argv)
{
        solve();

        return 0;
}

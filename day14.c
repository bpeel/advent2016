#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>
#include <openssl/md5.h>

#define QUEUE_SIZE 1000

typedef void
(* get_hash_func_t)(const char *salt,
                    int index,
                    char *result);

static void
hex_digest(MD5_CTX *md5_ctx,
           char *result)
{
        uint8_t hash[MD5_DIGEST_LENGTH];
        char byte[3];
        int i;

        MD5_Final(hash, md5_ctx);

        for (i = 0; i < MD5_DIGEST_LENGTH; i++) {
                sprintf(byte, "%02x", hash[i]);
                memcpy(result, byte, 2);
                result += 2;
        }
}

static void
get_hash_part1(const char *salt,
               int index,
               char *result)
{
        MD5_CTX md5_ctx;
        int num_len;
        char numbuf[32];

        num_len = sprintf(numbuf, "%i", index);

        MD5_Init(&md5_ctx);
        MD5_Update(&md5_ctx, salt, strlen(salt));
        MD5_Update(&md5_ctx, numbuf, num_len);
        hex_digest(&md5_ctx, result);
}

static void
get_hash_part2(const char *salt,
               int index,
               char *result)
{
        MD5_CTX md5_ctx;
        int i;

        get_hash_part1(salt, index, result);

        for (i = 0; i < 2016; i++) {
                MD5_Init(&md5_ctx);
                MD5_Update(&md5_ctx, result, MD5_DIGEST_LENGTH * 2);
                hex_digest(&md5_ctx, result);
        }
}

static void
initialise_hash_queue(const char *salt,
                      get_hash_func_t get_hash,
                      char *hash_queue)
{
        int i;

        for (i = 0; i < QUEUE_SIZE; i++)
                get_hash(salt, i, hash_queue + i * MD5_DIGEST_LENGTH * 2);
}

static int
find_triplet(const char *hash)
{
        int i;

        for (i = 0; i <= MD5_DIGEST_LENGTH * 2 - 3; i++) {
                if (hash[1] == hash[0] && hash[2] == hash[0])
                        return hash[0];
                hash++;
        }

        return -1;
}

static int
hash_has_quintuplet(const char *hash, char ch)
{
        int i, j;

        for (i = 0; i <= MD5_DIGEST_LENGTH * 2 - 5; i++) {
                for (j = 0; j < 5; j++) {
                        if (hash[i + j] != ch)
                                goto not_quintuplet;
                }

                return true;

        not_quintuplet:
                i += j;
        }

        return false;
}

static bool
find_quintuplet(const char *hash_queue, char ch)
{
        int i;

        for (i = 0; i < QUEUE_SIZE; i++) {
                if (hash_has_quintuplet(hash_queue, ch))
                        return true;
                hash_queue += MD5_DIGEST_LENGTH * 2;
        }

        return false;
}

static int
solve(const char *salt,
      get_hash_func_t get_hash)
{
        char *hash_queue = malloc(QUEUE_SIZE * MD5_DIGEST_LENGTH * 2 + 1);
        char *this_hash;
        int index = 0, found = 0;
        int triplet_character;

        initialise_hash_queue(salt, get_hash, hash_queue);

        while (true) {
                this_hash = (hash_queue +
                             (index % QUEUE_SIZE) * MD5_DIGEST_LENGTH * 2);
                triplet_character = find_triplet(this_hash);

                /* Replace this hash with a new one so that hash_queue
                 * is like a rolling queue of hashes */
                get_hash(salt, index + QUEUE_SIZE, this_hash);

                if (triplet_character != -1 &&
                    find_quintuplet(hash_queue, triplet_character) &&
                    ++found >= 64)
                        break;

                index++;
        }

        free(hash_queue);

        return index;
}

int
main(int argc, char **argv)
{
        const char *salt = "abc";

        if (argc >= 2)
                salt = argv[1];

        printf("Part 1: %i\n", solve(salt, get_hash_part1));
        printf("Part 2: %i\n", solve(salt, get_hash_part2));

        return 0;
}

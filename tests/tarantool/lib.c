#include <tarantool/module.h>
#include <msgpuck.h>

// returns sum of two numbers
int sum(box_function_ctx_t *ctx, const char *args_base, const char *args_end_base) {
    uint32_t arg_n = mp_decode_array(&args_base);
    assert(arg_n == 2);

    uint32_t a = mp_decode_uint(&args_base);
    uint32_t b = mp_decode_uint(&args_base);
    uint32_t result = a + b;

    char return_buf[16] = {0};
    char *return_buf_end = return_buf;
    return_buf_end = mp_encode_uint(return_buf_end, result);

    return box_return_mp(ctx, return_buf, return_buf_end);
}

// returns request as is
int echo(box_function_ctx_t *ctx, const char *args_base, const char *args_end_base) {
    return box_return_mp(ctx, args_base, args_end_base);
}

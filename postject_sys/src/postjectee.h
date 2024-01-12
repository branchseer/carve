// https://github.com/nodejs/postject/blob/3c4f2080ee56025716c3add0f6c03b16e2af54ff/test/test.c#L1
// _GNU_SOURCE is needed to enable dl_iterate_phdr and dl_phdr_info
#if defined(__linux__)
#define _GNU_SOURCE
#endif

#include "../cmake/postject/postject-api.h"

#ifdef __cplusplus
extern "C" {
#endif

const void *postjectee_find_resource(const char *name, size_t *size,
                                     const struct postject_options *options);
#ifdef __cplusplus
}
#endif

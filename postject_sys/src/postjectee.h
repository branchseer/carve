#include "../cmake/postject/postject-api.h"

#ifdef __cplusplus
extern "C" {
#endif

const void *postjectee_find_resource(const char *name, size_t *size,
                                     const struct postject_options *options);
#ifdef __cplusplus
}
#endif

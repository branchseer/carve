#include "./postjectee.h"

const void *postjectee_find_resource(const char *name, size_t *size,
                                     const struct postject_options *options) {
  return postject_find_resource(name, size, options);
}

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  const char* elf_section_name;
  const char* macho_section_name;
  const char* macho_segment_name;
  const char* pe_resource_name;
} postjectee_options;

const void *postjectee_find_resource(const char *name, size_t *size,
                                     postjectee_options options);
#ifdef __cplusplus
}
#endif

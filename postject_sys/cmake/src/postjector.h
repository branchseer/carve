#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef int PostjectorInjectResultType;

extern PostjectorInjectResultType POSTJECTOR_INJECT_ALREADY_EXISTS;
extern PostjectorInjectResultType POSTJECTOR_INJECT_ERROR;
extern PostjectorInjectResultType POSTJECTOR_INJECT_SUCCESS;
extern PostjectorInjectResultType POSTJECTOR_INJECT_UNKNOWN_EXECUTABLE_FORMAT;

typedef struct {
  uint8_t* head;
  size_t size;
} PostjectorBuffer;

typedef struct {
  PostjectorInjectResultType type;
  PostjectorBuffer data;
  char is_macho;
} PostjectorInjectResult;

PostjectorInjectResult postjector_inject(
    PostjectorBuffer executable,
    PostjectorBuffer resource,
    const char* elf_note_name,
    const char* macho_segment_name,
    const char* macho_section_name,
    const char* pe_resource_name,
    char override
);
void postjector_buffer_free(PostjectorBuffer buffer);

#ifdef __cplusplus
}
#endif
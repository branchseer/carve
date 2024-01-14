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

typedef struct OwnedPostjectorBuffer_s* PostjectorOwnedBuffer;

typedef struct {
  PostjectorInjectResultType type;
  PostjectorOwnedBuffer data;
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

PostjectorBuffer postjector_owned_buffer_data(const PostjectorOwnedBuffer buf);
void postjector_owned_buffer_free(PostjectorOwnedBuffer buf);

#ifdef __cplusplus
}
#endif
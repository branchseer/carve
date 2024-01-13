#include "./postjector.h"
#include "../postject/src/postject.hpp"
#include <string>
#include <LIEF/logging.hpp>
#include <mutex>

static inline std::vector<uint8_t> buf2vector(PostjectorBuffer buffer) {
  return {buffer.head, buffer.head + buffer.size};
}

extern "C" {
  PostjectorInjectResultType POSTJECTOR_INJECT_ALREADY_EXISTS =
      static_cast<PostjectorInjectResultType>(
          postject::InjectResultType::kAlreadyExists);
  PostjectorInjectResultType POSTJECTOR_INJECT_ERROR =
      static_cast<PostjectorInjectResultType>(postject::InjectResultType::kError);
  PostjectorInjectResultType POSTJECTOR_INJECT_SUCCESS =
      static_cast<PostjectorInjectResultType>(
          postject::InjectResultType::kSuccess);
  PostjectorInjectResultType POSTJECTOR_INJECT_UNKNOWN_EXECUTABLE_FORMAT = -1;

  PostjectorInjectResult postjector_inject(
      PostjectorBuffer executable, PostjectorBuffer resource,
      const char* elf_note_name, const char* macho_segment_name,
      const char* macho_section_name, const char* pe_resource_name,
      char override) {
    static std::once_flag config_lier_log_once;
    std::call_once(config_lier_log_once, [] {
        // Suppress warning on success injections: https://github.com/nodejs/postject/issues/83
        LIEF::logging::set_level(LIEF::logging::LOG_ERR);
    });
    std::vector<uint8_t> executable_buffer = buf2vector(executable);
    std::vector<uint8_t> resource_buffer = buf2vector(resource);
    bool override_bool = override;
    postject::InjectResult result;
    bool is_macho = false;
    switch (postject::get_executable_format(executable_buffer)) {
    case postject::ExecutableFormat::kELF:
      result =
          postject::inject_into_elf(executable_buffer, elf_note_name,
                                    resource_buffer, override_bool);
      break;
    case postject::ExecutableFormat::kMachO:
      is_macho = true;
      result = postject::inject_into_macho(
          executable_buffer, macho_segment_name,
          macho_section_name, resource_buffer, override_bool);
      break;
    case postject::ExecutableFormat::kPE:
      result = postject::inject_into_pe(executable_buffer,
                                        pe_resource_name,
                                        resource_buffer, override_bool);
      break;
    case postject::ExecutableFormat::kUnknown:
      return {POSTJECTOR_INJECT_UNKNOWN_EXECUTABLE_FORMAT};
    }
    PostjectorBuffer data;
    if (result.type == postject::InjectResultType::kSuccess) {
      data.size = result.output.size();
      try {
        data.head = new uint8_t[data.size];
      } catch (const std::bad_alloc &) {
        return {POSTJECTOR_INJECT_ERROR};
      }
      std::copy(result.output.begin(), result.output.end(), data.head);
    }

    return {static_cast<PostjectorInjectResultType>(result.type), data, is_macho };
  }

  void postjector_buffer_free(PostjectorBuffer buffer) { delete[] buffer.head; }

}
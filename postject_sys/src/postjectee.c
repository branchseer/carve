/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
// https://github.com/nodejs/postject/blob/3c4f2080ee56025716c3add0f6c03b16e2af54ff/test/test.c#L1
// _GNU_SOURCE is needed to enable dl_iterate_phdr and dl_phdr_info
#if defined(__linux__)
#define _GNU_SOURCE
#endif

#include "../cmake/postject/postject-api.h"

#include "./postjectee.h"

const void *postjectee_find_resource(const char *name, size_t *size,
                                     postjectee_options options) {
  struct postject_options postject_options;
  postject_options.elf_section_name = options.elf_section_name;
  postject_options.macho_framework_name = NULL;
  postject_options.macho_section_name = options.macho_section_name;
  postject_options.macho_segment_name = options.macho_segment_name;
  postject_options.pe_resource_name = options.pe_resource_name;
  return postject_find_resource(name, size, &postject_options);
}

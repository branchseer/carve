cmake_minimum_required(VERSION 3.14)

include(FetchContent)

if(MSVC)
    add_compile_options(/EHsc)
endif()

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_C_STANDARD 99)

project(postjector)

FetchContent_Declare(
  postject
  GIT_REPOSITORY https://github.com/branchseer/postject
  GIT_TAG f4d9bd14194e93f4bcacf88f160aa8b3e05b6aa6
)

FetchContent_MakeAvailable(postject)

add_library(postjector
    src/postjector.h
    src/postjector.cpp
)
target_link_libraries(postjector postject)

set_target_properties(postject_wasm PROPERTIES EXCLUDE_FROM_ALL TRUE)
install(TARGETS postjector postject)


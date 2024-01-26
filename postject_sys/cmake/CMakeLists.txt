cmake_minimum_required(VERSION 3.9)

if(MSVC)
    add_compile_options(/EHsc)
endif()

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_C_STANDARD 99)

project(postjector)

add_subdirectory(postject)

add_library(postjector
    src/postjector.h
    src/postjector.cpp
)
target_link_libraries(postjector postject)

set_target_properties(postject_wasm PROPERTIES EXCLUDE_FROM_ALL TRUE)
install(TARGETS postjector postject)

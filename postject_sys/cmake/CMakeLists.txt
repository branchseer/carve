cmake_minimum_required(VERSION 3.9)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_C_STANDARD 99)

project(postjector)

add_subdirectory(postject)

add_library(postjector
    src/postjector.h
    src/postjector.cpp
)
target_link_libraries(postjector postject)

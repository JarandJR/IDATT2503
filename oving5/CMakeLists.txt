cmake_minimum_required(VERSION 3.26)
project(c C)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c11 -Wall -Wextra")

add_library(format_string STATIC format_string.c)
target_include_directories(format_string PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_executable(c main.c)
target_link_libraries(c PRIVATE format_string)

enable_testing()
add_subdirectory(tests)

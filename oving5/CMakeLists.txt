cmake_minimum_required(VERSION 3.26)
project(oving5 C)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c11 -Wall -Wextra")

add_library(format_string INTERFACE)
target_include_directories(format_string INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

add_executable(main main.c)
target_link_libraries(main format_string)

enable_testing()
add_subdirectory(tests)

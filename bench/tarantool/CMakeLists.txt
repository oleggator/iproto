cmake_minimum_required(VERSION 3.14)
project(csum C)

include_directories(/usr/local/include)

set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -undefined suppress -flat_namespace")
set(CMAKE_MACOSX_RPATH 1)
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_FLAGS_RELEASE "-O3")
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_C_FLAGS_DEBUG "-g -Wall -Wextra -Wpedantic")

add_library(csum SHARED lib.c)
target_link_libraries(csum /usr/local/lib/libmsgpuck.a)

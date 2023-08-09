cmake_minimum_required(VERSION 3.14)
project(csum C)

if (APPLE)
    include_directories(/usr/local/include)
    add_link_options("-undefined" "suppress" "-flat_namespace")
endif()

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_FLAGS_RELEASE "-O3")
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_C_FLAGS_DEBUG "-g -Wall -Wextra -Wpedantic")

add_library(csum SHARED lib.c)
target_link_libraries(csum /usr/local/lib/libmsgpuck.a)

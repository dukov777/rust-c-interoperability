cmake_minimum_required(VERSION 3.10)

# Set project name and language
project(MyStaticLibrary C)

# Add source files for the library
add_library(static_c_lib STATIC
    ${PROJECT_SOURCE_DIR}/c_function.c
)
# Specify include directories
target_include_directories(static_c_lib PUBLIC
    ${PROJECT_SOURCE_DIR}/include
)
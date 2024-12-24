cmake_minimum_required(VERSION 3.10)
project(MyProject)

# Set paths to Rust code and build output
set(RUST_LIB_DIR ${CMAKE_SOURCE_DIR}/rust_lib)
set(RUST_LIB_OUTPUT_DIR ${RUST_LIB_DIR}/target/debug)

# Add C source files
add_executable(main ${CMAKE_SOURCE_DIR}/C/main.c)

# Add a custom command to build the Rust static library
add_custom_command(
    OUTPUT ${RUST_LIB_OUTPUT_DIR}/librust_lib.a # This is the expected output file
    COMMAND cargo build # Build the Rust project
    WORKING_DIRECTORY ${RUST_LIB_DIR} # Directory where the Cargo.toml is located
    COMMENT "Building the Rust static library"
    VERBOSE 1 # Add verbosity to see detailed output from this command
)

# Ensure the custom command is always run when building the main target
add_custom_target(
    build_rust_lib ALL
    DEPENDS ${RUST_LIB_OUTPUT_DIR}/librust_lib.a # This makes the C build depend on the Rust library
)

add_dependencies(main build_rust_lib)

# Link the Rust static library to the C executable
target_link_libraries(main ${RUST_LIB_OUTPUT_DIR}/librust_lib.a)

# Include directories for any headers if required by Rust (optional)
include_directories(${RUST_LIB_DIR}/include)

# Custom clean target for both Rust and C build directories

# Custom clean target with a different name (avoid using 'clean')
# Custom clean target with the correct working directory for Rust
add_custom_target(
    clean_all
    COMMAND cargo clean
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/rust_lib # Specify the path to the Rust project directory
    COMMAND rm -rf ${CMAKE_BINARY_DIR} # Clean CMake build artifacts (C build)
    COMMAND rm -rf ${RUST_LIB_DIR}/target/ # Force cleaning the Rust target directory
    COMMENT "Cleaning Rust and C build directories"
)


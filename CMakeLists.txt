cmake_minimum_required(VERSION 3.26.3)

if(DEFINED CMAKE_TOOLCHAIN_FILE)
    include(${CMAKE_TOOLCHAIN_FILE})
endif()

project(kernel1
    DESCRIPTION "Test kernel"
    VERSION 0.0.1
    LANGUAGES C CXX ASM)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if (WIN32 OR UNIX)
    set(EMULATOR ON)
else()
    set(EMULATOR OFF)
endif()

set(NO_STD_LINKER_FLAG
    #-nostartfiles
    #-nodefaultlibs #OK
    #-nolibc #OK
    -nostdlib
    )

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

include(ExternalProject)
ExternalProject_Add(
    DumbAlloc
    GIT_REPOSITORY https://github.com/burbokop/dumb_alloc.git
    CMAKE_ARGS
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DENABLE_TESTS=OFF
        -DENABLE_EXAMPLES=OFF
        -DLINK_FLAGS=${NO_STD_LINKER_FLAG}
        -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/deps-install)

include_directories(${CMAKE_BINARY_DIR}/deps-install/include)
link_directories(${CMAKE_BINARY_DIR}/deps-install/lib)

add_library(fake_libc STATIC
    ${CMAKE_CURRENT_LIST_DIR}/src/fake_libc/fake_stdlib.c
    ${CMAKE_CURRENT_LIST_DIR}/src/fake_libc/fake_stdlib.h)

target_link_options(fake_libc PUBLIC ${NO_STD_LINKER_FLAG} -z noexecstack)
add_dependencies(fake_libc DumbAlloc)
target_link_libraries(fake_libc PUBLIC gcc dumb_alloc)

if (NOT (EMULATOR))
    add_library(fake_libc_stdio STATIC
        ${CMAKE_CURRENT_LIST_DIR}/src/fake_libc/fake_stdio.c
        ${CMAKE_CURRENT_LIST_DIR}/src/fake_libc/fake_stdio.h)

    target_link_options(fake_libc_stdio PUBLIC ${NO_STD_LINKER_FLAG} -z noexecstack)
    target_link_libraries(fake_libc_stdio PUBLIC gcc fake_libc)
endif()

if (EMULATOR)
    set(CORROSION_EMULATOR_FLAGS --features emulator)
    set(ARCH_DUPLE emulator)
else()
    set(CORROSION_EMULATOR_FLAGS)
endif()

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH ${CMAKE_CURRENT_LIST_DIR}/Cargo.toml
    FLAGS -Zbuild-std=core,alloc ${CORROSION_EMULATOR_FLAGS})

# Only to be shown in IDE
file(GLOB_RECURSE RUST_SOURCES "${CMAKE_CURRENT_LIST_DIR}/src/*.rs")
add_custom_target(rkernel1_files SOURCES ${RUST_SOURCES})
add_custom_target(rkernel1_build_files SOURCES
    ${CMAKE_CURRENT_LIST_DIR}/build.rs
    ${CMAKE_CURRENT_LIST_DIR}/Cargo.toml
    ${CMAKE_CURRENT_LIST_DIR}/.cargo/config.toml
    ${CMAKE_CURRENT_LIST_DIR}/targets/${ARCH_DUPLE}/linker.ld)

add_executable(kernel1
    ${CMAKE_CURRENT_LIST_DIR}/src/main.cpp
    ${CMAKE_CURRENT_LIST_DIR}/src/panic.cpp
    ${CMAKE_CURRENT_LIST_DIR}/src/panic.h
    ${CMAKE_CURRENT_LIST_DIR}/src/targets/${ARCH_DUPLE}/boot.s)

target_link_libraries(kernel1 PUBLIC rkernel1 fake_libc gcc)

target_link_options(kernel1 PUBLIC
    ${NO_STD_LINKER_FLAG}
    -T ${CMAKE_CURRENT_LIST_DIR}/targets/${ARCH_DUPLE}/linker.ld)

if (NOT (EMULATOR))
    target_include_font(kernel1 "https://www.fontsquirrel.com/fonts/download/noto-mono" 14)
    target_link_libraries(kernel1 PUBLIC fake_libc_stdio)
endif()

install(TARGETS kernel1 DESTINATION bin)

if (NOT (EMULATOR))
    add_custom_target(iso
        ALL
        DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/program
    )
    add_custom_command(
        COMMAND bash ${CMAKE_CURRENT_LIST_DIR}/tools/pack ${CMAKE_BINARY_DIR}/kernel1 ${CMAKE_CURRENT_LIST_DIR}/targets/${ARCH_DUPLE}/iso/boot/grub/grub.cfg
        COMMENT "Generating multiboot iso"
        OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/program
        BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/program
        DEPENDS kernel1)
    install(FILES ${CMAKE_BINARY_DIR}/kernel1.iso DESTINATION bin)
endif()

# Package Generator  #######################################################
set(CPACK_PACKAGE_DESCRIPTION ${CMAKE_PROJECT_DESCRIPTION})
set(CPACK_PACKAGE_VERSION_MAJOR ${VERSION_MAJOR})
set(CPACK_PACKAGE_VERSION_MINOR ${VERSION_MINOR})
set(CPACK_PACKAGE_VERSION_PATCH ${VERSION_PATCH})
set(CPACK_PACKAGE_VENDOR "burbokop")
set(CPACK_PACKAGE_CONTACT "burbokop@gmail.com")
set(CPACK_PACKAGE_FILE_NAME "${CMAKE_PROJECT_NAME}-${ARCH_DUPLE}-${CMAKE_PROJECT_VERSION}")
include (CPack)

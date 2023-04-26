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

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

if (NOT (WIN32 OR UNIX))
    add_library(__cstd STATIC
        ${CMAKE_CURRENT_LIST_DIR}/src/cstd.c
        ${CMAKE_CURRENT_LIST_DIR}/src/cstd.h)

    target_link_options(__cstd PUBLIC -nostdlib)
    target_link_libraries(__cstd PUBLIC gcc)
endif()

if (WIN32 OR UNIX)
    set(CORROSION_FLAGS)
else()
    set(CORROSION_FLAGS -Zbuild-std=core,alloc)
endif()

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH ${CMAKE_CURRENT_LIST_DIR}/Cargo.toml
    FLAGS ${CORROSION_FLAGS})

#target_link_libraries(rkernel1 INTERFACE cstd)

add_executable(kernel1)

target_sources(kernel1 PRIVATE
    ${CMAKE_CURRENT_LIST_DIR}/src/main.cpp)
target_link_libraries(kernel1 PUBLIC rkernel1)

if (NOT (WIN32 OR UNIX))
    target_sources(kernel1 PRIVATE
        ${CMAKE_CURRENT_LIST_DIR}/src/${ARCH_DUPLE}/boot.s)

    target_link_libraries(kernel1 PUBLIC __cstd gcc)
    target_link_options(kernel1 PUBLIC
        -nostdlib
        -T ${CMAKE_CURRENT_LIST_DIR}/targets/${ARCH_DUPLE}/linker.ld)
endif()

add_custom_target(iso
    ALL
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/program
)

install(TARGETS kernel1 DESTINATION bin)

if (NOT (WIN32 OR UNIX))
    add_custom_command(
        COMMAND bash -c "${CMAKE_CURRENT_LIST_DIR}/tools/pack ${CMAKE_BINARY_DIR}/kernel1 ${CMAKE_CURRENT_LIST_DIR}/targets/${ARCH_DUPLE}/iso/boot/grub/grub.cfg"
        COMMENT "Generating multiboot iso"
        OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/program
        BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/program
        DEPENDS kernel1)
    install(FILES ${CMAKE_BINARY_DIR}/kernel.iso DESTINATION bin)
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

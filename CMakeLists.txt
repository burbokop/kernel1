cmake_minimum_required(VERSION 3.26.3)

#set(CMAKE_CXX_FLAGS "-static-libstdc++")
#set(CMAKE_CXX_FLAGS "-ffreestanding -static-libstdc++")

set(CMAKE_CXX_FLAGS "-ffreestanding -O2 -Wall -Wextra -std=gnu99 -fno-rtti")

if(DEFINED CMAKE_TOOLCHAIN_FILE)
    include(${CMAKE_TOOLCHAIN_FILE})
endif()

project(kernel1
    DESCRIPTION "Test kernel"
    VERSION 0.0.1
    LANGUAGES CXX ASM)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH ${CMAKE_CURRENT_LIST_DIR}/Cargo.toml
    FLAGS -Z build-std=core)

add_executable(kernel1.bin
    ${CMAKE_CURRENT_LIST_DIR}/src/${ARCH_DUPLE}/boot.s
    #${CMAKE_CURRENT_LIST_DIR}/src/${ARCH_DUPLE}/crt0.s
    ${CMAKE_CURRENT_LIST_DIR}/src/main.cpp)

target_link_libraries(kernel1.bin PUBLIC rkernel1)
target_link_options(kernel1.bin PUBLIC -nostdlib -lgcc -T ${CMAKE_CURRENT_LIST_DIR}/targets/${ARCH_DUPLE}/linker.ld -L ${CMAKE_CURRENT_LIST_DIR})

add_custom_target(iso
    ALL
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/program
)

add_custom_command(
    COMMAND ${CMAKE_CURRENT_LIST_DIR}/tools/pack ${CMAKE_BINARY_DIR}/kernel1.bin ${CMAKE_CURRENT_LIST_DIR}/targets/${ARCH_DUPLE}/iso/boot/grub/grub.cfg
    COMMENT "Generating multiboot iso"
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/program
    BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/program
    DEPENDS kernel1.bin
)

install(TARGETS kernel1.bin DESTINATION bin)
install(FILES ${CMAKE_BINARY_DIR}/kernel.iso DESTINATION bin)

# Package Generator  #######################################################
set(CPACK_PACKAGE_DESCRIPTION ${CMAKE_PROJECT_DESCRIPTION})
set(CPACK_PACKAGE_VERSION_MAJOR ${VERSION_MAJOR})
set(CPACK_PACKAGE_VERSION_MINOR ${VERSION_MINOR})
set(CPACK_PACKAGE_VERSION_PATCH ${VERSION_PATCH})
set(CPACK_PACKAGE_VENDOR "burbokop")
set(CPACK_PACKAGE_CONTACT "burbokop@gmail.com")
set(CPACK_PACKAGE_FILE_NAME "${CMAKE_PROJECT_NAME}-${ARCH_DUPLE}-${CMAKE_PROJECT_VERSION}")
include (CPack)

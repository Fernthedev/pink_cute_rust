# THIS BUILD SCRIPT EXISTS ONLY TO GENERATE A COMPILE_COMMANDS FILE FOR PROPER INTELLISENSE
# DO NOT USE THIS FOR BUILDING THE MOD, USE THE BUILD SCRIPTS IN QPM.JSON

cmake_minimum_required(VERSION 3.22)

option(QUEST "Build for quest" ON)

# Globals
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

set(SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/)
set(INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

add_compile_definitions(MOD_ID="${CMAKE_PROJECT_NAME}")
add_compile_definitions(VERSION="${CMAKE_PROJECT_VERSION}")

add_compile_options(-frtti -fexceptions -fvisibility=hidden -fPIE -fPIC)

include(../qpm_defines.cmake)



# Include. Include order matters!
include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/utils.cmake)
include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/qpm.cmake)

set(EXTERN_DIR_NAME extern)
set(EXTERN_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../${EXTERN_DIR_NAME})
message(STATUS "EXTERN_DIR: ${EXTERN_DIR}")

if(${CMAKE_BUILD_TYPE} STREQUAL "RELEASE" OR ${CMAKE_BUILD_TYPE} STREQUAL "RelWithDebInfo" OR ${CMAKE_BUILD_TYPE} STREQUAL "MinSizeRel")
        # Better optimizations
        add_compile_options(-O3)

        # LTO
        set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
        add_compile_options(-flto)
endif()

if(${CMAKE_BUILD_TYPE} STREQUAL "DEBUG" OR ${CMAKE_BUILD_TYPE} STREQUAL "RelWithDebInfo")
        add_compile_options(-g)
endif()

# Targets
if(QUEST)
        include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/targets/quest.cmake)
endif()

# Post build
include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/strip.cmake)

# stop symbols leaking
# TODO: Fix
# add_link_options(-Wl, --exclude-libs, ALL)
project(party_panel_quest
        VERSION ${PACKAGE_VERSION})


# Set COMPILE_ID for qpm purposes
set(COMPILE_ID ${CMAKE_PROJECT_NAME})

# recursively get all src files
RECURSE_FILES(cpp_file_list ${SOURCE_DIR}/*.cpp)
RECURSE_FILES(c_file_list ${SOURCE_DIR}/*.c)

add_library(
        ${CMAKE_PROJECT_NAME}
        SHARED
        ./quest_compat.cpp
)

# for inline hook
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -llog)

target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${INCLUDE_DIR})
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${SHARED_DIR})

include(../extern.cmake)


# beatsaber hook inline hook

# RECURSE_FILES(src_inline_hook_beatsaber_hook_local_extra_c ${EXTERN_DIR}/includes/beatsaber-hook/shared/inline-hook/*.c)
# RECURSE_FILES(src_inline_hook_beatsaber_hook_local_extra_cpp ${EXTERN_DIR}/includes/beatsaber-hook/shared/inline-hook/*.cpp)

# target_sources(${COMPILE_ID} PRIVATE ${src_inline_hook_beatsaber_hook_local_extra_c})
# target_sources(${COMPILE_ID} PRIVATE ${src_inline_hook_beatsaber_hook_local_extra_cpp})
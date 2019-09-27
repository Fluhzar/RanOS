cmake_minimum_required(VERSION 3.8)

project(AnimationGenerator CXX)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_SOURCE_DIR ${PROJECT_SOURCE_DIR}/src)
set(CMAKE_BINARY_DIR ${PROJECT_SOURCE_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

set(PROJECT_LIB_DIR ${PROJECT_SOURCE_DIR}/lib)
set(PROJECT_INC_DIR ${PROJECT_SOURCE_DIR}/inc)

include_directories(${PROJECT_INC_DIR})

set(PLATFORM_LIB_DIR ${PROJECT_LIB_DIR})

link_directories(BEFORE ${PLATFORM_LIB_DIR})
set(CMAKE_PREFIX_PATH ${PLATFORM_LIB_DIR} ${CMAKE_PREFIX_PATH})

if(MSVC)
  set(COMPILER_FLAGS "/DWin32 /D_WINDOWS /W4 /WX /GR /EHsc" CACHE STRING "Set compiler flags here")
  set(DEBUG_FLAG "")
else()
  set(COMPILER_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Werror -Wconversion
 -Winline -Wfloat-equal -Wundef -Wshadow -Wpointer-arith -Wcast-align\
 -Wwrite-strings -Wcast-qual -Wswitch-default -Wswitch-enum -Wunreachable-code\
 -Wduplicated-cond -Wnull-dereference -Wold-style-cast -Wdouble-promotion\
 -Wformat=2 -Wpedantic -Weffc++ -g -std=c++1z -fPIC ${CMAKE_CXX_FLAGS}"
                      CACHE STRING "Set compiler flags here")
  set(DEBUG_FLAG "-D_DEBUG -pg" CACHE STRING "\"-D_DEBUG -pg\" or \"-DNDEBUG -O4 -g\"")
endif()

set(CMAKE_CXX_FLAGS "${COMPILER_FLAGS} ${DEBUG_FLAGS}")

set(DEFAULT_OUTPUT "Output.bin" CACHE STRING "The default output file for the generated animations")
add_definitions("-DRANOS_DEFAULT_FILE=\"${DEFAULT_OUTPUT}\"")

set(DATA_LOCATION "dat/" CACHE STRING "The location for the data files containing animation data")
add_definitions("-DRANOS_DATA_DIR=\"${DATA_LOCATION}\"")

set(TESTS_ACTIVE ON CACHE BOOL "Sets whether or not tests will run (build to exe) or not (build to lib)")

set(external_files
  ${CMAKE_SOURCE_DIR}/external/jsoncpp.cpp
)
set(generator_files
  ${CMAKE_SOURCE_DIR}/Animation.hpp
  ${CMAKE_SOURCE_DIR}/gen.cpp ${CMAKE_SOURCE_DIR}/gen.hpp
  ${CMAKE_SOURCE_DIR}/RGB.hpp
)
set(test_files
  ${CMAKE_SOURCE_DIR}/main.cpp
)

source_group("External Source" FILES ${external_files})
source_group("Generator" FILES ${generator_files})

if(TESTS_ACTIVE)
  add_executable(${PROJECT_NAME}
                  ${external_files}
                  ${generator_files}
                  ${test_files}
  )
else()
  add_library(${PROJECT_NAME} STATIC
                ${external_files}
                ${generator_files}
  )
endif()

target_link_libraries(${PROJECT_NAME}
                        RIFF
)

if(WIN32)
  add_definitions(-D_CRT_SECURE_NO_WARNINGS)
  set_target_properties(${PROJECT_NAME} PROPERTIES WINDOWS_EXPORT_ALL_SYMBOLS true)
endif()
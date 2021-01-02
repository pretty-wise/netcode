cmake_minimum_required(VERSION 3.16)
project(netcode)

add_definitions(-DCMAKE_EXPORT_COMPILE_COMMANDS=ON)

set (CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set (CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

#
# compiler options
#

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_BUILD_TYPE Debug)
set (CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g -DDEBUG -D_DEBUG")

add_subdirectory(extern)

#
# client executable
#

find_package(glfw3)
find_package(OpenGL)
find_package(glew CONFIG)

add_executable (sample_listenserver
	samples/sample_listenserver/src/main.mm
	samples/sample_listenserver/src/imgui_impl_opengl2.cpp
	samples/sample_listenserver/src/imgui_impl_opengl2.h
	samples/sample_listenserver/src/imgui_impl_osx.h
	samples/sample_listenserver/src/imgui_impl_osx.mm
)

target_link_libraries(sample_listenserver PRIVATE imgui "-framework Foundation" "-framework Cocoa" OpenGL::GL)

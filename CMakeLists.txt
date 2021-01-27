cmake_minimum_required(VERSION 3.8)
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
# rust code
#

set(CARGO_LIBNAME "network")
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(CARGO_TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(CARGO_TARGET_DIR "release")
endif ()

file(MAKE_DIRECTORY ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/generated)

set(NETWORK_LIB ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/lib${CARGO_LIBNAME}.a)
set(NETWORK_HEADER ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/generated/${CARGO_LIBNAME}.h)
set(NETWORK_SOURCES 
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/lib.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/client/mod.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/server/mod.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/server/actor_ids.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/server/cmd_buffer.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/server/control.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/server/simulation.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/server/world.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/shared/mod.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/shared/socketio.rs
	${CMAKE_CURRENT_SOURCE_DIR}/${CARGO_LIBNAME}/src/shared/types.rs
)

add_custom_command(OUTPUT ${NETWORK_LIB}
	COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}/rust ${CARGO_CMD}
	COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/rust/${CARGO_TARGET_DIR}/lib${CARGO_LIBNAME}.a ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}
	DEPENDS ${NETWORK_SOURCES}
	COMMENT "compiling rust lib..."
)

add_custom_command(OUTPUT ${NETWORK_HEADER}
	COMMAND cbindgen --config cbindgen.toml --lang c --crate ${CARGO_LIBNAME} --output ${NETWORK_HEADER}
	WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
	DEPENDS ${NETWORK_LIB}
	COMMENT "generating rust lib c headers..."
)

# target that builds rust code
add_custom_target(rust_build
	DEPENDS ${NETWORK_LIB}
	DEPENDS ${NETWORK_HEADER}
	SOURCES ${NETWORK_SOURCES}
	COMMENT "generating rust target..."
)

# static library with include directories
add_library(rust_network STATIC IMPORTED)
set_property(TARGET rust_network PROPERTY
			 IMPORTED_LOCATION ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/lib${CARGO_LIBNAME}.a
)

set_target_properties(rust_network PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/generated"
)

# dependency to ensure rust code is built
add_dependencies(rust_network rust_build)

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


target_link_libraries(sample_listenserver PRIVATE imgui rust_network "-framework Foundation" "-framework Cocoa" OpenGL::GL)

#add_dependencies(sample_listenserver rust_network)
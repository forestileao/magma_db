cmake_minimum_required(VERSION 3.26)
project(magma_db)

set(CMAKE_CXX_STANDARD 23)

include(DownloadProject.cmake)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -march=native")

add_executable(magma_db src/main.cpp
        src/core/Document.cpp
        src/core/Document.h
        src/core/DocumentValue.cpp
        src/core/DocumentValue.h
        src/core/UUID.cpp
        src/core/UUID.h
)

download_project(
        PROJ           uuid_v4
        GIT_REPOSITORY https://github.com/crashoz/uuid_v4.git
        GIT_TAG        master
)

include_directories(${uuid_v4_SOURCE_DIR}/)
add_subdirectory(${uuid_v4_SOURCE_DIR} ${uuid_v4_BINARY_DIR})
target_link_libraries(magma_db uuid_v4)
find_package(uuid_v4 CONFIG REQUIRED)


cmake_minimum_required(VERSION 3.26)
project(magma_db)

set(CMAKE_CXX_STANDARD 23)

add_executable(magma_db src/main.cpp
        src/core/Document.cpp
        src/core/Document.h
        src/core/DocumentValue.cpp
        src/core/DocumentValue.h
        src/core/UUID.cpp
        src/core/UUID.h
)

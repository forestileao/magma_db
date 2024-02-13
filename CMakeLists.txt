cmake_minimum_required(VERSION 3.26)
project(pedro_db)

set(CMAKE_CXX_STANDARD 23)

add_executable(pedro_db src/main.cpp
        src/core/Document.cpp
        src/core/Document.h
        src/core/DocumentValue.cpp
        src/core/DocumentValue.h
        src/core/UUID.cpp
        src/core/UUID.h
)

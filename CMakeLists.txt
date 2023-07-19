cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(Lisp
        VERSION 0.1.0
        LANGUAGES CXX
        DESCRIPTION "A Lisp interpreter written in C++"
        HOMEPAGE_URL "https://github.com/jsilll/lisp")

if(MSVC)
	add_compile_options(/W4 /WX)
else()
	add_compile_options(-Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic -Werror)
endif()

file(GLOB SOURCES src/*.cpp)

file(GLOB HEADERS include/*.hpp)

add_library(lisp STATIC ${SOURCES} ${HEADERS})

target_include_directories(lisp PUBLIC include)

add_subdirectory(app)
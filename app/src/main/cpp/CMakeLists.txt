cmake_minimum_required(VERSION 3.18.1)

project("myrustapp")

add_library( myrustapp SHARED native-lib.cpp)

find_library( log-lib log)

target_link_libraries( myrustapp ${log-lib})

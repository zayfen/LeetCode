cmake_minimum_required(VERSION 2.8)

project(LeetCode)

get_filename_component(LEETCODE_PRO_DIR ${CMAKE_CURRENT_SOURCE_DIR} PATH)
message(${LEETCODE_PRO_DIR})

##if(USE_CPP11)
  ## We can't turn this on by default, since it breaks on travis
  message(STATUS "Enabling C++11")
  set(CMAKE_CXX_FLAGS "-std=c++11 ${CMAKE_CXX_FLAGS}")
##endif()

set(SOURCES
  ${LEETCODE_PRO_DIR}/test/testMain.cpp
  ${LEETCODE_PRO_DIR}/test/LongestPalindromeSubStringTest.cpp
  ${LEETCODE_PRO_DIR}/test/ReverseIntegerTest.cpp
  ${LEETCODE_PRO_DIR}/test/AtoITest.cpp
  )

include_directories(${LEETCODE_PRO_DIR}/test/external/single_include  ${LEETCODE_PRO_DIR}/src)
add_executable(LeetCodeTest ${SOURCES})

enable_testing()

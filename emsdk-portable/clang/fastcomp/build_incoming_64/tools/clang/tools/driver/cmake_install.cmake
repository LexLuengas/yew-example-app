# Install script for directory: /Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/driver

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/usr/local")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "clang" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES
    "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/build_incoming_64/bin/clang-6.0"
    "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/build_incoming_64/bin/clang"
    )
  foreach(file
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/clang-6.0"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/clang"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      if(CMAKE_INSTALL_DO_STRIP)
        execute_process(COMMAND "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/strip" "${file}")
      endif()
    endif()
  endforeach()
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "clang" OR NOT CMAKE_INSTALL_COMPONENT)
  include("/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/cmake/modules/LLVMInstallSymlink.cmake")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "clang" OR NOT CMAKE_INSTALL_COMPONENT)
  install_symlink(clang++ clang bin)
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "clang" OR NOT CMAKE_INSTALL_COMPONENT)
  include("/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/cmake/modules/LLVMInstallSymlink.cmake")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "clang" OR NOT CMAKE_INSTALL_COMPONENT)
  install_symlink(clang-cl clang bin)
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "clang" OR NOT CMAKE_INSTALL_COMPONENT)
  include("/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/cmake/modules/LLVMInstallSymlink.cmake")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "clang" OR NOT CMAKE_INSTALL_COMPONENT)
  install_symlink(clang-cpp clang bin)
endif()


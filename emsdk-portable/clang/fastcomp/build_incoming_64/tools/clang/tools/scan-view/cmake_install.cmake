# Install script for directory: /Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view

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

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view/bin/scan-view")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/scan-view" TYPE FILE FILES "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view/share/ScanView.py")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/scan-view" TYPE FILE FILES "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view/share/Reporter.py")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/scan-view" TYPE FILE FILES "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view/share/startfile.py")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/scan-view" TYPE FILE FILES "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view/share/FileRadar.scpt")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/scan-view" TYPE FILE FILES "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view/share/GetRadarVersion.scpt")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/scan-view" TYPE FILE FILES "/Users/Alexis/Documents/dev/bthesis/emsdk-portable/clang/fastcomp/src/tools/clang/tools/scan-view/share/bugcatcher.ico")
endif()


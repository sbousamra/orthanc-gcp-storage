cmake_minimum_required(VERSION 3.10.0)
project(
  orthanc-gcp-storage 
  VERSION 1.0.0 
  DESCRIPTION "Orthanc plugin to interface with gcp storage"
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS_DEBUG "-g")

add_library(
  orthanc_gcp_storage 
  SHARED
  src/plugin.cpp
)
target_include_directories(orthanc_gcp_storage PRIVATE src)

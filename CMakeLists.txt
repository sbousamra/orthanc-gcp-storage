cmake_minimum_required(VERSION 3.10)
project(orthanc-gcp-storage)

set(CMAKE_CXX_STANDARD 20
set(CMAKE_CXX_FLAGS_DEBUG "-g"))

set(ORTHANC_GCP_STORAGE_SRC_DIR ${CMAKE_CURRENT_LIST_DIR}/src)
file(GLOB_RECURSE ORTHANC_GCP_STORAGE_SRC_DIR ${ORTHANC_GCP_STORAGE_SRC_DIR}/*)

add_library(orthanc_gcp_storage SHARED ${ORTHANC_GCP_STORAGE_SRC_DIR})
target_include_directories(orthanc_gcp_storage PRIVATE ${ORTHANC_GCP_STORAGE_SRC_DIR})

cmake_minimum_required(VERSION 3.10)
project(sample)

if (WIN32)
    find_path(CURL_INCLUDE_DIRS curl/curl.h)
    find_library(CURL_LIBRARIES 
      NAMES curl libcurl
    )
    message("Curl include dirs: " ${CURL_INCLUDE_DIRS})
    message("Curl libraries: " ${CURL_LIBRARIES})
else()
    find_package(CURL REQUIRED)
endif()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -pedantic -std=c++14")
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS} ${CMAKE_CXX_FLAGS_DEBUG} -g")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS} ${CMAKE_CXX_FLAGS_RELEASE} -O2")

include_directories(../src)

# nx kit
set(nxKitLibraryType "STATIC" CACHE STRING "" FORCE)
set(nxKitWithTests "NO" CACHE STRING "" FORCE)
include_directories(${CMAKE_CURRENT_LIST_DIR}/../nx/nx_kit/src ${CURL_INCLUDE_DIRS})
add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/../nx/nx_kit ${CMAKE_CURRENT_LIST_DIR}/../build/nx/nx_kit)

add_executable(hub-client-sample
  hub-client-sample.cpp
  ../src/http_client.cpp
  ../src/hub_client.cpp
  ../src/logger.cpp)
target_link_libraries(hub-client-sample ${CURL_LIBRARIES} nx_kit)

add_executable(hub-health-service-sample
  hub-health-service-sample.cpp
  ../src/http_client.cpp
  ../src/hub_client.cpp
  ../src/hub_health_service.cpp
  ../src/logger.cpp)
if (WIN32)
  target_link_libraries(hub-health-service-sample ${CURL_LIBRARIES} nx_kit)
else()
  target_link_libraries(hub-health-service-sample pthread ${CURL_LIBRARIES} nx_kit)
endif()

add_executable(auth-client-sample
  auth-client-sample.cpp
  ../src/http_client.cpp
  ../src/auth_client.cpp
  ../src/utils.cpp
  ../src/config/auth_client_config.cpp
  ../src/logger.cpp)
target_link_libraries(auth-client-sample ${CURL_LIBRARIES} nx_kit)
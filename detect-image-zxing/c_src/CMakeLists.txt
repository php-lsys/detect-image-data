add_library(zxing_c_api reader.cpp)
link_directories("/usr/local/lib")
target_link_libraries(zxing_c_api stb ZXing)

add_executable(dome_zxing dome.cpp)
target_link_libraries(dome_zxing ZXing stb zxing_c_api)
SET(CMAKE_BUILD_TYPE "Debug")
SET(CMAKE_CXX_FLAGS_DEBUG "$ENV{CXXFLAGS} -O0 -Wall -g2 -ggdb")
SET(CMAKE_CXX_FLAGS_RELEASE "$ENV{CXXFLAGS} -O3 -Wall")
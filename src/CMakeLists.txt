# src/CMakeLists.txt

# Add source files
set(SOURCE_FILES
    main.cpp
    ./core/Logger.cpp
    # Add other source files here
)

# Add executable
add_executable(BaseWolfEngine ${SOURCE_FILES})

# Link libraries if necessary
# target_link_libraries(BaseWolfEngine <library_name>)
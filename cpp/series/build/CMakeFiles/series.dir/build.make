# CMAKE generated file: DO NOT EDIT!
# Generated by "MinGW Makefiles" Generator, CMake Version 3.19

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Disable VCS-based implicit rules.
% : %,v


# Disable VCS-based implicit rules.
% : RCS/%


# Disable VCS-based implicit rules.
% : RCS/%,v


# Disable VCS-based implicit rules.
% : SCCS/s.%


# Disable VCS-based implicit rules.
% : s.%


.SUFFIXES: .hpux_make_needs_suffix_list


# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

SHELL = cmd.exe

# The CMake executable.
CMAKE_COMMAND = C:\cmake\bin\cmake.exe

# The command to remove a file.
RM = C:\cmake\bin\cmake.exe -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = "E:\playground\Exercism Solutions\cpp\series"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "E:\playground\Exercism Solutions\cpp\series\build"

# Include any dependencies generated for this target.
include CMakeFiles/series.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/series.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/series.dir/flags.make

CMakeFiles/series.dir/series_test.cpp.obj: CMakeFiles/series.dir/flags.make
CMakeFiles/series.dir/series_test.cpp.obj: ../series_test.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\series\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/series.dir/series_test.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\series.dir\series_test.cpp.obj -c "E:\playground\Exercism Solutions\cpp\series\series_test.cpp"

CMakeFiles/series.dir/series_test.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/series.dir/series_test.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\series\series_test.cpp" > CMakeFiles\series.dir\series_test.cpp.i

CMakeFiles/series.dir/series_test.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/series.dir/series_test.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\series\series_test.cpp" -o CMakeFiles\series.dir\series_test.cpp.s

CMakeFiles/series.dir/series.cpp.obj: CMakeFiles/series.dir/flags.make
CMakeFiles/series.dir/series.cpp.obj: ../series.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\series\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/series.dir/series.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\series.dir\series.cpp.obj -c "E:\playground\Exercism Solutions\cpp\series\series.cpp"

CMakeFiles/series.dir/series.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/series.dir/series.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\series\series.cpp" > CMakeFiles\series.dir\series.cpp.i

CMakeFiles/series.dir/series.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/series.dir/series.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\series\series.cpp" -o CMakeFiles\series.dir\series.cpp.s

CMakeFiles/series.dir/test/tests-main.cpp.obj: CMakeFiles/series.dir/flags.make
CMakeFiles/series.dir/test/tests-main.cpp.obj: ../test/tests-main.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\series\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/series.dir/test/tests-main.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\series.dir\test\tests-main.cpp.obj -c "E:\playground\Exercism Solutions\cpp\series\test\tests-main.cpp"

CMakeFiles/series.dir/test/tests-main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/series.dir/test/tests-main.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\series\test\tests-main.cpp" > CMakeFiles\series.dir\test\tests-main.cpp.i

CMakeFiles/series.dir/test/tests-main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/series.dir/test/tests-main.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\series\test\tests-main.cpp" -o CMakeFiles\series.dir\test\tests-main.cpp.s

# Object files for target series
series_OBJECTS = \
"CMakeFiles/series.dir/series_test.cpp.obj" \
"CMakeFiles/series.dir/series.cpp.obj" \
"CMakeFiles/series.dir/test/tests-main.cpp.obj"

# External object files for target series
series_EXTERNAL_OBJECTS =

series.exe: CMakeFiles/series.dir/series_test.cpp.obj
series.exe: CMakeFiles/series.dir/series.cpp.obj
series.exe: CMakeFiles/series.dir/test/tests-main.cpp.obj
series.exe: CMakeFiles/series.dir/build.make
series.exe: CMakeFiles/series.dir/linklibs.rsp
series.exe: CMakeFiles/series.dir/objects1.rsp
series.exe: CMakeFiles/series.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir="E:\playground\Exercism Solutions\cpp\series\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_4) "Linking CXX executable series.exe"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles\series.dir\link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/series.dir/build: series.exe

.PHONY : CMakeFiles/series.dir/build

CMakeFiles/series.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles\series.dir\cmake_clean.cmake
.PHONY : CMakeFiles/series.dir/clean

CMakeFiles/series.dir/depend:
	$(CMAKE_COMMAND) -E cmake_depends "MinGW Makefiles" "E:\playground\Exercism Solutions\cpp\series" "E:\playground\Exercism Solutions\cpp\series" "E:\playground\Exercism Solutions\cpp\series\build" "E:\playground\Exercism Solutions\cpp\series\build" "E:\playground\Exercism Solutions\cpp\series\build\CMakeFiles\series.dir\DependInfo.cmake" --color=$(COLOR)
.PHONY : CMakeFiles/series.dir/depend


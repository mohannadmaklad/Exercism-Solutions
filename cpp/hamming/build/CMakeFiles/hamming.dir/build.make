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
CMAKE_SOURCE_DIR = "E:\playground\Exercism Solutions\cpp\hamming"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "E:\playground\Exercism Solutions\cpp\hamming\build"

# Include any dependencies generated for this target.
include CMakeFiles/hamming.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/hamming.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/hamming.dir/flags.make

CMakeFiles/hamming.dir/hamming_test.cpp.obj: CMakeFiles/hamming.dir/flags.make
CMakeFiles/hamming.dir/hamming_test.cpp.obj: ../hamming_test.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\hamming\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/hamming.dir/hamming_test.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\hamming.dir\hamming_test.cpp.obj -c "E:\playground\Exercism Solutions\cpp\hamming\hamming_test.cpp"

CMakeFiles/hamming.dir/hamming_test.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/hamming.dir/hamming_test.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\hamming\hamming_test.cpp" > CMakeFiles\hamming.dir\hamming_test.cpp.i

CMakeFiles/hamming.dir/hamming_test.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/hamming.dir/hamming_test.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\hamming\hamming_test.cpp" -o CMakeFiles\hamming.dir\hamming_test.cpp.s

CMakeFiles/hamming.dir/hamming.cpp.obj: CMakeFiles/hamming.dir/flags.make
CMakeFiles/hamming.dir/hamming.cpp.obj: ../hamming.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\hamming\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/hamming.dir/hamming.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\hamming.dir\hamming.cpp.obj -c "E:\playground\Exercism Solutions\cpp\hamming\hamming.cpp"

CMakeFiles/hamming.dir/hamming.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/hamming.dir/hamming.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\hamming\hamming.cpp" > CMakeFiles\hamming.dir\hamming.cpp.i

CMakeFiles/hamming.dir/hamming.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/hamming.dir/hamming.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\hamming\hamming.cpp" -o CMakeFiles\hamming.dir\hamming.cpp.s

CMakeFiles/hamming.dir/test/tests-main.cpp.obj: CMakeFiles/hamming.dir/flags.make
CMakeFiles/hamming.dir/test/tests-main.cpp.obj: ../test/tests-main.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\hamming\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/hamming.dir/test/tests-main.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\hamming.dir\test\tests-main.cpp.obj -c "E:\playground\Exercism Solutions\cpp\hamming\test\tests-main.cpp"

CMakeFiles/hamming.dir/test/tests-main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/hamming.dir/test/tests-main.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\hamming\test\tests-main.cpp" > CMakeFiles\hamming.dir\test\tests-main.cpp.i

CMakeFiles/hamming.dir/test/tests-main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/hamming.dir/test/tests-main.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\hamming\test\tests-main.cpp" -o CMakeFiles\hamming.dir\test\tests-main.cpp.s

# Object files for target hamming
hamming_OBJECTS = \
"CMakeFiles/hamming.dir/hamming_test.cpp.obj" \
"CMakeFiles/hamming.dir/hamming.cpp.obj" \
"CMakeFiles/hamming.dir/test/tests-main.cpp.obj"

# External object files for target hamming
hamming_EXTERNAL_OBJECTS =

hamming.exe: CMakeFiles/hamming.dir/hamming_test.cpp.obj
hamming.exe: CMakeFiles/hamming.dir/hamming.cpp.obj
hamming.exe: CMakeFiles/hamming.dir/test/tests-main.cpp.obj
hamming.exe: CMakeFiles/hamming.dir/build.make
hamming.exe: CMakeFiles/hamming.dir/linklibs.rsp
hamming.exe: CMakeFiles/hamming.dir/objects1.rsp
hamming.exe: CMakeFiles/hamming.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir="E:\playground\Exercism Solutions\cpp\hamming\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_4) "Linking CXX executable hamming.exe"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles\hamming.dir\link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/hamming.dir/build: hamming.exe

.PHONY : CMakeFiles/hamming.dir/build

CMakeFiles/hamming.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles\hamming.dir\cmake_clean.cmake
.PHONY : CMakeFiles/hamming.dir/clean

CMakeFiles/hamming.dir/depend:
	$(CMAKE_COMMAND) -E cmake_depends "MinGW Makefiles" "E:\playground\Exercism Solutions\cpp\hamming" "E:\playground\Exercism Solutions\cpp\hamming" "E:\playground\Exercism Solutions\cpp\hamming\build" "E:\playground\Exercism Solutions\cpp\hamming\build" "E:\playground\Exercism Solutions\cpp\hamming\build\CMakeFiles\hamming.dir\DependInfo.cmake" --color=$(COLOR)
.PHONY : CMakeFiles/hamming.dir/depend

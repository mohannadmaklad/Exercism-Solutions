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
CMAKE_SOURCE_DIR = "E:\playground\Exercism Solutions\cpp\word-count"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "E:\playground\Exercism Solutions\cpp\word-count\build"

# Include any dependencies generated for this target.
include CMakeFiles/word-count.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/word-count.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/word-count.dir/flags.make

CMakeFiles/word-count.dir/word_count_test.cpp.obj: CMakeFiles/word-count.dir/flags.make
CMakeFiles/word-count.dir/word_count_test.cpp.obj: ../word_count_test.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\word-count\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/word-count.dir/word_count_test.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\word-count.dir\word_count_test.cpp.obj -c "E:\playground\Exercism Solutions\cpp\word-count\word_count_test.cpp"

CMakeFiles/word-count.dir/word_count_test.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/word-count.dir/word_count_test.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\word-count\word_count_test.cpp" > CMakeFiles\word-count.dir\word_count_test.cpp.i

CMakeFiles/word-count.dir/word_count_test.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/word-count.dir/word_count_test.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\word-count\word_count_test.cpp" -o CMakeFiles\word-count.dir\word_count_test.cpp.s

CMakeFiles/word-count.dir/word_count.cpp.obj: CMakeFiles/word-count.dir/flags.make
CMakeFiles/word-count.dir/word_count.cpp.obj: ../word_count.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\word-count\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/word-count.dir/word_count.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\word-count.dir\word_count.cpp.obj -c "E:\playground\Exercism Solutions\cpp\word-count\word_count.cpp"

CMakeFiles/word-count.dir/word_count.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/word-count.dir/word_count.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\word-count\word_count.cpp" > CMakeFiles\word-count.dir\word_count.cpp.i

CMakeFiles/word-count.dir/word_count.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/word-count.dir/word_count.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\word-count\word_count.cpp" -o CMakeFiles\word-count.dir\word_count.cpp.s

CMakeFiles/word-count.dir/test/tests-main.cpp.obj: CMakeFiles/word-count.dir/flags.make
CMakeFiles/word-count.dir/test/tests-main.cpp.obj: ../test/tests-main.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="E:\playground\Exercism Solutions\cpp\word-count\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/word-count.dir/test/tests-main.cpp.obj"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles\word-count.dir\test\tests-main.cpp.obj -c "E:\playground\Exercism Solutions\cpp\word-count\test\tests-main.cpp"

CMakeFiles/word-count.dir/test/tests-main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/word-count.dir/test/tests-main.cpp.i"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "E:\playground\Exercism Solutions\cpp\word-count\test\tests-main.cpp" > CMakeFiles\word-count.dir\test\tests-main.cpp.i

CMakeFiles/word-count.dir/test/tests-main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/word-count.dir/test/tests-main.cpp.s"
	C:\MinGW\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "E:\playground\Exercism Solutions\cpp\word-count\test\tests-main.cpp" -o CMakeFiles\word-count.dir\test\tests-main.cpp.s

# Object files for target word-count
word__count_OBJECTS = \
"CMakeFiles/word-count.dir/word_count_test.cpp.obj" \
"CMakeFiles/word-count.dir/word_count.cpp.obj" \
"CMakeFiles/word-count.dir/test/tests-main.cpp.obj"

# External object files for target word-count
word__count_EXTERNAL_OBJECTS =

word-count.exe: CMakeFiles/word-count.dir/word_count_test.cpp.obj
word-count.exe: CMakeFiles/word-count.dir/word_count.cpp.obj
word-count.exe: CMakeFiles/word-count.dir/test/tests-main.cpp.obj
word-count.exe: CMakeFiles/word-count.dir/build.make
word-count.exe: CMakeFiles/word-count.dir/linklibs.rsp
word-count.exe: CMakeFiles/word-count.dir/objects1.rsp
word-count.exe: CMakeFiles/word-count.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir="E:\playground\Exercism Solutions\cpp\word-count\build\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_4) "Linking CXX executable word-count.exe"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles\word-count.dir\link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/word-count.dir/build: word-count.exe

.PHONY : CMakeFiles/word-count.dir/build

CMakeFiles/word-count.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles\word-count.dir\cmake_clean.cmake
.PHONY : CMakeFiles/word-count.dir/clean

CMakeFiles/word-count.dir/depend:
	$(CMAKE_COMMAND) -E cmake_depends "MinGW Makefiles" "E:\playground\Exercism Solutions\cpp\word-count" "E:\playground\Exercism Solutions\cpp\word-count" "E:\playground\Exercism Solutions\cpp\word-count\build" "E:\playground\Exercism Solutions\cpp\word-count\build" "E:\playground\Exercism Solutions\cpp\word-count\build\CMakeFiles\word-count.dir\DependInfo.cmake" --color=$(COLOR)
.PHONY : CMakeFiles/word-count.dir/depend


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
CMAKE_SOURCE_DIR = "E:\playground\Exercism Solutions\cpp\matching-brackets"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "E:\playground\Exercism Solutions\cpp\matching-brackets\build"

# Utility rule file for test_matching-brackets.

# Include the progress variables for this target.
include CMakeFiles/test_matching-brackets.dir/progress.make

CMakeFiles/test_matching-brackets: matching-brackets.exe
	.\matching-brackets.exe

test_matching-brackets: CMakeFiles/test_matching-brackets
test_matching-brackets: CMakeFiles/test_matching-brackets.dir/build.make

.PHONY : test_matching-brackets

# Rule to build all files generated by this target.
CMakeFiles/test_matching-brackets.dir/build: test_matching-brackets

.PHONY : CMakeFiles/test_matching-brackets.dir/build

CMakeFiles/test_matching-brackets.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles\test_matching-brackets.dir\cmake_clean.cmake
.PHONY : CMakeFiles/test_matching-brackets.dir/clean

CMakeFiles/test_matching-brackets.dir/depend:
	$(CMAKE_COMMAND) -E cmake_depends "MinGW Makefiles" "E:\playground\Exercism Solutions\cpp\matching-brackets" "E:\playground\Exercism Solutions\cpp\matching-brackets" "E:\playground\Exercism Solutions\cpp\matching-brackets\build" "E:\playground\Exercism Solutions\cpp\matching-brackets\build" "E:\playground\Exercism Solutions\cpp\matching-brackets\build\CMakeFiles\test_matching-brackets.dir\DependInfo.cmake" --color=$(COLOR)
.PHONY : CMakeFiles/test_matching-brackets.dir/depend


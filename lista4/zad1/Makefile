# Compiler
CC := g++

# Compiler flags
CFLAGS := -Wall -Wextra -Wpedantic

# Source files
SRCS := main.cpp utils.cpp

# Object files
OBJS := $(SRCS:.cpp=.o)

# Executable name
TARGET := main

# Default target
all: $(TARGET)

# Link object files into executable
$(TARGET): $(SRCS)
	$(CC) $(CFLAGS) $^ -o $@

# Clean up object files and executable
clean:
	rm -f $(OBJS) $(TARGET)

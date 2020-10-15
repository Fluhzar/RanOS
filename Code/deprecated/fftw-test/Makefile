comma:=,
empty:=
space:=$(empty) $(empty)

EXE:=bin/fftw-test
EXE_OPTIONS:=dat/engage.wav

CC:=clang
CXX:=clang++
LD:=clang++

CFLAGS:=
CDEFINES_RAW:=
CDEFINES:=$(foreach define,$(CDEFINES_RAW),-D$(define))
CXXFLAGS:=-Wall -Wextra -Werror -Wconversion -Winline -Wfloat-equal -Wundef -Wshadow -Wpointer-arith -Wcast-align -Wwrite-strings -Wcast-qual -Wswitch-default -Wswitch-enum -Wunreachable-code -Wnull-dereference -Wold-style-cast -Wformat=2 -Wpedantic -Weffc++ -g -std=c++1z
CXXDEFINES_RAW:=
CXXDEFINES:=$(foreach define,$(CXXDEFINES_RAW),-D$(define))
LFLAGS:=-lfftw3 -lm
LDEFINES_RAW:=
LDEFINES:=$(foreach define,$(LDEFINES_RAW),-D$(define))

INCS:=inc
INC_DIRS:=$(foreach inc,$(INCS),-I$(inc))

BUILD_DIRS!=find src -type d -print | sed -e 's/\bsrc/build\/&/' | sed ':a;N;$!ba;s/\n/ /g'
DIRS=build $(BUILD_DIRS) bin
SRC_FILES!=find src -name "*.cpp" -printf "%p "
BUILD_FILES!=find src -name "*.cpp" -print | sed -e 's/\bsrc/build\/&/' -e 's/cpp/o/g' | sed ':a;N;$!ba;s/\n/ /g'

all : $(BUILD_DIRS) $(EXE)

$(EXE) : $(BUILD_FILES)
	$(LD) $(LDEFINES) $^ $(LFLAGS) -o $(EXE)

build/%.o : %.cpp
	$(CXX) $(CXXDEFINES) $(INC_DIRS) -c $(CXXFLAGS) $< -o $@

$(BUILD_DIRS) :
	mkdir $(DIRS)

clean:
	rm -rf $(EXE) $(BUILD_FILES) $(BUILD_DIRS)

rebuild : clean all

run:
	./$(EXE) $(EXE_OPTIONS)

CC=g++
CFLAGS=-Wall -L. -I. -DMAIN
CFLAGS_LIB=-Wall -L. -I.
LDFLAGS=-lskia -lpthread
SRC=previews-generator.cpp
BIN=previews-generator

all:
	$(CC) $(CFLAGS) -o $(BIN) $(SRC) $(LDFLAGS)

lib:
	$(CC) $(CFLAGS_LIB) -c $(SRC) $(LDFLAGS)
	ar rvs lib$(BIN).a $(BIN).o
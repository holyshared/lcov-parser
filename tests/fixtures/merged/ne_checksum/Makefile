FILE1 = fixture1.info
FILE2 = fixture2.info
MERGED_FILE = merged_fixture.info
CWD = $(shell pwd)

all: build test clean path

build:
	gcc -g --coverage fixture1.c -o fixture1
	gcc -g --coverage fixture2.c -o fixture2

test:
	./fixture1
	./fixture2
	gcov -fbc fixture1.c
	gcov -fbc fixture2.c
	lcov -t example -c -d . -o $(FILE1) --checksum --rc=lcov_branch_coverage=1
	lcov -t example -c -d . -o $(FILE2) --checksum --rc=lcov_branch_coverage=1

path:
	mv $(FILE1) $(FILE1).original
	mv $(FILE2) $(FILE2).original
	sed 's#$(CWD)##' $(FILE1).original > $(FILE1)
	sed 's#$(CWD)##' $(FILE2).original > $(FILE2)
	mv $(FILE2) $(FILE2).original
	# replace checksum!!
	sed 's#y7GE3Y4FyXCeXcrtqgSVzw#invalid#' $(FILE2).original > $(FILE2)
	rm -rf *.original

clean:
	rm -rf fixture1 fixture2 *.dSYM *.gcda *.gcno

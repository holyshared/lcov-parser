all:
	gcc -g --coverage fixture.c func1.c func2.c -o fixture
test: 
	./fixture
	gcov -fbc fixture.c
	lcov -t example -c -d . -o fixture.info --checksum --rc=lcov_branch_coverage=1
clean:
	rm fixture *.gcda *.gcno *.info

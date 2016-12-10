WITH_TESTNAME_FILE = fixture.info
WITHOUT_TESTNAME_FILE = without_test_name_fixture.info
MERGED_WITH_TESTNAME_FILE = merged_fixture.info
MERGED_WITHOUT_TESTNAME_FILE = marged_without_test_name_fixture.info
WITHOUT_CHECKSUM_FILE = without_checksum_fixture.info

all:
	gcc -g --coverage fixture.c func1.c func2.c -o fixture
test: 
	./fixture
	gcov -fbc fixture.c
	lcov -t example -c -d . -o $(WITH_TESTNAME_FILE) --checksum --rc=lcov_branch_coverage=1
	lcov -t example -c -d . -o $(WITHOUT_CHECKSUM_FILE) --rc=lcov_branch_coverage=1
	lcov -c -d . -o $(WITHOUT_TESTNAME_FILE) --checksum --rc=lcov_branch_coverage=1
	lcov -d . -a $(WITH_TESTNAME_FILE) -a $(WITH_TESTNAME_FILE) -o $(MERGED_WITH_TESTNAME_FILE) --checksum --rc=lcov_branch_coverage=1
	lcov -d . -a $(WITHOUT_TESTNAME_FILE) -a $(WITHOUT_TESTNAME_FILE) -o $(MERGED_WITHOUT_TESTNAME_FILE) --checksum --rc=lcov_branch_coverage=1
clean:
	rm fixture *.gcda *.gcno *.info

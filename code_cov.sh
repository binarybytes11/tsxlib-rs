#!/bin/bash

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

EXCLUDE="/.cargo,/examples,/usr/lib"
TARGET="target/cov"

echo -e "${GREEN}*** Set up kcov ***${NC}"
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz &&
tar xzf master.tar.gz &&
cd kcov-master &&
mkdir build &&
cd build &&
cmake .. &&
make &&
make install DESTDIR=../../kcov-build &&
cd ../.. &&
rm -rf kcov-master &&

TSXLIB_UNIT_TESTS="target/debug/deps/tsxlib-"

export RUSTFLAGS="-C link-dead-code"

echo -e "${GREEN}*** Clean previous coverage results and executables ***${NC}"
rm -rf "$TARGET"
rm -f "$TSXLIB_UNIT_TESTS"*


echo -e "${GREEN}*** Rebuilding tests ***${NC}"
cargo clean
cargo test --no-run


echo -e "${GREEN}*** Run coverage on tsxlib unit tests ***${NC}"
for test_file in `ls "$TSXLIB_UNIT_TESTS"*`
do
    if [[ ! -x "$test_file" ]]; then
        echo -e "${YELLOW}*** skipping non executable $test_file ***${NC}"
        continue
    fi
    echo -e "${GREEN}*** Running $test_file ***${NC}"
    
    mkdir -p "target/cov/$(basename $test_file)" 
    ./kcov-build/usr/local/bin/kcov --exclude-pattern=$EXCLUDE --verify "target/cov/$(basename $test_file)" "$test_file";   
    if [ "$?" != "0" ]; then
        echo -e "${RED}*** Failure during unit test converage ***${NC}"
        exit 1
    fi
    bash <(curl -s https://codecov.io/bash)
    
done

echo -e "${GREEN}*** Coverage completed and uploaded successfully ***${NC}"

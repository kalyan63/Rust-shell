# Here we just run all the files using cargo and add its binary to the bin folder.
cd cat
cargo build
cd ..
cp "cat/target/debug/cat" "bin/cat"

cd cmp
cargo build
cd ..
cp "cmp/target/debug/cmp" "bin/cmp"

cd cp
cargo build
cd ..
cp "cp/target/debug/cp" "bin/cp"

cd echo
cargo build
cd ..
cp "echo/target/debug/echo" "bin/echo"

cd grep
cargo build
cd ..
cp "grep/target/debug/grep" "bin/grep"

cd mkdir
cargo build
cd ..
cp "mkdir/target/debug/mkdir" "bin/mkdir"

cd mv
cargo build
cd ..
cp "mv/target/debug/mv" "bin/mv"

cd rm
cargo build
cd ..
cp "rm/target/debug/rm" "bin/rm"

cd sort
cargo build
cd ..
cp "sort/target/debug/sort" "bin/sort"

cd touch
cargo build
cd ..
cp "touch/target/debug/touch" "bin/touch"

cd wc
cargo build
cd ..
cp "wc/target/debug/wc" "bin/wc"
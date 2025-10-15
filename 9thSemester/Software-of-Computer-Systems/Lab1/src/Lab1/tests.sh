cargo build --package Lab1

TESTS_AMOUNT=18
for i in {1..$TESTS_AMOUNT}
do
    echo "Test $i"
    ./target/debug/Lab1 -p -c ./Code/Tests/test$i.xai
    echo "\n\n"
done
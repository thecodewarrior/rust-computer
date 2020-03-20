#include "cpudef/cpudef.asm"

findprimes:
    frame 1 # a frame with one variable
    .maybe_prime = 0 # an assembly constant

    # start the prime with a 1. This will be incremented to 3 in a moment
    push 1;
    store .maybe_prime 

.not_prime:
    # step forward to the next candidate, skipping even numbers
    load .maybe_prime
    push 2
    uadd 
    store .maybe_prime

    push 2 # the first number to test against
.test_loop: 
        dup                # the stack is now [test, test]
        load .maybe_prime  # now [test, test, maybe_prime]
        swap               # now [test, maybe_prime, test]
        urem               # get the remainder
        ujmp_ez .divisible # if the remainder is zero, we're halfway there

        push 1             # stack is now [test, 1]
        uadd               # increment the test value
        jmp .test_loop     # try again with the new test value
.divisible:
        load .maybe_prime
        # if the test isn't equal to the prime, that means we found 
        # a factor before we got to `n % n` == 0, so it's not a prime
        ujmp_ne .not_prime

        # if we survived that gauntlet, we're a prime!
        load .maybe_prime
        dup
        dup
        dup
        dup
        dup
        dup
        pop
        pop
        pop
        pop
        pop
        pop
        pop
        jmp .not_prime # okay enough partying, time to try the next number
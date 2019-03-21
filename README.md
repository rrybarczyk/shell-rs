# shelly
Assignment 1 from the [CS140 course](https://cs140e.sergio.bz/assignments/1-shell/).

## Phase 1: `ferris-wheel` directory
- [x] `compile-fail` - modify programs to fail
- [x] `compile-pass` - modify programs to pass
- [x] `run-pass` - modify programs to pass assertions
- [ ] `questions` - answer Phase 0 questions

## Phase 2: Oxidation
- [x] Subphase A: `stack-vec`
- [ ] Subphase B: `volatile`
- [x] Subphase C: `xmodem` 
- [x] Subphase D: `ttywrite`

### Subphase D: ttywrite Testing
In a new shell run:

```
$ cd ttywrite
$ source test.sh
```

The output should be:
``
Compiling project with 'cargo build'...
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
Opening PTYs...
Running test 1/10.
wrote 8 bytes to input
Running test 2/10.
wrote 116 bytes to input
Running test 3/10.
wrote 224 bytes to input
Running test 4/10.
wrote 275 bytes to input
Running test 5/10.
wrote 383 bytes to input
Running test 6/10.
wrote 491 bytes to input
Running test 7/10.
wrote 30 bytes to input
Running test 8/10.
wrote 138 bytes to input
Running test 9/10.
wrote 246 bytes to input
Running test 10/10.
wrote 296 bytes to input
SUCCESS
Saving session...[1]+  Exit 143                socat -u ${PARAMS},link=input ${PARAMS},link=output

...copying shared history...
...saving history...truncating history files...
...completed.

[Process completed]
```

## Phase 3: *Not* a Seashell
- [ ] Subphase A: Getting Started
- [ ] Subphase B: System Timer
- [ ] Subphase C: GPIO
- [ ] Subphase D: UART
- [ ] Subphase E: The Shell
     
     
## Phase 4: Boot 'em Up

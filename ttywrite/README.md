# ttywrite
Subphase D of Assignment 1 from the [CS140 course](https://cs140e.sergio.bz/assignments/1-shell/).

Write to TTY using the XMODEM protocol by default.

```
USAGE:
    ttywrite [FLAGS] [OPTIONS] <tty_path>

FLAGS:
    -h, --help       Prints help information
    -r, --raw        Disable XMODEM
    -V, --version    Prints version information

OPTIONS:
    -b, --baud <baud_rate>               Set baud rate [default: 115200]
    -w, --width <char_width>             Set data character width in bits [default: 8]
    -f, --flow-control <flow_control>    Enable flow control ('hardware' or 'software') [default: none]
    -i <input>                           Input file (defaults to stdin if not set)
    -s, --stop-bits <stop_bits>          Set number of stop bits [default: 1]
    -t, --timeout <timeout>              Set timeout in seconds [default: 10]

ARGS:
    <tty_path>    Path to TTY device
```

## Testing
In a new shell:

```
$ cd ttywrite
$ source test.sh
```

The output should be:
```
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

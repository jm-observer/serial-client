# serial-client
a common serial client

 ```
./serial-client /dev/ttyCH9344USB0  getTxQueueCount str -b 115200 -e rn
./serial-client /dev/ttyS1 01040000000271cb -p even
 ```
 ```
Usage: serial-client [OPTIONS] <PATH> <DATA>

Arguments:
  <PATH>
          

  <DATA>
          

Options:
  -b <BAUD_RATE>
          [default: 9600]

  -d <DATA_BITS>
          [default: eight]

          Possible values:
          - five:  5 bits per character
          - six:   6 bits per character
          - seven: 7 bits per character
          - eight: 8 bits per character

  -p <PARITY>
          [default: none]

          Possible values:
          - none: No parity bit
          - odd:  Parity bit sets odd number of 1 bits
          - even: Parity bit sets even number of 1 bits

  -s <STOP_BITS>
          [default: one]

          Possible values:
          - one: One stop bit
          - two: Two stop bits

  -f <FLOW_CONTROL>
          [default: none]

          Possible values:
          - none:     No flow control
          - software: Flow control using XON/XOFF bytes
          - hardware: Flow control using RTS/CTS signals

  -t <TIMEOUT>
          [default: 500]

  -e <ENDING>
          [default: none]

          Possible values:
          - none: not ending to append data
          - r:    \r
          - rn:   \r\n

  -l <LOG>
          [default: info]
          [possible values: debug, info]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
          ```
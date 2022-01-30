# compact-encoding spec


(uintX) = [byteN, ..., byte2, byte1, byte0]

least significant bit (LSB) for little endian = byte0

| Type              | Encoding                                                       | Implemented | Tested |
|:------------------|:---------------------------------------------------------------|-------------|--------|
| none              |                                                                | [ ]         | [ ]    |
| bool              |                                                                | [ ]         | [ ]    |
|                   |                                                                |             |        |
| uint              | see dedicated uintX                                            | [ ]         | [ ]    |
| uint8             | [0xFC, byte0] (byte <= 0xFC)                                   | [X]         | [ ]    |
| uint16            | [0xFD, byte0, byte1]                                           | [X]         | [ ]    |
| uint24            | available in JS, but not called automatically                  | [ ]         | [ ]    |
| uint32            | [0xFE, byte0, byte1, byte2, byte3]                             | [X]         | [ ]    |
| uint64            | [0xFF, byte0, byte1, byte2, byte3, byte4, byte5, byte6, byte7] | [x]         | [ ]    |
|                   |                                                                |             |        |
| int               | see dedicated intX                                             | [ ]         | [ ]    |
| int8              |                                                                | [ ]         | [ ]    |
| int16             |                                                                | [ ]         | [ ]    |
| int24             | available in JS, but not called automatically                  | [ ]         | [ ]    |
| int32             |                                                                | [ ]         | [ ]    |
| int64             |                                                                | [ ]         | [ ]    |
|                   |                                                                |             |        |
| float32           |                                                                | [ ]         | [ ]    |
| float64           |                                                                | [ ]         | [ ]    |
|                   |                                                                |             |        |
| buffer / uint8[]? |                                                                | [ ]         | [ ]    |
| uint32[]          |                                                                | [ ]         | [ ]    |
| array             |                                                                | [ ]         | [ ]    |
|                   |                                                                |             |        |
| raw               |                                                                | [ ]         | [ ]    |
|                   |                                                                |             |        |
| string            |                                                                | [ ]         | [ ]    |
|                   |                                                                |             |        |
| fixed32           |                                                                | [ ]         | [ ]    |
| fixed64           |                                                                | [ ]         | [ ]    |
|                   |                                                                |             |        |


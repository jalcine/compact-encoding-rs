// SPDX-License-Identifier: MIT
// compact-encoding-rs Authors: see AUTHORS.txt

#[cfg(test)]
mod tests {
    use crate::*;

    // mdn: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER
    const MAX_SAFE_INTEGER: u64 = (2 as u64).pow(53) - 1;

    #[test]
    fn test_uint() {
        let mut state = State::new();

        (42u8).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 1,
                buffer: None,
            }
        );

        (4200u16).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 4,
                buffer: None,
            }
        );

        MAX_SAFE_INTEGER.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 13,
                buffer: None,
            }
        );

        state.alloc();

        assert_eq!(42u8.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 1,
                end: 13,
                buffer: Some(vec![42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            }
        );

        assert_eq!(4200u16.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 4,
                end: 13,
                buffer: Some(vec![42, 0xFD, 104, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            }
        );
        assert_eq!(MAX_SAFE_INTEGER.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 13,
                end: 13,
                buffer: Some(vec![
                    42, 0xFD, 104, 16, 0xFF, 255, 255, 255, 255, 255, 255, 31, 0
                ]),
            }
        );

        state.start = 0;
        assert_eq!(u8::decode(&mut state), Ok(42));
        assert_eq!(state.start, 1);
        assert_eq!(state.end, 13);

        assert_eq!(u16::decode(&mut state), Ok(4200));
        assert_eq!(state.start, 4);
        assert_eq!(state.end, 13);

        assert_eq!(u64::decode(&mut state), Ok(MAX_SAFE_INTEGER));
        assert_eq!(state.start, 13);
        assert_eq!(state.end, 13);

        assert_eq!(state.start, state.end);

        assert_eq!(u8::decode(&mut state), Err(DecodeError::BufferTooSmall));
    }

    #[test]
    fn test_int() {
        let mut state = State::new();

        42i8.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 1,
                buffer: None,
            }
        );

        (-4200i16).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 4,
                buffer: None,
            }
        );

        state.alloc();

        assert_eq!(42i8.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 1,
                end: 4,
                buffer: Some(vec![84, 0, 0, 0]),
            }
        );

        assert_eq!((-4200i16).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 4,
                end: 4,
                buffer: Some(vec![84, 0xFD, 207, 32]),
            }
        );

        state.start = 0;
        assert_eq!(i8::decode(&mut state), Ok(42));
        assert_eq!(state.start, 1);
        assert_eq!(state.end, 4);

        assert_eq!(i16::decode(&mut state), Ok(-4200));
        assert_eq!(state.start, 4);
        assert_eq!(state.end, 4);

        assert_eq!(state.start, state.end);

        assert_eq!(i8::decode(&mut state), Err(DecodeError::BufferTooSmall));
    }

    #[test]
    fn test_float64() {
        let mut state = State::new();
        const NUM: f64 = 162.2377294;

        NUM.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 8,
                buffer: None,
            }
        );

        state.alloc();
        assert_eq!(
            state,
            State {
                start: 0,
                end: 8,
                buffer: Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            }
        );

        assert_eq!(NUM.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 8,
                end: 8,
                buffer: Some(vec![0x87, 0xC9, 0xAF, 0x7A, 0x9B, 0x47, 0x64, 0x40]),
            }
        );

        state.start = 0;
        assert_eq!(f64::decode(&mut state), Ok(NUM));
        assert_eq!(state.start, state.end);

        assert_eq!(f64::decode(&mut state), Err(DecodeError::BufferTooSmall));

        // alignment
        state.dealloc();

        0u8.pre_encode(&mut state);
        NUM.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 9,
                buffer: None,
            }
        );

        state.alloc();
        assert_eq!(
            state,
            State {
                start: 0,
                end: 9,
                buffer: Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            }
        );

        assert_eq!(0u8.encode(&mut state), Ok(()));
        assert_eq!(NUM.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 9,
                end: 9,
                buffer: Some(vec![0, 0x87, 0xC9, 0xAF, 0x7A, 0x9B, 0x47, 0x64, 0x40]),
            }
        );

        state.start = 0;
        assert_eq!(u8::decode(&mut state), Ok(0));
        assert_eq!(f64::decode(&mut state), Ok(NUM));
        assert_eq!(state.start, state.end);

        assert_eq!(f64::decode(&mut state), Err(DecodeError::BufferTooSmall));

        // subarray (replace buffer?)
        // TODO: check what this test is about and why it is needed
        //       would require state.buffer to point to a buffer instead of buffer being a member
        // let buffer = vec![0; 10];
        // state.buffer = &buffer[1..];
        // assert_eq!(
        //     state,
        //     State {
        //         start: 0,
        //         end: 9,
        //         buffer: Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
        //     }
        // );

        // assert_eq!(0u8.encode(&mut state), Ok(()));
        // assert_eq!(NUM.encode(&mut state), Ok(()));
        // assert_eq!(
        //     state,
        //     State {
        //         start: 9,
        //         end: 9,
        //         buffer: Some(vec![0, 0x87, 0xC9, 0xAF, 0x7A, 0x9B, 0x47, 0x64, 0x40]),
        //     }
        // );

        // state.start = 0;
        // assert_eq!(u8::decode(&mut state), Ok(0));
        // assert_eq!(f64::decode(&mut state), Ok(NUM));
        // assert_eq!(state.start, state.end);

        // 0
        state.dealloc();
        NUM.pre_encode(&mut state);

        state.alloc();
        assert_eq!(0f64.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 8,
                end: 8,
                buffer: Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            }
        );

        state.start = 0;
        assert_eq!(f64::decode(&mut state), Ok(0.));
        assert_eq!(state.start, state.end);

        // infinity
        state.dealloc();
        f64::INFINITY.pre_encode(&mut state);

        state.alloc();
        assert_eq!(f64::INFINITY.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 8,
                end: 8,
                buffer: Some(vec![0, 0, 0, 0, 0, 0, 0xF0, 0x7F]),
            }
        );

        state.start = 0;
        assert_eq!(f64::decode(&mut state), Ok(f64::INFINITY));
        assert_eq!(state.start, state.end);

        // edge cases
        state.dealloc();
        (0.1 + 0.2).pre_encode(&mut state);

        state.alloc();
        assert_eq!((0.1 + 0.2).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 8,
                end: 8,
                buffer: Some(vec![0x34, 0x33, 0x33, 0x33, 0x33, 0x33, 0xD3, 0x3F]),
            }
        );

        state.start = 0;
        assert_eq!(f64::decode(&mut state), Ok(0.1 + 0.2));
        assert_eq!(state.start, state.end);
    }

    #[test]
    fn test_buffer() {
        let mut state = State::new();

        Some("hi".as_bytes()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 3,
                buffer: None,
            }
        );
        Some("hello".as_bytes()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 9,
                buffer: None,
            }
        );
        None.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 10,
                buffer: None,
            }
        );

        state.alloc();

        assert_eq!(Some("hi".as_bytes()).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 3,
                end: 10,
                buffer: Some(vec![
                    2, 'h' as u8, 'i' as u8, // "hi"
                    0, 0, 0, 0, 0, 0, // "hello"
                    0, // None
                ]),
            }
        );
        assert_eq!(Some("hello".as_bytes()).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 9,
                end: 10,
                buffer: Some(vec![
                    2, 'h' as u8, 'i' as u8, // "hi"
                    5, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, // "hello"
                    0,         // None
                ]),
            }
        );
        assert_eq!(None.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 10,
                end: 10,
                buffer: Some(vec![
                    2, 'h' as u8, 'i' as u8, // "hi"
                    5, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, // "hello"
                    0,         // None
                ]),
            }
        );

        state.start = 0;
        assert_eq!(
            Option::<Box<Vec<u8>>>::decode(&mut state),
            Ok(Some(Box::new(vec!['h' as u8, 'i' as u8])))
        );
        assert_eq!(
            Option::<Box<Vec<u8>>>::decode(&mut state),
            Ok(Some(Box::new(vec![
                'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8
            ])))
        );
        assert_eq!(Option::<Box<Vec<u8>>>::decode(&mut state), Ok(None));
        assert_eq!(state.start, state.end);
        assert_eq!(
            Option::<Box<Vec<u8>>>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
        // set a smaller buffer -> should throw an error
        state.buffer = Some(Vec::from(&state.buffer.expect("buffer must exist")[0..8]));
        state.start = 3;
        // element at index 3 is 5, which is interpreted as the encoded buffer length
        // however, the newly set buffer has only 4 elements left -> should throw an error
        assert_eq!(
            Option::<Box<Vec<u8>>>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
    }

    #[test]
    fn test_uint32array() {
        let mut state = State::new();

        let array1 = vec![1u32];
        let array2 = vec![42u32, 43u32];
        U32Array::VecRef(&array1).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 5,
                buffer: None,
            }
        );

        U32Array::Slice(array2.as_slice()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 14,
                buffer: None,
            }
        );

        state.alloc();

        assert_eq!(U32Array::VecRef(&array1).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 5,
                end: 14,
                buffer: Some(vec![
                    1, 1, 0, 0, 0, // [1]
                    0, 0, 0, 0, 0, 0, 0, 0, 0 // [42, 43]
                ]),
            }
        );

        assert_eq!(
            U32Array::Slice(array2.as_slice()).encode(&mut state),
            Ok(())
        );
        assert_eq!(
            state,
            State {
                start: 14,
                end: 14,
                buffer: Some(vec![
                    1, 1, 0, 0, 0, // [1]
                    2, 42, 0, 0, 0, 43, 0, 0, 0 // [42, 43]
                ]),
            }
        );

        state.start = 0;
        assert_eq!(U32Array::decode(&mut state), Ok(U32Array::Vec(vec![1])));
        assert_eq!(
            U32Array::decode(&mut state),
            Ok(U32Array::Vec(vec![42, 43]))
        );

        assert_eq!(state.start, state.end);
        assert_eq!(
            Vec::<u32>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
    }

    #[test]
    fn test_array() {
        let mut state = State::new();

        let array1 = [true, false, true];
        let vec1 = vec![false, false, true, true];

        array1.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 4,
                buffer: None,
            }
        );

        vec1.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 9,
                buffer: None,
            }
        );

        state.alloc();
        assert_eq!(array1.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 4,
                end: 9,
                buffer: Some(vec![
                    3, // array length
                    1, 0, 1, // array content
                    0, // vec length
                    0, 0, 0, 0, // vec content
                ]),
            }
        );

        assert_eq!(vec1.encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 9,
                end: 9,
                buffer: Some(vec![
                    3, // array length
                    1, 0, 1, // array content
                    4, // vec length
                    0, 0, 1, 1, // vec content
                ]),
            }
        );

        state.start = 0;
        assert_eq!(Vec::<bool>::decode(&mut state), Ok(vec![true, false, true]));
        assert_eq!(
            Vec::<bool>::decode(&mut state),
            Ok(vec![false, false, true, true])
        );
        assert_eq!(state.start, state.end);
        assert_eq!(
            Vec::<bool>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
    }

    #[test]
    fn test_string() {
        let mut state = State::new();
        let emoji_string = "🌾";
        let utf8_string = "høsten er fin";

        emoji_string.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 5,
                buffer: None,
            }
        );

        utf8_string.pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 20,
                buffer: None,
            }
        );

        state.alloc();

        assert_eq!((&emoji_string).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 5,
                end: 20,
                buffer: Some(
                    "\x04🌾\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                        .as_bytes()
                        .to_vec()
                ),
            }
        );

        assert_eq!((&utf8_string).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 20,
                end: 20,
                buffer: Some("\x04🌾\x0ehøsten er fin".as_bytes().to_vec()),
            }
        );

        state.start = 0;
        assert_eq!(String::decode(&mut state), Ok(emoji_string.into()));
        assert_eq!(String::decode(&mut state), Ok(utf8_string.into()));
        assert_eq!(state.start, state.end);

        assert_eq!(String::decode(&mut state), Err(DecodeError::BufferTooSmall));
    }

    #[test]
    fn test_raw() {
        let mut state = State::new();

        let buffer = "hi".as_bytes();

        Raw::Slice(buffer).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 2,
                buffer: None,
            }
        );

        state.alloc();
        assert_eq!(Raw::Slice(buffer).encode(&mut state), Ok(()));
        assert_eq!(
            state,
            State {
                start: 2,
                end: 2,
                buffer: Some(vec![
                    'h' as u8, 'i' as u8, // "hi"
                ]),
            }
        );

        state.start = 0;
        assert_eq!(Raw::decode(&mut state), Ok(Raw::Vec(buffer.into())));
        assert_eq!(state.start, state.end);
    }

    #[test]
    fn test_fixed32() {
        let mut state = State::new();

        let buffer_a = ['a' as u8; 32];
        let buffer_b = ['b' as u8; 32];

        Fixed(buffer_a.clone()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 32,
                buffer: None,
            }
        );

        Fixed(buffer_b.clone()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 64,
                buffer: None,
            }
        );

        state.alloc();
        assert_eq!(Fixed(buffer_a.clone()).encode(&mut state), Ok(()));
        let mut test_buffer_a = [0 as u8; 64];
        test_buffer_a[..32].copy_from_slice(&buffer_a);
        assert_eq!(
            state,
            State {
                start: 32,
                end: 64,
                buffer: Some(test_buffer_a.to_vec()),
            }
        );

        assert_eq!(Fixed(buffer_b.clone()).encode(&mut state), Ok(()));
        let mut test_buffer_b = [0 as u8; 64];
        test_buffer_b[..32].copy_from_slice(&buffer_a);
        test_buffer_b[32..].copy_from_slice(&buffer_b);
        assert_eq!(
            state,
            State {
                start: 64,
                end: 64,
                buffer: Some(test_buffer_b.to_vec()),
            }
        );

        state.start = 0;
        assert_eq!(Fixed::decode(&mut state), Ok(Fixed(buffer_a)));
        assert_eq!(Fixed::decode(&mut state), Ok(Fixed(buffer_b)));
        assert_eq!(state.start, state.end);
        assert_eq!(
            Fixed::<32>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
    }

    #[test]
    fn test_fixed64() {
        let mut state = State::new();

        let buffer_a = ['a' as u8; 64];
        let buffer_b = ['b' as u8; 64];

        Fixed(buffer_a.clone()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 64,
                buffer: None,
            }
        );

        Fixed(buffer_b.clone()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 128,
                buffer: None,
            }
        );

        state.alloc();
        assert_eq!(Fixed(buffer_a.clone()).encode(&mut state), Ok(()));
        let mut test_buffer_a = [0 as u8; 128];
        test_buffer_a[..64].copy_from_slice(&buffer_a);
        assert_eq!(
            state,
            State {
                start: 64,
                end: 128,
                buffer: Some(test_buffer_a.to_vec()),
            }
        );

        assert_eq!(Fixed(buffer_b.clone()).encode(&mut state), Ok(()));
        let mut test_buffer_b = [0 as u8; 128];
        test_buffer_b[..64].copy_from_slice(&buffer_a);
        test_buffer_b[64..].copy_from_slice(&buffer_b);
        assert_eq!(
            state,
            State {
                start: 128,
                end: 128,
                buffer: Some(test_buffer_b.to_vec()),
            }
        );

        state.start = 0;
        assert_eq!(Fixed::decode(&mut state), Ok(Fixed(buffer_a)));
        assert_eq!(Fixed::decode(&mut state), Ok(Fixed(buffer_b)));
        assert_eq!(state.start, state.end);
        assert_eq!(
            Fixed::<64>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
    }

    #[test]
    fn test_fixed() {
        let mut state = State::new();

        let buffer_a = ['a' as u8; 3];
        let buffer_b = ['b' as u8; 3];

        Fixed(buffer_a.clone()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 3,
                buffer: None,
            }
        );

        Fixed(buffer_b.clone()).pre_encode(&mut state);
        assert_eq!(
            state,
            State {
                start: 0,
                end: 6,
                buffer: None,
            }
        );

        state.alloc();
        assert_eq!(Fixed(buffer_a.clone()).encode(&mut state), Ok(()));
        let mut test_buffer_a = [0 as u8; 6];
        test_buffer_a[..3].copy_from_slice(&buffer_a);
        assert_eq!(
            state,
            State {
                start: 3,
                end: 6,
                buffer: Some(test_buffer_a.to_vec()),
            }
        );

        assert_eq!(Fixed(buffer_b.clone()).encode(&mut state), Ok(()));
        let mut test_buffer_b = [0 as u8; 6];
        test_buffer_b[..3].copy_from_slice(&buffer_a);
        test_buffer_b[3..].copy_from_slice(&buffer_b);
        assert_eq!(
            state,
            State {
                start: 6,
                end: 6,
                buffer: Some(test_buffer_b.to_vec()),
            }
        );

        state.start = 0;
        assert_eq!(Fixed::decode(&mut state), Ok(Fixed(buffer_a)));
        assert_eq!(Fixed::decode(&mut state), Ok(Fixed(buffer_b)));
        assert_eq!(state.start, state.end);
        assert_eq!(
            Fixed::<3>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
        state.start = 4;

        assert_eq!(
            Fixed::<3>::decode(&mut state),
            Err(DecodeError::BufferTooSmall)
        );
    }
}

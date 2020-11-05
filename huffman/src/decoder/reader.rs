use super::{DecoderError};

/// An object for decoding Huffman sequence back to the original form.
pub(crate) struct DecodeReader {
    /// The number of bits that the reader should read at a time.
    speed: usize,

    /// The ID of the last row in the translation matrix from where the
    /// decoding should continue.
    id: usize,

    /// Internal buffer of received bits.
    buf: usize,

    /// The number of bits stored in the internal buffer.
    buf_size: usize,

    /// The bit sequence of the last character. This is needed for validating
    /// how the sequence ends. If the padding is not valid, then the whole 
    /// sequence is invalid.
    tail: usize,

    /// The number of bits stored in the tail variable.
    tail_size: usize,
}

impl DecodeReader {
    /// Returns a new reader instance.
    pub fn new(speed: usize) -> Self {
        Self {
            speed,
            id: 0,
            buf: 0,
            buf_size: 0,
            tail: 0,
            tail_size: 0,
        }
    }

    /// Decodes the entiry buffer of bits (N-bits where N=speed). If leftovers
    /// are found in the sequence, some bits < speed will remain in the buffer.
    pub fn decode(&mut self, byte: u8, dst: &mut Vec<u8>) -> Result<(), DecoderError> {

        self.buf <<= 8; // make space for new chunk
        self.buf_size += 8;
        self.buf |= byte as usize; // apply new chunk

        loop {
            if self.buf_size < self.speed { // has chunks to process
                break;
            } else {
                self.decode_next(dst)?;
            }
        }

        Ok(())
    }

    /// When an application receives the last byte this method should be called
    /// to adjust the remaining bits in the internal buffer. When needed, the
    /// buffer size is extended to the remaining speed size so the last chunk
    /// can be processed by the `decode` method. The extended bits are treated
    /// as a buffer bits of value 1.
    pub fn finalize(&mut self, dst: &mut Vec<u8>) -> Result<(), DecoderError> {
        let shift_len = (self.buf_size as f64 / self.speed as f64).ceil() as usize * self.speed as usize - self.buf_size; // how much missing to chunk size
        
        self.buf <<= shift_len; // expand buffer to chunk size
        self.buf_size += shift_len;

        if self.buf_size >= self.speed { // has chunks to process
            if let Ok((_, _, leftover)) = self.find_target(self.buf) {
                if shift_len <= leftover as usize { // has another character
                    self.decode_next(dst)?;
                }
            }
        }

        self.buf >>= shift_len; // remove leftover
        self.buf_size -= shift_len;

        self.tail <<= self.buf_size; // append buffer to tail
        self.tail_size += self.buf_size;
        self.tail |= self.buf;
        self.buf = 0;
        self.buf_size = 0;

        if !vec![0, 1, 3, 7, 15, 31, 63, 127].iter().any(|p| *p == self.tail) { // validate padding
            return Err(DecoderError::InvalidInput);
        }

        self.tail = 0; // reset (make object reusable)
        self.tail_size = 0;

        Ok(())
    }

    /// Tries to decode the next chunk of N bits where N represents the speed.
    /// 
    /// This function expects that the `buf_size` is grater or equal to 1. You
    /// should not call this function if this condition is not meet.
    fn decode_next(&mut self, dst: &mut Vec<u8>) -> Result<(), DecoderError> {
        let key = self.buf >> self.buf_size - self.speed;
        let (next_id, ascii, leftover) = self.find_target(key)?;

        self.buf -= key >> leftover << self.buf_size - self.speed + leftover as usize; // remove key from buffer
        self.buf_size -= self.speed - leftover as usize;

        self.tail <<= self.speed - leftover as usize; // append chunk to tail
        self.tail |= key >> leftover;
        self.tail_size += self.speed - leftover as usize;

        if let Some(ascii) = ascii {
            self.id = 0;
            self.tail = 0;
            self.tail_size = 0;
            if ascii < 256 { // valid character
                dst.push(ascii as u8);
                Ok(())
            } else {
                Err(DecoderError::InvalidInput)
            }
        } else if let Some(next_id) = next_id { // transition
            self.id = next_id as usize;
            Ok(())
        } else {
            Err(DecoderError::InvalidInput)
        }
    }    

    /// Returns the translation target tuple based on reader speed.
    fn find_target(&self, key: usize) -> Result<(Option<u8>, Option<u16>, u8), DecoderError> {
        match self.speed {
            2 => {
                match crate::decoder::table2::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            3 => {
                match crate::decoder::table3::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            4 => {
                match crate::decoder::table4::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            5 => {
                match crate::decoder::table5::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            _ => {
                match crate::decoder::table1::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            }
        }
    }
}

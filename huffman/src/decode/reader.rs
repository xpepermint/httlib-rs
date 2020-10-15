use crate::DecoderError;

/// * dekoder ima kapaciteto 128
pub(crate) struct DecodeReader {
    speed: usize,
    id: usize,
    buf: usize,
    buf_size: usize,
}

impl DecodeReader {
    pub const MAX_BUFFER_SIZE: usize = 32;

    pub fn new(speed: u8) -> Self {
        Self {
            speed: speed as usize,
            id: 0,
            buf: 0,
            buf_size: 0,
        }
    }

    /// Vedno sprejemamo byte!!!!
    pub fn write(&mut self, src: u8) -> Result<(), DecoderError> {
        if self.buf_size + 8 > Self::MAX_BUFFER_SIZE {
            return Err(DecoderError::BufferOverflow);
        }

        self.buf <<= 8; // make space for new chunk
        self.buf_size += 8;
        self.buf |= src as usize; // apply new chunk

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), DecoderError> {
        let shift_len = (self.buf_size as f64 / self.speed as f64).ceil() as usize * self.speed as usize - self.buf_size;

        self.buf <<= shift_len;
        self.buf |= 2u32.pow(shift_len as u32) as usize - 1;
        self.buf_size += shift_len;

        Ok(())
    }

    // Decodes 4 bits
    pub fn decode(&mut self, dst: &mut Vec<u8>) -> Result<(), DecoderError> {
        loop {
            if self.buf_size < self.speed {
                break;
            } else if let Some(byte) = self.decode_next()? {
                dst.push(byte);
            }
        }
        Ok(())
    }

    /// predvideva da je buffer_len vsaj velikosti 1 chunk, sicer ne klicat.
    fn decode_next(&mut self) -> Result<Option<u8>, DecoderError> {
        let key = self.buf >> self.buf_size - self.speed;
        let (next_id, ascii, leftover) = self.find_target(key)?;

        self.buf -= key >> leftover << self.buf_size - self.speed + leftover; // remove key from buffer
        self.buf_size -= self.speed - leftover;

        if let Some(ascii) = ascii {
            self.id = 0;
            Ok(Some(ascii as u8))
        } else if let Some(next_id) = next_id {
            self.id = next_id;
            Ok(None)
        } else {
           Err(DecoderError::InvalidInput)
        }
    }    

    /// Uposteva speed za izbor translation tabele
    fn find_target(&self, key: usize) -> Result<(Option<usize>, Option<usize>, usize), DecoderError> {
        match self.speed {
            #[cfg(feature = "decode1")]
            1 => {
                match crate::decode::table1::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            #[cfg(feature = "decode2")]
            2 => {
                match crate::decode::table2::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            #[cfg(feature = "decode3")]
            3 => {
                match crate::decode::table3::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            #[cfg(feature = "decode4")]
            4 => {
                match crate::decode::table4::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            #[cfg(feature = "decode5")]
            5 => {
                match crate::decode::table5::DECODE_TABLE.get(self.id) {
                    Some(transitions) => match transitions.get(key as usize) {
                        Some(target) => Ok(*target),
                        None => Err(DecoderError::InvalidInput),
                    },
                    None => Err(DecoderError::InvalidInput),
                }
            },
            _ => {
                Err(DecoderError::InvalidSpeed)
            }
        }
    }
}

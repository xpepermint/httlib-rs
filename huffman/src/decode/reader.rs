use crate::DecoderError;

/// * dekoder ima kapaciteto 128
pub struct DecodeReader {
    speed: u8,
    id: usize,
    buf: usize,
    buf_size: usize,
}

impl DecodeReader {
    pub fn new(speed: u8) -> Self {
        Self {
            speed,
            id: 0,
            buf: 0,
            buf_size: 0,
        }
    }

    pub fn write(&mut self, src: usize) -> Result<(), DecoderError> {
        // Handle DecoderError::BufferOverflow
        // Handle DecoderError::InvalidChunkSize
        let speed = self.speed as usize;

        println!("  src:            {:0>1$}", format!("{:b}", src), 8);

        self.buf <<= speed; // make space for new chunk
        self.buf_size += speed;
        self.buf |= src; // apply new chunk
        println!("   self.buf:      {:0>1$}", format!("{:b}", self.buf), 8);

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), DecoderError> {
        let shift_len = (self.buf_size as f64 / self.speed as f64).ceil() as usize * self.speed as usize - self.buf_size;
        self.buf <<= shift_len;
        self.buf_size += shift_len;
        Ok(())
    }

    // Decodes 4 bits
    pub fn decode(&mut self) -> Result<Option<u8>, DecoderError> {
        if self.buf_size < self.speed as usize {
            return Ok(None);
        }

        let speed = self.speed as usize;

        let transitions = match crate::data::decoder4::DECODE_TABLE.get(self.id) {
            Some(record) => record,
            None => return Err(DecoderError::InvalidHuffmanCode),
        };
        // println!("---------");
        // println!("  id:             {:?}", self.id);

        let key = self.buf >> self.buf_size - speed;
        // println!("   key:           {:0>1$}", format!("{:b}", key), 8);
        // println!("   key:           {:?}", key);

        let (next_id, ascii, leftover) = match transitions.get(key as usize) {
            Some(next_state) => next_state,
            None => return Err(DecoderError::InvalidHuffmanCode),
        };

        let key = self.buf >> self.buf_size - speed + leftover << leftover;
        self.buf -= key >> leftover << self.buf_size - speed + leftover; // remove key from buffer
        self.buf_size -= speed - *leftover;
        // println!("   self.buf:      {:0>1$}", format!("{:b}", self.buf), 8);
        // println!("   self.buf_size  {:?}", self.buf_size);
        // println!("   leftover:      {:?}", leftover);
        // println!("   next_id:       {:?}", next_id);

        if let Some(ascii) = ascii {
            // println!("  END ======> {:?}", ascii);
            self.id = 0;
            Ok(Some(*ascii as u8))
        } else if let Some(next_id) = next_id {
            self.id = *next_id;
            Ok(None)
        } else {
           Err(DecoderError::InvalidHuffmanCode)
        }
    }
}

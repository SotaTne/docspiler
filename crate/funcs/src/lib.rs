mod lines;
mod text_size;

pub struct UnicodeBufferScratch {
    buffer: rustybuzz::UnicodeBuffer,
}

impl UnicodeBufferScratch {
    pub fn new() -> Self {
        Self {
            buffer: rustybuzz::UnicodeBuffer::new(),
        }
    }

    pub fn steal_buffer(&mut self) -> rustybuzz::UnicodeBuffer {
        std::mem::replace(&mut self.buffer, rustybuzz::UnicodeBuffer::new())
    }
}

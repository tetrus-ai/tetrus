use super::PlayAreaSize;

impl PlayAreaSize {
    pub fn with_width_and_height(width: u8, height: u8) -> Self {
        PlayAreaSize { width, height }
    }
}

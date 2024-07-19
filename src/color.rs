#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn sum(&self, other: &Color) -> Color {
        let r = self.r.saturating_add(other.r);
        let g = self.g.saturating_add(other.g);
        let b = self.b.saturating_add(other.b);
        Color { r, g, b }
    }

    pub fn multiply(&self, constant: u8) -> Color {
        let r = (self.r as u32 * constant as u32 / 255) as u8;
        let g = (self.g as u32 * constant as u32 / 255) as u8;
        let b = (self.b as u32 * constant as u32 / 255) as u8;
        Color { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let color = Color::new(100, 200, 255);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 200);
        assert_eq!(color.b, 255); 
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex(0xFF00FF);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(100, 200, 150);
        assert_eq!(color.to_hex(), 0x64C896);
    }

    #[test]
    fn test_sum() {
        let color1 = Color::new(100, 150, 200);
        let color2 = Color::new(50, 100, 150);
        let result = color1.sum(&color2);
        assert_eq!(result.r, 150);
        assert_eq!(result.g, 250);
        assert_eq!(result.b, 255); 
    }

    #[test]
    fn test_multiply() {
        let color = Color::new(100, 150, 200);
        let result = color.multiply(128);
       
        assert_eq!(result.r, 50);
        assert_eq!(result.g, 75);
        assert_eq!(result.b, 100);
    }
}
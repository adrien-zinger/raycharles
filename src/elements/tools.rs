pub fn map_value(s: u32, a1: u32, a2: u32, b1: u32, b2: u32) -> u32 {
  b1 + ((s - a1) * (b2 - b1) / (a2 - a1)) 
}

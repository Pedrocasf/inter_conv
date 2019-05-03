pub fn u8_to_u32(val:&[u8])->u32{
    let b0 = val[0] as u32;
    let b1 = val[1] as u32;
    let b2 = val[2] as u32;
    let b3 = val[3] as u32;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | (b3 << 0) 
}
pub fn u8_to_u16(val:&[u8])->u16{
    let b0 = val[0] as u16;
    let b1 = val[1] as u16;
    (b0 << 8) | (b1 << 0) 
}
pub fn u32_to_u8(x:u32) -> [u8;4] {
    let b0 : u8 = ((x >> 24) & 0xff) as u8;
    let b1 : u8 = ((x >> 16) & 0xff) as u8;
    let b2 : u8 = ((x >> 8) & 0xff) as u8;
    let b3 : u8 = (x & 0xff) as u8;
    [b0, b1, b2, b3]
}
pub fn u16_to_u8(x:u16) -> [u8;2] {
    let b0 : u8 = ((x >> 8) & 0xff) as u8;
    let b1 : u8 = (x & 0xff) as u8;
    [b0, b1]
}
pub fn u16_to_u32(val:&[u16])->u32{
    let b0 = val[0] as u32;
    let b1 = val[1] as u32;
    (b0 <<16) | (b1 << 0) 
}
pub fn u32_to_u16(x:u32) -> [u32;2] {
    let b0 : u32 = ((x >> 16) & 0xffff) as u32;
    let b1 : u32 = (x & 0xffff) as u32;
    [b0, b1]
}

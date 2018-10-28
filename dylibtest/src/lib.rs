pub struct DataBufferTest {
    pub x : i32
}
// no_mangle is mandatory for dynamic lib loading!
#[no_mangle]
pub fn hello_world(data : &mut DataBufferTest) {
    println!("tyti");
}

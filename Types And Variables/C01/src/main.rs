//By Default Rust Variables are immutable
use std::mem;
fn main(){
    let a:u8 = 123; // 8 Bits Unsigned(u stands for unsigned 0 or positive and range is :0 to 255) //They are Immutable
    let b:i8 = 123; //8 Bits Signed (i  stands for signed integer and range is: -127 to 128 ) //They are Immutable
    println!("The Value of Unsigned A is:{} and the Value of Scigned B is:{}",a,b);
    //Mutable
    let mut c:i8 = 0; //mut means mutable
    println!("C is :{}",c);
    c=42;
    println!("C value is :{}",c);
    let mut d = 12345678; //i32 -32 bit signed variable
    println!("Value of D is:{} and size is :{} bytes",d,mem::size_of_val(&c));
    let z:isize = 123;//useful when we need the variable whose size is same as size of memory of system you are running
    // the isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit 
    //architecture and 32 bits if you’re on a 32-bit architecture
    let size_of_z = mem::size_of_val(&z); 
    println!("Z:{} & size of Z :{} & {}-bit-OS System",z,size_of_z,size_of_z*8);
    let e = 'x';
    println!("e :{} & size of e is:{}",e ,mem::size_of_val(&e));
    let f:f32 = 2.5;//double precision or 8 bytes or 64 bits(f64) 
    println!("f :{} & size of f is:{}",f ,mem::size_of_val(&f));
    let g:bool = true;
    println!("g :{} & size of g is:{}",g ,mem::size_of_val(&g));

}

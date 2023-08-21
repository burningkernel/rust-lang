static MAX: u32 = 0;

fn foo() {}

fn main() {
    let hello = "Hello world".to_string();
    let data = Box::new(1);
    let tmp = 10;

    println!("RODATA:\t\t\t{:p}", "hello world!");
    println!("DATA(static var):\t{:p}", &MAX);
    println!("TEXT(function):\t\t{:p}", foo as *const());
    println!("STACK(&hello):\t\t{:p}", &hello);
    println!("HEAP(&*hello):\t\t{:p}", &*hello);
    println!("STACK(&tmp):\t\t{:p}", &tmp);
    println!("HEAP(box impl Pointer):\t{:p} {:p}", data, &*data);
}

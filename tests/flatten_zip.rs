#[macro_use]
extern crate my_proc_macro;

#[test]
fn it_works() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9];
    let result = 2 + 2;
    for (i, (x, y, z)) in flatten_zip!(a, b, c).enumerate() {
        println!("{}, {}, {}", x, y, z);
        match i {
            0 => assert!(x == 1 && y == 4 && z == 7),
            1 => assert!(x == 2 && y == 5 && z == 8),
            _ => assert!(x == 3 && y == 6 && z == 9),
        }
    }
}

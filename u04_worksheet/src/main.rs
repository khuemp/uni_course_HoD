fn main() {
    let closure0 = |i : u16| i + 5;

    let v = vec![5, 7, 8, 19];
    let closure1 = |j : u16| j * v.iter().sum::<u16>();

    let v = vec![9, 8, 7];
    let closure2 = move |k: u16| {
        println!("Vec: {:#?}, k: {}", v, k);
        v
    };

    let mut v = vec!['R', 'u', 's'];
    let mut closure3 = |c| v.push(c);

    println!("{}", closure0(5));
    println!("{}", closure1(2));
    println!("{:#?}", closure2(5));
    //closure2(3);
    println!("{:#?}", closure3('t'));
    println!("{:#?}", v);
}
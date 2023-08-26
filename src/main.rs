// use std::collections::HashMap;

// fn main() {
//     let foo: Vec<_> = vec![1, 2, 3]
//         .iter()
//         .map(|x| x + 1)
//         .collect();

//     println!("{:?}", foo)
// }

// below shows what collect does, but does it manually
// fn main() {
// let data = vec![1, 2, 3];
// let mut foo = data.iter().map(|x| x + 1);
// let mut new_vector = vec![];

// while let Some(x) = foo.next() {
//     new_vector.push(x);
// }

// println!("{:?}", new_vector)
// let sum: i32 = vec![1, 2, 3].iter().sum();
// println!("{:?}", sum)

// let how_many_items: usize = vec![1, 2, 3].iter().skip(2).count();
// println!("{:?}", how_many_items)

// vec![1, 2, 5, 9, 4]
//     .iter()
//     .skip(2)
//     .take_while(|&&x| x > 4)
//     .for_each(|x| println!("{}", x));

// let map = HashMap::from([
//     ("foo", 1),
//     ("bar", 2),
//     ("baz", 3),
// ]);
// }

fn main() {
    // let file = std::fs::read_to_string("lines").unwrap();
    // file.lines()
    //     .enumerate()
    //     .filter(|(index, _)| index % 2 == 0)
    //     .for_each(|(_, line)| println!("{}", line))

    let file = std::fs::read_to_string("lines").unwrap();
    file.lines()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line))
        
}

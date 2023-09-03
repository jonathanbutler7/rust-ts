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

// fn main() {
//     // let file = std::fs::read_to_string("lines").unwrap();
//     // file.lines()
//     //     .enumerate()
//     //     .filter(|(index, _)| index % 2 == 0)
//     //     .for_each(|(_, line)| println!("{}", line))

//     let file = std::fs::read_to_string("lines").unwrap();
//     file.lines()
//         .enumerate()
//         .filter(|(index, _)| index % 2 == 0)
//         .skip(2)
//         .take(2)
//         .for_each(|(_, line)| println!("{}", line))

// }

// enum Color {
//     Red,
//     Green,
//     Blue,
//     Yellow,
// }

// // fn is_green(color: Color) -> bool {
// //     if color == Color::Green { true } else { false }
// // }
// impl Color {
//     fn is_green(&self) -> bool {
//         if let Color::Green = self {
//             return true;
//         }
//         return false;
//     }

//     fn is_green_parts(&self) -> bool {
//         match self {
//             Color::Red => {
//                 return false;
//             }
//             Color::Green => {
//                 return false;
//             }
//             Color::Blue => {
//                 return true;
//             }
//             Color::Yellow => {
//                 return true;
//             }
//         }
//     }
// }

// fn print_color(color: Color) {
//     match color {
//         Color::Red => println!("red"),
//         Color::Green => println!("green"),
//         Color::Blue => println!("blue"),
//         Color::Yellow => println!("yellow"),
//     }
// }

// fn main() {
//     let foo = Color::Green;
//     foo.is_green();
// }

// struct Custom {
//     age: usize,
//     name: String,
// }

// enum Item {
//     Number(usize),
//     String(String),
//     MyCustom(Custom),
// }

// fn append(items: &mut Vec<Item>) {
//     // items.push(Item::String("Hello".to_string()))
//     items.push(Item::String("Hello".into()))
// }

// fn main() {
//     let mut items: Vec<Item> = vec![];
//     append(&mut items);
// }

// fn multiply(num: Option<usize>) -> usize {
//     return num.unwrap_or(0) * 5;
// }

// fn main() {
//     let file_name = std::env::args().nth(1).expect("the file name to be passed in");
//     let file = std::fs::read_to_string(file_name).expect("unable to read file to string");
//     file.lines().for_each(|line| {
//         // let print:usize = line.parse();
//         if let Ok(value) = line.parse::<usize>() {
//             println!("{}", value)
//         } else {
//             println!("line not a number")
//         }
//     })
// }

// fn multiply(nums: Vec<usize>, index: usize) -> usize {
//     return nums.get(index).unwrap_or(&index) * 5;
// }

// #[derive(Debug)]
// struct Item {
//     count: usize,
// }

// fn add_one(item: &mut Item) {
//     item.count += 1;
// }

// fn main() {
//     let mut item = Item { count: 1 };
//     add_one(&mut item);
//     println!("{:?}", item);
// }
#[derive(Debug)]
struct Item {
    count: usize,
}
fn print_all(items: &Vec<Item>) {
    items.iter().for_each(|item| { println!("{:?}", item.count) })
    // for item in items {
    //     println!("{:?}", item);
    // }
}

fn main() {
    let mut items = vec![Item { count: 1 }];
    let first = items.first_mut();
    println!("{:?}", first);
    
    print_all(&items);
}

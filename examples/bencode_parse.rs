use serde_bencode::Value;

fn main() {
    let content = std::fs::read("./examples/example2.torrent").unwrap();
    let bencode: Value = serde_bencode::from_binary(&content).unwrap();

    println!("Bencode: {bencode:#?}");
}

// fn print_bencode(bencode: serde_bencode::Value) {
//     match bencode {
//         Value::Number(n) => print!("{n}"),
//         Value::String(s) => print!("\"{s}\""),
//         Value::List(list) => {
//             print!("[");
//             let len = list.len();
//             for (i, v) in list.into_iter().enumerate() {
//                 print_bencode(v);
//
//                 if i != len - 1 {
//                     print!(",");
//                 }
//             }
//             print!("]");
//         }
//         Value::Dictionary(dictionary) => {
//             print!("{{");
//             let len = dictionary.len();
//             let mut i = 0;
//             for (k, v) in dictionary {
//                 print!("\"{k}\":");
//                 print_bencode(v);
//
//                 if i != len - 1 {
//                     print!(",");
//                 }
//                 i += 1;
//             }
//             print!("}}");
//         }
//     }
// }

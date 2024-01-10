use std::collections::HashMap;

macro_rules! de_tests {
    ($($f:ident, $t:ty, $bencode:literal, $typed:expr),*) => {
        mod de {
            use super::*;
            $(
                #[test]
                pub fn $f() {
                    let expected: $t = $typed;
                    assert_eq!(expected, serde_bencode::from_binary::<$t>($bencode).unwrap());
                }
            )*
        }
    }
}

macro_rules! ser_tests {
    ($($f:ident, $t:ty, $bencode:literal, $typed:expr),*) => {
        mod ser {
            use super::*;
            $(
                #[test]
                pub fn $f() {
                    let typed: $t = $typed;
                    assert_eq!($bencode, &serde_bencode::to_binary(typed).unwrap()[..])
                }
            )*
        }
    }
}

macro_rules! tests {
    ($($f:ident : $t:ty => ($bencode:literal == $typed:expr));*) => {
        de_tests!($($f, $t, $bencode, $typed),*);
        ser_tests!($($f, $t, $bencode, $typed),*);
    }
}

tests! {
    test_number: i64 => (b"i-114e" == -114);
    test_string: String => (b"3:foo" == "foo".to_string());
    test_empty_list: Vec<String> => (b"le" == Vec::new());
    test_list: Vec<String> => (b"l4:spam4:eggse" == vec!["spam".to_string(), "eggs".to_string()]);
    test_tuple: (String, String) => (b"l4:spam4:eggse" == ("spam".to_string(), "eggs".to_string()));
    test_empty_dictionary: HashMap<String, i32> => (b"de" == HashMap::new());
    test_dictionary: HashMap<String, u8> => (b"d3:onei1e3:twoi2e5:threei3e4:fouri4ee" == {
        let mut map = HashMap::new();
        map.insert("one".to_string(), 1);
        map.insert("two".to_string(), 2);
        map.insert("three".to_string(), 3);
        map.insert("four".to_string(), 4);
        map
    });
    test_list_in_dictionary: _ => (b"d4:spaml1:a1:bee" == {
        let mut map = HashMap::new();
        map.insert("spam".to_string(), vec!["a".to_string(), "b".to_string()]);
        map
    });
    test_bytes: &[u8] => (b"4:asdf" == b"asdf");
    test_bytes_list: Vec<&[u8]> => (b"l4:teste" == vec![&b"test"[..]]);
    test_borrow_str: &str => (b"4:meta" == "meta")

    // test_dyn_number: _ => (b"i4e" == Number(4));
    // test_dyn_string: _ => (b"4:test" == String("test".to_string()));
    // test_dyn_bytes: _ => (b"4:l\xFFlw" == Bytes(b"l\xFFlw".to_vec()));
    // test_dyn_list: _ => (b"l3:foo3:bare" == List(vec![String("foo".to_string()), String("bar".to_string())]));
    // test_dyn_dictionary: _ => (b"d4:spaml1:a1:bee" == {
    //     let mut map = BTreeMap::new();
    //     map.insert(
    //         "spam".to_string(),
    //         List(vec![String("a".to_string()), String("b".to_string())]),
    //     );
    //     Dictionary(map)
    // })
}

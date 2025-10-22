use bincode::config::standard;
use bincode::serde::{encode_to_vec, decode_from_slice};
use std::collections::HashMap;
use libactionkv::ActionKV;

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE";

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = encode_to_vec(&a.index, standard()).unwrap();
    a.index = HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}
fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut a = ActionKV::open(path).expect("unable to open file");
    a.load().expect("unable to load data");

    match action {
        "get" => {
            let index_as_bytes = a.get(&INDEX_KEY).unwrap().unwrap();
            let (index, _len): (HashMap<ByteString, u64>, usize) = decode_from_slice(&index_as_bytes, standard()).unwrap();

            match index.get(key) {
                Some(&i) => {
                    let kv = a.get_at(i).unwrap();
                    println!("{:?}", kv.value)
                }
                None => eprintln!("{:?} not found", key),
            }
        },

        "delete" => a.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.insert(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.update(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }

        _ => eprintln!("{}", &USAGE),
    }
}

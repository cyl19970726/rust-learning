
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "rootC":["889556720033888098","9624980835188975094","17659386807097641759","6443351084880882127"],
            "root1":["1982432400487068821","15437996004651699851","10089762160193269030","263009234682912086"],
            "root2":["12364052984629808614","13066500727264825316","6321076066274078148","11393071566019822187"],
            "root3":["5467213570782227248","7664139402543995977","3725518918858623169","4359881039942919345"],
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    

    Ok(())
}

/// Combine the `input1.zkin.json` and `input1.zkin.json` into one(`out.zkin.json`)
// ../../target/release/eigen-zkit join_zkin --zkin1 0/fibonacci.recursive1/input.zkin.json  --zkin2 1/fibonacci.recursive1/input.zkin.json  --zkinout 0/fibonacci.recursive1/r1_input-rs.zkin.json
fn join_zkin(
    // stark_setup_file: &String,
    zkin1: &str,
    zkin2: &str,
)->Result<()> {
    // 1. load files.
    // porting from compressor12_exec.(input_file)
    // let stark_struct = load_json::<StarkStruct>(&stark_setup_file).unwrap();
    // let inputs_str = std::fs::read_to_string(zkin1).unwrap();
    let zkin1_map: BTreeMap<String, serde_json::Value> = serde_json::from_str(zkin1)?;

    let inputs_str = std::fs::read_to_string(zkin2).unwrap();
    let zkin2_map: BTreeMap<String, serde_json::Value> = serde_json::from_str(zkin2)?;

    let mut zkout_map = BTreeMap::new();

    for (k, v) in zkin1_map {
        zkout_map.insert(format!("a_{k}"), v);
    }

    println!("Please call {} at the number {}", zkout_map["a_name"], zkout_map["a_phones"][0]);

    // 3. save zkout to file
    // dump zkin file porting from stark_prove
    let input = serde_json::to_string(&zkout_map)?;
    println!("{}",input);
    // let mut file = File::create(&zkout)?;
    // write!(file, "{}", input).unwrap();
    Ok(())
}

#[test]
fn test_join_zkin() {
    untyped_example();

    // let data = r#"
    // {
    //     "name": "John Doe",
    //     "age": 43,
    //     "phones": [
    //         "+44 1234567",
    //         "+44 2345678"
    //     ],
    //     "rootC":["889556720033888098","9624980835188975094","17659386807097641759","6443351084880882127"],
    //     "root1":["1982432400487068821","15437996004651699851","10089762160193269030","263009234682912086"],
    //     "root2":["12364052984629808614","13066500727264825316","6321076066274078148","11393071566019822187"],
    //     "root3":["5467213570782227248","7664139402543995977","3725518918858623169","4359881039942919345"],
    // }"#;


    // let data1 = r#"
    // {
    //     "name": "John Doe",
    //     "age": 43,
    //     "phones": [
    //         "+44 1234567",
    //         "+44 2345678"
    //     ],
    //     "rootC":["889556720033888098","9624980835188975094","17659386807097641759","6443351084880882127"],
    //     "root1":["1982432400487068821","15437996004651699851","10089762160193269030","263009234682912086"],
    //     "root2":["12364052984629808614","13066500727264825316","6321076066274078148","11393071566019822187"],
    //     "root3":["5467213570782227248","7664139402543995977","3725518918858623169","4359881039942919345"],
    // }"#;
    // join_zkin(&data.to_string(), &data1.to_string());
}

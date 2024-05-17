use std::fs::{self, File};

use boon::{Compiler, Schemas};
use serde_json::Value;

fn main() {
    let mut schemas = Schemas::new(); // container for compiled schemas
    let mut compiler = Compiler::new();
    let sch_index = compiler
        .compile("report_schema.json", &mut schemas)
        .unwrap();
    let instance: Value = serde_json::from_reader(File::open("report.json").unwrap()).unwrap();
    let valid = schemas.validate(&instance, sch_index);
    println!("{valid:#?}");

    let report = fs::read_to_string("report.json").unwrap();
    // let value = serde_json::to_value(&report).unwrap();
    // let valid = schemas.validate(&value, sch_index);
    // println!("{valid:#?}");
    let report = serde_json::from_str::<report::report::Report>(&report).unwrap();
    println!("{report:#?}");

    // 1、通过json schema校验json文件
    // 2、反序列化json文件到struct，解析json文件
    // 3、根据Rpc或HTTP或SQL请求数据
    // 4、根据json文件解析返回的数据
    // 5、组装数据
    // 6、生成报表
}

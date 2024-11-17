use crate::opts::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, _format: OutputFormat) -> Result<()> {
    let mut ret = Vec::with_capacity(128);

    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;

        // 不使用迭代器的写法
        // for i in 0..headers.len() {
        //     let json_value = serde_json::json!({
        //         headers[i]: record[i],
        //     });
        //     ret.push(json_value);
        // }

        // 使用迭代器的写法，更清晰
        // headers.iter() -> 获得headers迭代器
        // record.iter() -> 获得record迭代器
        // collect::<Value>() -> 将两个迭代器合并成一个元组迭代器 [(header, record), ...]
        let val = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(val);
    }

    let content = match _format {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, &content)?;

    Ok(())
}

#[allow(dead_code)]
pub fn process_csv_to_player(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // 函数式编程
    // let records = reader
    //     .deserialize()
    //     .map(|record| record?)
    //     .collect::<Vec<Player>>();
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }

    fs::write(output, serde_json::to_string(&ret)?)?;

    Ok(())
}

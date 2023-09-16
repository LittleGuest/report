use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReqMethod {
    Rpc,
    Http,
    Sql,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Number,
    String,
    Array,
    Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
    name: String,
    required: Option<bool>,
    r#type: DataType,
    desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultMap {
    name: String,
    r#type: DataType,
    desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    id: String,
    desc: Option<String>,
    req_method: ReqMethod,
    configuration: Option<Configuration>,
    exec: String,
    params: Option<Vec<Param>>,
    result_map: Option<Vec<ResultMap>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    name: Option<String>,
    router: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    /// 饼图
    PieChart,
    /// 折线图
    LineChart,
    /// 柱状图
    BarChart,
    /// 散点图
    ScatterChart,
    /// 自定义
    CustomChart,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PieChart {
    // id: String,
    name: Option<String>,
    datas: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineChart {
    // id: String,
    name: Option<String>,
    datas: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarChart {
    // id: String,
    name: Option<String>,
    datas: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScatterChart {
    // id: String,
    name: Option<String>,
    datas: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomChart {
    // id: String,
    name: Option<String>,
    datas: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chart {
    id: String,
    r#type: ChartType,
    name: Option<String>,
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    datas: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableColumn {
    id: String,
    hidden: Option<bool>,
    export: Option<bool>,
    data: String,
    desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    id: String,
    name: Option<String>,
    page: Option<bool>,
    columns: Vec<TableColumn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    menu: Menu,
    charts: Vec<Chart>,
    tables: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    name: Option<String>,
    datasets: Vec<Dataset>,
    layout: Layout,
}

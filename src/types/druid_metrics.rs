use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DataSourceTypes {
    String(String),
    Vec(Vec<String>)
}

#[derive(Deserialize, Debug)]
pub struct DruidMetric {
    pub value: Option<f64>,
    pub metric: Option<String>,
    pub service: Option<String>,
    pub host: Option<String>,
    #[serde(alias = "dutyGroup")]
    pub duty_group: Option<String>,
    #[serde(alias = "dataSource", default = "default_data_string")]
    pub data_source: DataSourceTypes,
    #[serde(alias = "cpuName")]
    pub cpu_name: Option<String>,
    #[serde(alias = "cpuTime")]
    pub cpu_time: Option<String>,
}

fn default_data_string() -> DataSourceTypes {
    DataSourceTypes::String(String::default())
}
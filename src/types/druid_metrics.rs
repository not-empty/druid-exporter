use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DataSourceTypes {
    String(String),
    Vec(Vec<String>)
}

#[derive(Deserialize, Serialize, Debug)]
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
    pub id: Option<String>,
}

pub enum Returns<'a> {
    Float(&'a Option<f64>),
    String(&'a Option<String>),
    Types(&'a DataSourceTypes)
}

impl DruidMetric {
    pub fn get(&self, field: &str) -> Result<Returns, String> {
        match field {
            "value" => Ok(Returns::Float(&self.value)),
            "metric" => Ok(Returns::String(&self.metric)),
            "service" => Ok(Returns::String(&self.service)),
            "host" => Ok(Returns::String(&self.host)),
            "duty_group" => Ok(Returns::String(&self.duty_group)),
            "data_source" => Ok(Returns::Types(&self.data_source)),
            "cpu_name" => Ok(Returns::String(&self.cpu_name)),
            "cpu_time" => Ok(Returns::String(&self.cpu_time)),
            "id" => Ok(Returns::String(&self.id)),
            _ => Err(format!("Field {} not found", field))
        }
    }
}

fn default_data_string() -> DataSourceTypes {
    DataSourceTypes::String(String::default())
}
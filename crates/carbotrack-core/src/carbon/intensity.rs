use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CarbonIntensity {
    pub gco2e_per_kwh: f64,
    pub source: &'static str,
}

impl CarbonIntensity {
    pub const GLOBAL: Self = Self {
        gco2e_per_kwh: 475.0,
        source: "IEA 2024",
    };
    pub const CHINA: Self = Self {
        gco2e_per_kwh: 556.0,
        source: "中国电力企业联合会",
    };
    pub const USA: Self = Self {
        gco2e_per_kwh: 379.0,
        source: "EPA",
    };
    pub const EU: Self = Self {
        gco2e_per_kwh: 231.0,
        source: "EEA",
    };
    pub const FRANCE: Self = Self {
        gco2e_per_kwh: 56.0,
        source: "EEA",
    };
    pub const NORWAY: Self = Self {
        gco2e_per_kwh: 26.0,
        source: "EEA",
    };
    pub const AWS_US_EAST_1: Self = Self {
        gco2e_per_kwh: 379.0,
        source: "AWS 官方",
    };
    pub const ALIYUN_CN_EAST: Self = Self {
        gco2e_per_kwh: 556.0,
        source: "阿里云官方",
    };

    pub fn from_region(region: &str) -> Self {
        match region.to_ascii_lowercase().as_str() {
            "global" => Self::GLOBAL,
            "china" | "cn" | "aliyun-cn-east" | "cn-east" => Self::CHINA,
            "usa" | "us" => Self::USA,
            "eu" | "europe" => Self::EU,
            "france" | "fr" => Self::FRANCE,
            "norway" | "no" => Self::NORWAY,
            "us-east-1" => Self::AWS_US_EAST_1,
            _ => Self::GLOBAL,
        }
    }
}

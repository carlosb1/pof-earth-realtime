use log::trace;
use std::io::Read;

static HOST: &str = "https://geoserveis.icgc.cat/icc_mapesbase/wms/service";
static PARAMS: &[(&str, &str)] = &[
    ("REQUEST", "GetMap"),
    ("VERSION", "1.1.0"),
    ("SERVICE", "WMS"),
    ("SRS", "EPSG:25831"),
    ("LAYERS", "orto25c"),
    ("STYLES", ""),
    ("FORMAT", "image/jpeg"),
    ("BGCOLOR", "0xFFFFFF"),
    ("TRANSPARENT", "TRUE"),
    ("EXCEPTION", "INIMAGE"),
];

const MAX_SIZE_FILE: u64 = 10_000_000;

type QueryResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct GeoServiceQuery {
    pub host: String,
    pub config_params: Vec<(String, String)>,
    pub box_params: (f32, f32, f32, f32),
    pub width: u64,
    pub height: u64,
}

impl Default for GeoServiceQuery {
    fn default() -> Self {
        GeoServiceQuery {
            host: HOST.to_string(),
            config_params: PARAMS
                .iter()
                .map(|(x, y)| (x.to_string(), y.to_string()))
                .collect(),
            box_params: (0., 0., 0., 0.),
            width: 0,
            height: 0,
        }
    }
}
impl GeoServiceQuery {
    fn generate_extra_params(&mut self) -> Vec<(String, String)> {
        vec![
            (
                "BBOX".to_string(),
                format!(
                    "{:},{:},{:},{:}",
                    self.box_params.0, self.box_params.1, self.box_params.2, self.box_params.3
                )
                .to_string(),
            ),
            ("WIDTH".to_string(), self.width.to_string()),
            ("HEIGHT".to_string(), self.height.to_string()),
        ]
    }
    pub fn fetch_url(&mut self) -> QueryResult<Vec<u8>> {
        let mut config_params = self.config_params.clone();
        config_params.append(&mut self.generate_extra_params());

        let parsed_params: Vec<(&str, &str)> = config_params
            .iter()
            .map(|(str1, str2)| (str1.as_str(), str2.as_str()))
            .collect();

        let resp: ureq::Response = ureq::get(HOST).query_pairs(parsed_params).call()?;
        trace!("request url={:?}", resp.get_url());

        let mut bytes: Vec<u8> = vec![];
        resp.into_reader()
            .take(MAX_SIZE_FILE)
            .read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

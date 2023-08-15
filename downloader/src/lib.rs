use std::io::Cursor;

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
    pub async fn fetch_url(&mut self) -> QueryResult<Vec<u8>> {
        let mut cloned_params: Vec<(String, String)> = self.config_params.clone();
        cloned_params.append(&mut self.generate_extra_params());
        let url = reqwest::Url::parse_with_params(self.host.as_str(), cloned_params)?;
        let response = reqwest::get(url).await?;
        let mut content = Cursor::new(response.bytes().await?);

        let mut result: Vec<u8> = vec![];
        std::io::copy(&mut content, &mut result)?;
        Ok(result)
    }
}

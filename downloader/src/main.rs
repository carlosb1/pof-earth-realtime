use std::io::Cursor;


static HOST: &str = "https://geoserveis.icgc.cat/icc_mapesbase/wms/service";
static PARAMS: &[(&str, &str)]= &[("REQUEST", "GetMap"),
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
struct GeoServiceQuery {
    host: String,
    config_params: Vec<(String, String)>,
    box_params: (f32, f32, f32, f32),
    width: u64,
    height: u64
}

impl Default for GeoServiceQuery {
    fn default() -> Self {
        GeoServiceQuery{host: HOST.to_string(), config_params: PARAMS.iter().map(|(x, y)|
            (x.to_string(), y.to_string())
        ).collect(), box_params: (0., 0., 0., 0.), width: 0, height: 0 }

    }
}
impl GeoServiceQuery {
    fn generate_extra_params(&mut self) -> Vec<(String, String)> {
        vec![("BBOX".to_string(),
              format!("{:},{:},{:},{:}", self.box_params.0, self.box_params.1, self.box_params.2, self.box_params.3).to_string()),
             ("WIDTH".to_string(), self.width.to_string()), ("HEIGHT".to_string(), self.height.to_string())]
    }
    async fn fetch_url(&mut self, file_name: &str) -> QueryResult<()> {
        let mut cloned_params: Vec<(String, String)> = self.config_params.clone();
        cloned_params.append(&mut self.generate_extra_params());
        let url = reqwest::Url::parse_with_params(self.host.as_str(), cloned_params)?;
        let response = reqwest::get(url).await?;

        let mut file = std::fs::File::create(file_name)?;
        let mut content =  Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut geoservice = GeoServiceQuery{box_params: (290368.84, 4543236.42, 292203.28, 4545070.86), width: 520, height: 520,..Default::default()};
    geoservice.fetch_url("bin3.jpg").await.unwrap();
}
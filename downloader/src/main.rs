use pof_earth_downloader::GeoServiceQuery;
use std::io::Write;

fn main() {
    let mut geoservice = GeoServiceQuery {
        box_params: (290368.84, 4543236.42, 292203.28, 4545070.86),
        width: 520,
        height: 520,
        ..Default::default()
    };
    let file_bytes = geoservice.fetch_url().unwrap();
    let mut file = std::fs::File::create("bin5.jpg").unwrap();
    file.write_all(&file_bytes)
        .expect("It was not possible write the file.");
}

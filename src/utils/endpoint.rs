
pub fn level(lv: &str) -> String {
    let arcgis_endpoint = "https://services9.arcgis.com/N9p5hsImWXAccRNI/arcgis/rest/services/Nc2JKvYFoAEOFCG5JSI6/FeatureServer";
    match lv {
        "county"    => format!("{e}/{n}/query", e=arcgis_endpoint, n=1),
        "country"   => format!("{e}/{n}/query", e=arcgis_endpoint, n=2),
        "province"  => format!("{e}/{n}/query", e=arcgis_endpoint, n=3),
        "time"      => format!("{e}/{n}/query", e=arcgis_endpoint, n=4),
        _           => format!("{e}/{n}/query", e=arcgis_endpoint, n=4),
    }
}
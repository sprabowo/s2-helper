use wasm_bindgen::prelude::*;
use s2::cell::Cell;
use s2::cellid::CellID;
use s2::latlng::LatLng;
use s2::s1::Deg;

#[wasm_bindgen]
pub fn calculate(lat: f64, lng: f64, lvl: i64) -> String {
    let lat_lng = LatLng::new(Deg(lat).into(), Deg(lng).into());
    let ci = CellID::from(lat_lng).parent(lvl as u64);
    ci.0.to_string()
}

#[wasm_bindgen]
pub fn get_coordinates(cellid: i64) -> String {
    let ci = CellID(cellid as u64);
    if !ci.is_valid() {
        return "".to_string()
    }
    let points = Cell::from(ci).vertices();
    let coordinates: Vec<String> = points.iter().map(|p| format!("{},{}", p.longitude().deg(), p.latitude().deg())).collect();
    coordinates.join(";")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_success_calc() {
        assert_eq!("3344469575738589184", calculate(-6.228968465405475 as f64, 106.8071658857885 as f64, 13));
    }

    #[test]
    fn it_success_get_coordinates() {
        assert_eq!("106.80603880529831,-6.238318259447594;106.81783770824106,-6.237933179178703;106.81783770824107,-6.227712965171154;106.80603880529831,-6.228097424504746", get_coordinates(3344469575738589184));
    }

    #[test]
    fn it_failed_get_coordinates() {
        assert_eq!("", get_coordinates(0));
    }
}

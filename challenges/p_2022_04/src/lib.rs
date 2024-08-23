use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{http::{Request, Response}, http_component};
use spin_sdk::http::{IntoResponse, Params, Router};

#[derive(Serialize, Deserialize)]
struct Point {
    lat: f64,
    long: f64
}

#[derive(Serialize, Deserialize)]
struct InputData {
    d1: Point,
    d2: Point
}

#[derive(Serialize, Deserialize)]
struct OutputData {
    distance: f64
}



pub fn distance(start:Point, end:Point) -> f64 {

    // let kilometers: f64 = 6371.0;
    // let miles: f64 = 3960.0;
    let nautic_miles: f64 = 3440.065;

    let mut r: f64 = nautic_miles;

    let d_lat: f64 = (end.lat - start.lat).to_radians();
    let d_lon: f64 = (end.long - start.long).to_radians();
    let lat1: f64 = start.lat.to_radians();
    let lat2: f64 = end.lat.to_radians();

    let a: f64 = ((d_lat/2.0).sin()) * ((d_lat/2.0).sin()) + ((d_lon/2.0).sin()) * ((d_lon/2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * (a.sqrt().atan2((1.0-a).sqrt()));

    let calc =  r * c;

    // Round 0.1
    (calc * 10.0).round() / 10.0
}

#[http_component]
fn aos_calc_distance(req: Request) -> Response {

    println!("Calculate...");

    let data:InputData = serde_json::from_slice(req.body()).expect("Parsing error");

    let dist = distance(data.d1, data.d2);

    let result:OutputData = OutputData {distance: dist};

    let response = Response::builder()
        .body(serde_json::to_string(&result).expect("Error converting to json"))
        .header("Content-Type", "application/json")
        .status(200)
        .build();

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let d1 = Point {lat:0.0, long:0.0};
        let d2 = Point {lat:0.0, long:0.0};
        let result = distance(d1, d2);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_2() {
        let d1 = Point {lat:0.1, long:0.1};
        let d2 = Point {lat:0.3, long:0.3};
        let result = distance(d1, d2);

        assert_eq!(result, 17.0);
    }

    #[test]
    fn test_3() {
        let d1 = Point {lat:-0.1, long:-0.1};
        let d2 = Point {lat:0.3, long:0.3};
        let result = distance(d1, d2);

        assert_eq!(result, 34.0);
    }
}
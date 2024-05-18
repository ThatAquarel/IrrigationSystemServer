use std::fs::File;
use std::io::Read;

use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn run() {
    let zones = &["f", "bp", "bs"];

    let mut uri = String::new();
    let mut expected = String::new();
    let client: Client = Client::tracked(super::rocket()).unwrap();

    uri.push_str("/routine/run/?");
    for i in 0..zones.len() {
        uri.push_str(&format!(
            "routine[{}]zone={}&routine[{}]duration=10&",
            i, zones[i], i
        ));
        expected.push_str(&format!("Zone: {i}, Duration: 10\n"));
    }

    let response = client.get(uri.clone()).dispatch();
    assert_eq!(response.into_string(), Some(expected.clone()));

    uri.clear();
    expected.clear();

    uri.push_str("/routine/run/?");
    for i in 0..zones.len() {
        uri.push_str(&format!(
            "routine[{}]zone={}&routine[{}]duration=-1&",
            i, zones[i], i
        ));
    }
    expected.push_str("Malformed request");

    let response = client.get(uri.clone()).dispatch();
    assert_eq!(response.into_string(), Some(expected.clone()));

    uri.clear();
    expected.clear();

    uri.push_str("/routine/run/?");
    expected.push_str("Zones number is not three\n");
    for i in 0..(zones.len() - 1) {
        uri.push_str(&format!(
            "routine[{}]zone={}&routine[{}]duration=10&",
            i, zones[i], i
        ));
        expected.push_str(&format!("Zone: {i}, Duration: 10\n"));
    }
    expected.push_str("Missing zones in routine, bitmask: 3\n");

    let response = client.get(uri.clone()).dispatch();
    assert_eq!(response.into_string(), Some(expected.clone()));
}

#[test]
fn stop() {
    let client: Client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/routine/stop").dispatch();
    assert_eq!(
        response.into_string(),
        Some("Stopped current routine".into())
    );
}

#[track_caller]
fn test_query_file<T>(path: &str, file: T, status: Status)
where
    T: Into<Option<&'static str>>,
{
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get(path).dispatch();
    assert_eq!(response.status(), status);

    let body_data = response.into_bytes();
    if let Some(filename) = file.into() {
        let expected_data = read_file_content(filename);
        assert!(body_data.map_or(false, |s| s == expected_data));
    }
}

fn read_file_content(path: &str) -> Vec<u8> {
    let mut fp = File::open(&path).expect(&format!("Can't open {}", path));
    let mut file_content = vec![];

    fp.read_to_end(&mut file_content)
        .expect(&format!("Reading {} failed.", path));
    file_content
}

#[test]
fn test_index_html() {
    let client: Client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::SeeOther);

    test_query_file("/prod/", "static/index.html", Status::Ok);
}

#[test]
fn test_resources() {
    test_query_file("/prod/icon.png", "static/icon.png", Status::Ok);
    test_query_file("/prod/script.js", "static/script.js", Status::Ok);
    test_query_file("/prod/styles.css", "static/styles.css", Status::Ok);
}

#[test]
fn test_invalid_path() {
    test_query_file("/thou_shalt_not_exist", None, Status::NotFound);
    test_query_file("/thou/shalt/not/exist", None, Status::NotFound);
    test_query_file("/thou/shalt/not/exist?a=b&c=d", None, Status::NotFound);
}

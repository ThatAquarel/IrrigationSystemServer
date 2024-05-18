use rocket::local::blocking::Client;

#[test]
fn run() {
    let zones = &["f", "bp", "bs"];

    let mut uri = String::new();
    let mut expected = String::new();
    let client: Client = Client::tracked(super::rocket()).unwrap();

    uri.push_str("/run/?");
    for i in 0..zones.len() {
        uri.push_str(&format!("routine[{}]zone={}&routine[{}]duration=10&", i, zones[i], i));
        expected.push_str(&format!("Zone: {i}, Duration: 10\n"));
    }

    let response = client.get(uri.clone()).dispatch();
    assert_eq!(response.into_string(), Some(expected.clone()));
    
    uri.clear();
    expected.clear();

    uri.push_str("/run/?");
    for i in 0..zones.len() {
        uri.push_str(&format!("routine[{}]zone={}&routine[{}]duration=-1&", i, zones[i], i));
    }
    expected.push_str("Malformed request");

    let response = client.get(uri.clone()).dispatch();
    assert_eq!(response.into_string(), Some(expected.clone()));

    uri.clear();
    expected.clear();

    uri.push_str("/run/?");
    expected.push_str("Zones number is not three\n");
    for i in 0..(zones.len()-1) {
        uri.push_str(&format!("routine[{}]zone={}&routine[{}]duration=10&", i, zones[i], i));
        expected.push_str(&format!("Zone: {i}, Duration: 10\n"));
    }
    expected.push_str("Missing zones in routine, bitmask: 3\n");

    let response = client.get(uri.clone()).dispatch();
    assert_eq!(response.into_string(), Some(expected.clone()));
}

#[test]
fn stop() {
    let client: Client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/stop").dispatch();
    assert_eq!(response.into_string(), Some("Stopped current routine".into()));
}

use crate::grpc::server::grpc_server::{CoordinateFilter, Waypoint};
use regex::Regex;

/// TODO(R4): Security analysis of this dependency
///  Currently using this to read and convert waypoints file
///  (temporary R3 hack)
use dms_coordinates::{Bearing, DMS};

/// Converts a string in DMS format (e.g. 51° 30' 0.0" N) to a decimal degree
pub fn dms_to_double(dms: &str) -> Result<f64, ()> {
    let dms_fmt = r#"(?P<d>\d+)°\s*(?P<m>\d+)'\s*(?P<s>\d+\.\d+)"\s*(?P<b>N|S|E|W)"#;

    let re = Regex::new(dms_fmt).unwrap();
    let result = re.captures(dms);
    let Some(cap) = result else {
        region_error!("(dms_to_double) invalid DMS format.");
        region_debug!("(dms_to_double) dms: {}", dms);
        return Err(());
    };

    let (deg, min, sec, bearing) = (&cap["d"], &cap["m"], &cap["s"], &cap["b"]);

    let bearing = match bearing {
        "N" => Bearing::North,
        "S" => Bearing::South,
        "E" => Bearing::East,
        "W" => Bearing::West,
        _ => {
            region_error!("(dms_to_double) invalid bearing.");
            region_debug!("(dms_to_double) bearing: {}", bearing);
            return Err(());
        }
    };

    let wp = DMS::new(
        deg.parse::<i32>().unwrap(),
        min.parse::<i32>().unwrap(),
        sec.parse::<f64>().unwrap(),
        bearing,
    );

    Ok(wp.to_decimal_degrees())
}

#[derive(Debug, serde::Deserialize)]
struct Entry {
    identifier: String,
    latitude: String,
    longitude: String,
}

pub fn parse_waypoints_file(fname: &str, filter: CoordinateFilter) -> Result<Vec<Waypoint>, ()> {
    region_info!("(parse_waypoints_file) entry.");

    let mut waypoints = Vec::new();
    let Ok(mut rdr) = csv::Reader::from_path(fname) else {
        region_error!("(parse_waypoints_file) error opening waypoints file.");
        return Err(());
    };

    for result in rdr.deserialize::<Entry>() {
        println!("TEST: {:?}", result);
        let Ok(entry) = result else {
            region_error!("(parse_waypoints_file) error parsing waypoints file: {:?}", result.unwrap_err());
            return Err(());
        };

        let Ok(latitude) = dms_to_double(&entry.latitude) else {
            region_error!("(parse_waypoints_file) error parsing waypoints file.");
            return Err(());
        };

        let Ok(longitude) = dms_to_double(&entry.longitude) else {
            region_error!("(parse_waypoints_file) error parsing waypoints file.");
            return Err(());
        };

        let waypoint = Waypoint {
            identifier: entry.identifier,
            latitude,
            longitude,
        };

        //
        // Apply Filter
        //
        if let Some(min) = filter.min {
            if waypoint.latitude < min.latitude || waypoint.longitude < min.longitude {
                continue;
            }
        };

        if let Some(max) = filter.max {
            if waypoint.latitude > max.latitude || waypoint.longitude > max.longitude {
                continue;
            }
        };

        region_debug!(
            "Waypoint {{ identifier: {:?}.to_string(), latitude: {:.4}, longitude: {:.4} }},",
            waypoint.identifier,
            waypoint.latitude,
            waypoint.longitude
        );

        waypoints.push(waypoint);
    }

    Ok(waypoints)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::server::grpc_server::{Coordinate, CoordinateFilter};

    #[test]
    fn ut_dms_to_decimal() {
        let tolerance = 0.000001;
        let dms = "9°58'1.1\"N";
        let expected = 9.966972;
        let result = dms_to_double(dms).unwrap();
        assert!((result - expected).abs() < tolerance);

        let dms = "76°17' 1.1\"E";
        let expected = 76.283638;
        let result = dms_to_double(dms).unwrap();
        assert!((result - expected).abs() < tolerance);

        let dms = "51° 30'0.0\" W";
        let expected = -51.5;
        let result = dms_to_double(dms).unwrap();
        assert!((result - expected).abs() < tolerance);

        let dms = "8° 53'10.0\"S";
        let expected = -8.886111;
        let result = dms_to_double(dms).unwrap();
        assert!((result - expected).abs() < tolerance);
    }

    #[test]
    fn ut_parse_waypoints_file() {
        let fname = "/tmp/waypoints.csv";
        let waypoints = vec![
            (
                "JFK",
                r#"40°38'0.0"N"#,
                r#"73°47'0.0"W"#,
                40.633333,
                -73.783333,
            ),
            (
                "LAX",
                r#"33°56'0.0"N"#,
                r#"118°24'0.0"W"#,
                33.933333,
                -118.4,
            ),
            (
                "SFO",
                r#"37°37'0.0"N"#,
                r#"122°23'0.0"W"#,
                37.616667,
                -122.383333,
            ),
            (
                "ORD",
                r#"41°59'0.0"N"#,
                r#"87°53'0.0"W"#,
                41.983333,
                -87.883333,
            ),
        ];

        let mut data = "identifier,latitude,longitude\n".to_string();
        for w in &waypoints {
            data.push_str(&format!("{},{},{}\n", w.0, w.1, w.2));
        }
        let _ = std::fs::write(fname, data).unwrap();

        // Test no filter
        let results = parse_waypoints_file(
            fname,
            CoordinateFilter {
                min: None,
                max: None,
            },
        )
        .unwrap();
        assert_eq!(results.len(), waypoints.len());

        // Test Filter
        let filter = CoordinateFilter {
            min: Some(Coordinate {
                latitude: 34.0,
                longitude: -120.0,
            }),
            max: Some(Coordinate {
                latitude: 41.0,
                longitude: 0.0,
            }),
        };

        let results = parse_waypoints_file(fname, filter).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].identifier == "JFK");
    }

    #[test]
    fn ut_parse_waypoints_file_invalid() {
        assert!(parse_waypoints_file(
            "dne.csv",
            CoordinateFilter {
                min: None,
                max: None,
            }
        )
        .is_err());

        let fname = "/tmp/waypoints_invalid.csv";
        let mut data = "identifier,latitude,longitude\n".to_string();
        // improperly formatted coordinates
        data.push_str("JFK,40°38'0.0N,73°470.0\"W");
        let _ = std::fs::write(fname, data).unwrap();
        assert!(parse_waypoints_file(
            fname,
            CoordinateFilter {
                min: None,
                max: None,
            }
        )
        .is_err());
    }
}

use crate::grpc::server::grpc_server::{CoordinateFilter, Waypoint};
use regex::Regex;

/// TODO(R4): Security analysis of this dependency
///  Currently using this to read and convert waypoints file
///  (temporary R3 hack)
use dms_coordinates::{Bearing, DMS};

/// Converts a string in DMS format (e.g. 51° 30' 0.0" N) to a decimal degree
pub fn dms_to_double(dms: &str) -> Result<f64, ()> {
    let dms_fmt = r#"(?P<d>\d+)° (?P<m>\d+)' (?P<s>\d+\.\d+)" (?P<b>N|S|E|W)"#;

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
    let mut rdr = csv::Reader::from_path(fname).unwrap();

    for result in rdr.deserialize::<Entry>() {
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

        println!(
            "Waypoint {{ identifier: {:?}.to_string(), latitude: {:.4}, longitude: {:.4} }},",
            waypoint.identifier, waypoint.latitude, waypoint.longitude
        );

        waypoints.push(waypoint);
    }

    Ok(waypoints)
}

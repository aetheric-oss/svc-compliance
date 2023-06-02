//! Region utility functions

use regex::Regex;

/// TODO(R4): Security analysis of this dependency
///  Currently using this to read and convert waypoints file
///  (temporary R3 hack)
use dms_coordinates::{Bearing, DMS};
use svc_gis_client_grpc::Coordinates;

/// Filter for coordinates
#[derive(Debug, Copy, Clone)]
pub struct CoordinateFilter {
    min: Option<Coordinates>,
    max: Option<Coordinates>,
}

/// Custom Error type for dms functions
#[derive(Debug, Clone, Copy)]
pub enum DmsError {
    /// Provided DMS string is not the correct format
    InvalidFormat,
    /// The provided bearing string is not the correct format
    InvalidBearing,
}

/// Converts a string in DMS format (e.g. 51° 30' 0.0" N) to a decimal degree
pub fn dms_to_double(dms: &str) -> Result<f64, DmsError> {
    let dms_fmt = r#"(?P<d>\d+)°\s*(?P<m>\d+)'\s*(?P<s>\d+\.\d+)"\s*(?P<b>N|S|E|W)"#;

    let re = Regex::new(dms_fmt).unwrap();
    let result = re.captures(dms);
    let Some(cap) = result else {
        region_error!("(dms_to_double) invalid DMS format.");
        region_debug!("(dms_to_double) dms: {}", dms);
        return Err(DmsError::InvalidFormat);
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
            return Err(DmsError::InvalidBearing);
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

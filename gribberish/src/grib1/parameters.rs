/// GRIB1 Parameter Tables
///
/// Maps parameter numbers to variable names and units for different centers.
/// Starting with ECMWF (center 98) parameters.

#[derive(Debug, Clone)]
pub struct Grib1Parameter {
    pub number: u8,
    pub abbreviation: &'static str,
    pub name: &'static str,
    pub units: &'static str,
}

/// Get parameter information for a given center and parameter number
pub fn get_parameter(center_id: u8, parameter: u8) -> Option<Grib1Parameter> {
    match center_id {
        98 => get_ecmwf_parameter(parameter),  // ECMWF
        7 => get_ncep_parameter(parameter),     // NCEP
        _ => get_wmo_standard_parameter(parameter), // WMO standard
    }
}

/// ECMWF parameter table (center 98)
fn get_ecmwf_parameter(parameter: u8) -> Option<Grib1Parameter> {
    let param = match parameter {
        1 => Grib1Parameter {
            number: 1,
            abbreviation: "sp",
            name: "Surface pressure",
            units: "Pa",
        },
        2 => Grib1Parameter {
            number: 2,
            abbreviation: "prmsl",
            name: "Pressure reduced to MSL",
            units: "Pa",
        },
        11 => Grib1Parameter {
            number: 11,
            abbreviation: "t",
            name: "Temperature",
            units: "K",
        },
        20 => Grib1Parameter {
            number: 20,
            abbreviation: "vit",
            name: "Visibility",
            units: "m",
        },
        22 => Grib1Parameter {
            number: 22,
            abbreviation: "clmr",
            name: "Mixing ratio",
            units: "kg kg-1",
        },
        29 => Grib1Parameter {
            number: 29,
            abbreviation: "lvt",
            name: "Type of low vegetation",
            units: "~",
        },
        31 => Grib1Parameter {
            number: 31,
            abbreviation: "ci",
            name: "Sea-ice cover",
            units: "(0-1)",
        },
        32 => Grib1Parameter {
            number: 32,
            abbreviation: "asn",
            name: "Snow albedo",
            units: "(0-1)",
        },
        33 => Grib1Parameter {
            number: 33,
            abbreviation: "rsn",
            name: "Snow density",
            units: "kg m-3",
        },
        34 => Grib1Parameter {
            number: 34,
            abbreviation: "sstk",
            name: "Sea surface temperature",
            units: "K",
        },
        39 => Grib1Parameter {
            number: 39,
            abbreviation: "swvl1",
            name: "Volumetric soil water layer 1",
            units: "m3 m-3",
        },
        44 => Grib1Parameter {
            number: 44,
            abbreviation: "es",
            name: "Snow evaporation",
            units: "m of water equivalent",
        },
        47 => Grib1Parameter {
            number: 47,
            abbreviation: "dsrp",
            name: "Direct solar radiation",
            units: "W m-2 s",
        },
        49 => Grib1Parameter {
            number: 49,
            abbreviation: "10fg",
            name: "10 metre wind gust",
            units: "m s-1",
        },
        50 => Grib1Parameter {
            number: 50,
            abbreviation: "lspf",
            name: "Large-scale precipitation fraction",
            units: "s",
        },
        51 => Grib1Parameter {
            number: 51,
            abbreviation: "q",
            name: "Specific humidity",
            units: "kg kg-1",
        },
        52 => Grib1Parameter {
            number: 52,
            abbreviation: "r",
            name: "Relative humidity",
            units: "%",
        },
        53 => Grib1Parameter {
            number: 53,
            abbreviation: "q",
            name: "Humidity mixing ratio",
            units: "kg kg-1",
        },
        54 => Grib1Parameter {
            number: 54,
            abbreviation: "pwat",
            name: "Precipitable water",
            units: "kg m-2",
        },
        59 => Grib1Parameter {
            number: 59,
            abbreviation: "prate",
            name: "Precipitation rate",
            units: "kg m-2 s-1",
        },
        61 => Grib1Parameter {
            number: 61,
            abbreviation: "tp",
            name: "Total precipitation",
            units: "m",
        },
        66 => Grib1Parameter {
            number: 66,
            abbreviation: "lsff",
            name: "Lake shape factor",
            units: "dimensionless",
        },
        67 => Grib1Parameter {
            number: 67,
            abbreviation: "lmlt",
            name: "Lake mix-layer temperature",
            units: "K",
        },
        71 => Grib1Parameter {
            number: 71,
            abbreviation: "tcc",
            name: "Total cloud cover",
            units: "%",
        },
        78 => Grib1Parameter {
            number: 78,
            abbreviation: "tclw",
            name: "Total column cloud liquid water",
            units: "kg m-2",
        },
        79 => Grib1Parameter {
            number: 79,
            abbreviation: "tciw",
            name: "Total column cloud ice water",
            units: "kg m-2",
        },
        89 => Grib1Parameter {
            number: 89,
            abbreviation: "sunsd",
            name: "Sunshine duration",
            units: "s",
        },
        121 => Grib1Parameter {
            number: 121,
            abbreviation: "mx2t",
            name: "Maximum temperature at 2 metres",
            units: "K",
        },
        122 => Grib1Parameter {
            number: 122,
            abbreviation: "mn2t",
            name: "Minimum temperature at 2 metres",
            units: "K",
        },
        123 => Grib1Parameter {
            number: 123,
            abbreviation: "10fg",
            name: "10 metre wind gust",
            units: "m s-1",
        },
        124 => Grib1Parameter {
            number: 124,
            abbreviation: "emis",
            name: "Surface emissivity",
            units: "dimensionless",
        },
        125 => Grib1Parameter {
            number: 125,
            abbreviation: "veg",
            name: "Vegetation fraction",
            units: "(0-1)",
        },
        126 => Grib1Parameter {
            number: 126,
            abbreviation: "sltyp",
            name: "Soil type",
            units: "dimensionless",
        },
        127 => Grib1Parameter {
            number: 127,
            abbreviation: "cape",
            name: "Convective available potential energy",
            units: "J kg-1",
        },
        128 => Grib1Parameter {
            number: 128,
            abbreviation: "cin",
            name: "Convective inhibition",
            units: "J kg-1",
        },
        129 => Grib1Parameter {
            number: 129,
            abbreviation: "z",
            name: "Geopotential",
            units: "m2 s-2",
        },
        130 => Grib1Parameter {
            number: 130,
            abbreviation: "t",
            name: "Temperature",
            units: "K",
        },
        131 => Grib1Parameter {
            number: 131,
            abbreviation: "u",
            name: "U component of wind",
            units: "m s-1",
        },
        132 => Grib1Parameter {
            number: 132,
            abbreviation: "v",
            name: "V component of wind",
            units: "m s-1",
        },
        133 => Grib1Parameter {
            number: 133,
            abbreviation: "q",
            name: "Specific humidity",
            units: "kg kg-1",
        },
        134 => Grib1Parameter {
            number: 134,
            abbreviation: "sp",
            name: "Surface pressure",
            units: "Pa",
        },
        135 => Grib1Parameter {
            number: 135,
            abbreviation: "w",
            name: "Vertical velocity",
            units: "Pa s-1",
        },
        136 => Grib1Parameter {
            number: 136,
            abbreviation: "tcw",
            name: "Total column water",
            units: "kg m-2",
        },
        137 => Grib1Parameter {
            number: 137,
            abbreviation: "tcwv",
            name: "Total column water vapour",
            units: "kg m-2",
        },
        139 => Grib1Parameter {
            number: 139,
            abbreviation: "stl1",
            name: "Soil temperature level 1",
            units: "K",
        },
        141 => Grib1Parameter {
            number: 141,
            abbreviation: "sd",
            name: "Snow depth",
            units: "m of water equivalent",
        },
        143 => Grib1Parameter {
            number: 143,
            abbreviation: "cp",
            name: "Convective precipitation",
            units: "m",
        },
        144 => Grib1Parameter {
            number: 144,
            abbreviation: "sf",
            name: "Snowfall",
            units: "m of water equivalent",
        },
        148 => Grib1Parameter {
            number: 148,
            abbreviation: "chnk",
            name: "Charnock",
            units: "dimensionless",
        },
        151 => Grib1Parameter {
            number: 151,
            abbreviation: "prmsl",
            name: "Pressure reduced to MSL",
            units: "Pa",
        },
        157 => Grib1Parameter {
            number: 157,
            abbreviation: "r",
            name: "Relative humidity",
            units: "%",
        },
        159 => Grib1Parameter {
            number: 159,
            abbreviation: "blh",
            name: "Boundary layer height",
            units: "m",
        },
        164 => Grib1Parameter {
            number: 164,
            abbreviation: "tcc",
            name: "Total cloud cover",
            units: "(0-1)",
        },
        165 => Grib1Parameter {
            number: 165,
            abbreviation: "u10",
            name: "10 metre U wind component",
            units: "m s-1",
        },
        166 => Grib1Parameter {
            number: 166,
            abbreviation: "v10",
            name: "10 metre V wind component",
            units: "m s-1",
        },
        167 => Grib1Parameter {
            number: 167,
            abbreviation: "t2m",
            name: "2 metre temperature",
            units: "K",
        },
        168 => Grib1Parameter {
            number: 168,
            abbreviation: "d2m",
            name: "2 metre dewpoint temperature",
            units: "K",
        },
        169 => Grib1Parameter {
            number: 169,
            abbreviation: "ssrd",
            name: "Surface solar radiation downwards",
            units: "J m-2",
        },
        179 => Grib1Parameter {
            number: 179,
            abbreviation: "ttr",
            name: "Top net thermal radiation",
            units: "J m-2",
        },
        186 => Grib1Parameter {
            number: 186,
            abbreviation: "lcc",
            name: "Low cloud cover",
            units: "(0-1)",
        },
        187 => Grib1Parameter {
            number: 187,
            abbreviation: "mcc",
            name: "Medium cloud cover",
            units: "(0-1)",
        },
        188 => Grib1Parameter {
            number: 188,
            abbreviation: "hcc",
            name: "High cloud cover",
            units: "(0-1)",
        },
        213 => Grib1Parameter {
            number: 213,
            abbreviation: "vimd",
            name: "Vertically integrated moisture divergence",
            units: "kg m-2",
        },
        217 => Grib1Parameter {
            number: 217,
            abbreviation: "sdwe",
            name: "Standard deviation wave height",
            units: "m",
        },
        218 => Grib1Parameter {
            number: 218,
            abbreviation: "mpww",
            name: "Mean wave period based on second moment",
            units: "s",
        },
        219 => Grib1Parameter {
            number: 219,
            abbreviation: "p1ww",
            name: "Mean wave period based on first moment",
            units: "s",
        },
        220 => Grib1Parameter {
            number: 220,
            abbreviation: "mzww",
            name: "Mean zero-crossing wave period",
            units: "s",
        },
        221 => Grib1Parameter {
            number: 221,
            abbreviation: "ipww",
            name: "Mean period of wind waves",
            units: "s",
        },
        226 => Grib1Parameter {
            number: 226,
            abbreviation: "10ws",
            name: "10 metre wind speed",
            units: "m s-1",
        },
        228 => Grib1Parameter {
            number: 228,
            abbreviation: "tp",
            name: "Total precipitation",
            units: "m",
        },
        229 => Grib1Parameter {
            number: 229,
            abbreviation: "iews",
            name: "Instantaneous eastward turbulent surface stress",
            units: "N m-2",
        },
        230 => Grib1Parameter {
            number: 230,
            abbreviation: "inss",
            name: "Instantaneous northward turbulent surface stress",
            units: "N m-2",
        },
        231 => Grib1Parameter {
            number: 231,
            abbreviation: "ishf",
            name: "Instantaneous surface heat flux",
            units: "W m-2",
        },
        232 => Grib1Parameter {
            number: 232,
            abbreviation: "ie",
            name: "Instantaneous moisture flux",
            units: "kg m-2 s-1",
        },
        234 => Grib1Parameter {
            number: 234,
            abbreviation: "lsrh",
            name: "Logarithm of surface roughness length for heat",
            units: "dimensionless",
        },
        235 => Grib1Parameter {
            number: 235,
            abbreviation: "skt",
            name: "Skin temperature",
            units: "K",
        },
        236 => Grib1Parameter {
            number: 236,
            abbreviation: "stl4",
            name: "Soil temperature level 4",
            units: "K",
        },
        237 => Grib1Parameter {
            number: 237,
            abbreviation: "swvl4",
            name: "Volumetric soil water layer 4",
            units: "m3 m-3",
        },
        238 => Grib1Parameter {
            number: 238,
            abbreviation: "tsn",
            name: "Temperature of snow layer",
            units: "K",
        },
        239 => Grib1Parameter {
            number: 239,
            abbreviation: "csf",
            name: "Convective snowfall",
            units: "m of water equivalent",
        },
        240 => Grib1Parameter {
            number: 240,
            abbreviation: "lsf",
            name: "Large-scale snowfall",
            units: "m of water equivalent",
        },
        241 => Grib1Parameter {
            number: 241,
            abbreviation: "acf",
            name: "Accumulated cloud fraction tendency",
            units: "(-1 to 1)",
        },
        243 => Grib1Parameter {
            number: 243,
            abbreviation: "fal",
            name: "Forecast albedo",
            units: "(0-1)",
        },
        244 => Grib1Parameter {
            number: 244,
            abbreviation: "fsr",
            name: "Forecast surface roughness",
            units: "m",
        },
        246 => Grib1Parameter {
            number: 246,
            abbreviation: "clwc",
            name: "Cloud liquid water content",
            units: "kg kg-1",
        },
        247 => Grib1Parameter {
            number: 247,
            abbreviation: "ciwc",
            name: "Cloud ice water content",
            units: "kg kg-1",
        },
        _ => return None,
    };

    Some(param)
}

/// NCEP parameter table (center 7) - subset of common parameters
fn get_ncep_parameter(parameter: u8) -> Option<Grib1Parameter> {
    get_wmo_standard_parameter(parameter)
}

/// WMO standard parameter table (Table 2)
fn get_wmo_standard_parameter(parameter: u8) -> Option<Grib1Parameter> {
    let param = match parameter {
        1 => Grib1Parameter {
            number: 1,
            abbreviation: "pres",
            name: "Pressure",
            units: "Pa",
        },
        2 => Grib1Parameter {
            number: 2,
            abbreviation: "prmsl",
            name: "Pressure reduced to MSL",
            units: "Pa",
        },
        7 => Grib1Parameter {
            number: 7,
            abbreviation: "gh",
            name: "Geopotential height",
            units: "gpm",
        },
        11 => Grib1Parameter {
            number: 11,
            abbreviation: "t",
            name: "Temperature",
            units: "K",
        },
        33 => Grib1Parameter {
            number: 33,
            abbreviation: "u",
            name: "U-component of wind",
            units: "m s-1",
        },
        34 => Grib1Parameter {
            number: 34,
            abbreviation: "v",
            name: "V-component of wind",
            units: "m s-1",
        },
        39 => Grib1Parameter {
            number: 39,
            abbreviation: "w",
            name: "Vertical velocity",
            units: "Pa s-1",
        },
        51 => Grib1Parameter {
            number: 51,
            abbreviation: "q",
            name: "Specific humidity",
            units: "kg kg-1",
        },
        52 => Grib1Parameter {
            number: 52,
            abbreviation: "r",
            name: "Relative humidity",
            units: "%",
        },
        61 => Grib1Parameter {
            number: 61,
            abbreviation: "tp",
            name: "Total precipitation",
            units: "kg m-2",
        },
        _ => return None,
    };

    Some(param)
}

/// Get level type name and units
pub fn get_level_type_info(level_type: u8) -> (&'static str, &'static str) {
    match level_type {
        1 => ("surface", ""),
        2 => ("cloud_base", ""),
        3 => ("cloud_top", ""),
        4 => ("isotherm_zero", "m"),
        100 => ("isobaric", "hPa"),
        102 => ("mean_sea_level", ""),
        103 => ("fixed_height", "m"),
        105 => ("fixed_height_above_ground", "m"),
        106 => ("sigma", "sigma"),
        107 => ("sigma_height", "sigma"),
        108 => ("sigma_pressure", "sigma"),
        109 => ("hybrid", "hybrid"),
        111 => ("depth_below_surface", "m"),
        112 => ("layer_between_depths", "m"),
        113 => ("isentropic", "K"),
        114 => ("layer_between_isentropic", "K"),
        200 => ("entire_atmosphere", ""),
        201 => ("entire_ocean", ""),
        _ => ("unknown", ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecmwf_parameters() {
        let param = get_parameter(98, 11).unwrap();
        assert_eq!(param.abbreviation, "t");
        assert_eq!(param.name, "Temperature");

        let param = get_parameter(98, 131).unwrap();
        assert_eq!(param.abbreviation, "u");
    }

    #[test]
    fn test_level_types() {
        let (name, units) = get_level_type_info(100);
        assert_eq!(name, "isobaric");
        assert_eq!(units, "hPa");

        let (name, _) = get_level_type_info(1);
        assert_eq!(name, "surface");
    }
}

use std::ops::Range;

#[derive(Debug, Clone)]
pub struct StringGroupConfigurations {
    pub configurations: Vec<(FrequencyRange, StringGroupConfiguration)>,
}

pub type FrequencyRange = Range<f64>;

#[derive(Debug, Default, Copy, Clone)]
pub struct StringGroupConfiguration {
    /// Every hammer has 1-3 strings. We assume that the first string is in perfect tune and string 2 and 3 are a bit offtuned.
    /// This is the amount of offtuning of string 2 and 3.
    pub offtune: f64,
    pub ap_num: i32,
    pub gl: f64,
}

impl Default for StringGroupConfigurations {
    fn default() -> Self {
        Self {
            configurations: vec![
                (
                    0.0..120.0,
                    StringGroupConfiguration {
                        offtune: 0.25,
                        ap_num: 20,
                        gl: -0.96,
                    },
                ),
                (
                    120.0..150.0,
                    StringGroupConfiguration {
                        offtune: 0.18,
                        ap_num: 20,
                        gl: -0.968,
                    },
                ),
                (
                    150.0..200.0,
                    StringGroupConfiguration {
                        offtune: 0.13,
                        ap_num: 18,
                        gl: -0.975,
                    },
                ),
                (
                    200.0..261.626,
                    StringGroupConfiguration {
                        offtune: 0.09,
                        ap_num: 16,
                        gl: -0.98,
                    },
                ),
                (
                    261.626..390.0,
                    StringGroupConfiguration {
                        offtune: 0.06,
                        ap_num: 14,
                        gl: -0.985,
                    },
                ),
                (
                    390.0..750.0,
                    StringGroupConfiguration {
                        offtune: 0.04,
                        ap_num: 12,
                        gl: -0.99,
                    },
                ),
                (
                    750.0..980.0,
                    StringGroupConfiguration {
                        offtune: 0.03,
                        ap_num: 8,
                        gl: -0.993,
                    },
                ),
                (
                    980.0..1500.0,
                    StringGroupConfiguration {
                        offtune: 0.02,
                        ap_num: 6,
                        gl: -0.995,
                    },
                ),
                (
                    1500.0..1800.0,
                    StringGroupConfiguration {
                        offtune: 0.01,
                        ap_num: 4,
                        gl: -0.995,
                    },
                ),
                (
                    1800.0..1900.0,
                    StringGroupConfiguration {
                        offtune: 0.005,
                        ap_num: 3,
                        gl: -0.977,
                    },
                ),
                (
                    1900.0..3000.0,
                    StringGroupConfiguration {
                        offtune: 0.005,
                        ap_num: 2,
                        gl: -0.977,
                    },
                ),
                (
                    3000.0..f64::MAX,
                    StringGroupConfiguration {
                        offtune: 0.01,
                        ap_num: 0,
                        gl: -0.977,
                    },
                ),
            ],
        }
    }
}

impl StringGroupConfigurations {
    pub fn get_configuration(&self, frequency: f64) -> &'_ StringGroupConfiguration {
        for (range, config) in &self.configurations {
            if range.contains(&frequency) {
                return config;
            }
        }
        unreachable!()
    }
}

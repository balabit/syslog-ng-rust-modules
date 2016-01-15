use ::types::*;
use ::ffi::from_c_str_to_borrowed_str;

pub enum GlobalConfig {}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn cfg_get_user_version(cfg: *const GlobalConfig) -> c_int;
    pub fn cfg_get_parsed_version(cfg: *const GlobalConfig) -> c_int;
    pub fn cfg_get_filename(cfg: *const GlobalConfig) -> *const c_char;
}

impl GlobalConfig {
    pub fn get_user_version(&self) -> (u8,u8) {
       let mut version = unsafe {
           cfg_get_user_version(self)
       };

       if version < 0 {
           error!("User config version must be greater than 0, using 0 as version");
           version = 0;
       }

       convert_version(version as u16)
    }

    pub fn get_parsed_version(&self) -> (u8,u8) {
       let mut version = unsafe {
           cfg_get_parsed_version(self)
       };

       if version < 0 {
           error!("Parsed config version must be greater than 0, using 0 as version");
           version = 0;
       }

       convert_version(version as u16)
    }

    pub fn get_filename(&self) -> &str {
        unsafe {
            from_c_str_to_borrowed_str(cfg_get_filename(self))
        }
    }
}

fn hex_to_dec(hex: u8) -> u8 {
    let mut dec = 0;
    let mut shifted_hex = hex;

    for i in 0..2 {
        dec += (shifted_hex % 16) * 10u8.pow(i);
        shifted_hex >>= 4;
    }

    dec
}

fn convert_version(version: u16) -> (u8, u8) {
   let minor = hex_to_dec(version as u8);
   let major = hex_to_dec((version >> 8) as u8);
   (major, minor)
}

#[test]
fn one_digit_hex_number_when_converted_to_decimal_works() {
    let dec = GlobalConfig::hex_to_dec(0x3);
    assert_eq!(dec, 3);
}

#[test]
fn more_digits_hex_number_when_converted_to_decimal_works() {
    let dec = GlobalConfig::hex_to_dec(0x22);
    assert_eq!(dec, 22);
}

#[test]
fn hex_version_when_converted_to_minor_version_works() {
    let version = 0x0316;

    let (_, minor) = GlobalConfig::convert_version(version);
    assert_eq!(minor, 16);
}

#[test]
fn hex_version_when_converted_to_major_version_works() {
    let version = 0x0316;

    let (major, _) = GlobalConfig::convert_version(version);
    assert_eq!(major, 3);
}

#![allow(clippy::enum_clike_unportable_variant)]



use libc::{c_char, c_int, c_uint, c_ulong, c_void};


// TODO: ----- General Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_version(major: *mut c_uint, minor: *mut c_uint) -> c_int;
    pub fn zbar_set_verbosity(verbosity: c_int);
}

// TODO: ----- General Interface END-----

// TODO: ----- Image Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_image_create() -> *mut c_void;
    pub fn zbar_image_destroy(image: *mut c_void);
    pub fn zbar_image_ref(image: *mut c_void, refs: c_int);
    pub fn zbar_image_convert(image: *const c_void, format: c_ulong) -> *mut c_void;
    pub fn zbar_image_convert_resize(
        image: *const c_void,
        format: c_ulong,
        width: c_uint,
        height: c_int,
    ) -> *mut c_void;
    pub fn zbar_image_get_format(image: *const c_void) -> c_ulong;
    pub fn zbar_image_get_sequence(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_width(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_height(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_size(image: *const c_void, width: *mut c_uint, height: *mut c_uint);
    pub fn zbar_image_get_crop(
        image: *const c_void,
        x: *mut c_uint,
        y: *mut c_uint,
        width: *mut c_uint,
        height: *mut c_uint,
    );
    pub fn zbar_image_get_data(image: *const c_void) -> *const c_void;
    pub fn zbar_image_get_data_length(image: *const c_void) -> c_ulong;
    pub fn zbar_image_get_symbols(image: *const c_void) -> *const c_void;
    pub fn zbar_image_set_symbols(image: *mut c_void, symbols: *const c_void);
    pub fn zbar_image_first_symbol(image: *const c_void) -> *const c_void;
    pub fn zbar_image_set_format(image: *mut c_void, format: c_ulong);
    pub fn zbar_image_set_sequence(image: *mut c_void, sequence_num: c_ulong);
    pub fn zbar_image_set_size(image: *mut c_void, width: c_ulong, height: c_ulong);
    pub fn zbar_image_set_crop(
        image: *mut c_void,
        x: c_ulong,
        y: c_ulong,
        width: c_ulong,
        height: c_ulong,
    );
    pub fn zbar_image_set_data(
        image: *mut c_void,
        data: *const c_void,
        data_byte_length: c_ulong,
        handler: *mut c_void,
    );
    pub fn zbar_image_free_data(image: *mut c_void);
    pub fn zbar_image_set_userdata(image: *mut c_void, userdata: *const c_void);
    pub fn zbar_image_get_userdata(image: *const c_void) -> *const c_void;
    pub fn zbar_image_write(image: *const c_void, filebase: *const c_char) -> c_uint;
    pub fn zbar_image_read(filename: *mut c_char) -> *const c_void;
}

// TODO: ----- Image Interface END-----

// TODO: ----- Symbol Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_symbol_ref(symbol: *const c_void, refs: c_int);
    pub fn zbar_symbol_get_type(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_configs(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_modifiers(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_data(symbol: *const c_void) -> *mut c_char;
    pub fn zbar_symbol_get_data_length(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_quality(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_count(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_loc_size(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_loc_x(symbol: *const c_void, index: c_uint) -> c_int;
    pub fn zbar_symbol_get_loc_y(symbol: *const c_void, index: c_uint) -> c_int;
    pub fn zbar_symbol_get_orientation(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_next(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_get_components(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_first_component(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_xml(
        symbol: *const c_void,
        buffer: *mut *mut c_char,
        buflen: *mut c_uint,
    ) -> *mut c_char;
}

// TODO: ----- Symbol Interface END-----

// TODO: ----- Image Scanner Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_image_scanner_create() -> *mut c_void;
    pub fn zbar_image_scanner_destroy(scanner: *mut c_void);
    pub fn zbar_image_scanner_set_data_handler(
        scanner: *mut c_void,
        handler: *const c_void,
        userdata: *const c_void,
    );
    pub fn zbar_image_scanner_set_config(
        scanner: *mut c_void,
        symbology: c_int,
        config: c_int,
        value: c_int,
    ) -> c_int;
    pub fn zbar_image_scanner_parse_config(
        scanner: *mut c_void,
        config_string: *const c_char,
    ) -> c_int;
    pub fn zbar_image_scanner_enable_cache(scanner: *mut c_void, enable: c_int);
    pub fn zbar_image_scanner_recycle_image(scanner: *mut c_void, image: *mut c_void);
    pub fn zbar_image_scanner_get_results(scanner: *const c_void) -> *const c_void;
    pub fn zbar_scan_image(scanner: *mut c_void, image: *mut c_void) -> c_int;
}
// TODO: ----- Image Scanner Interface END-----

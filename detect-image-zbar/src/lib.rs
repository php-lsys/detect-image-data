/*!
High-level and low-level ZBar binding for the Rust language.

## Compilation

To compile this crate, you need to compile the ZBar library first. You can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.

## Examples

```rust,ignore
extern crate zbar_rust;
extern crate image;

use zbar_rust::ZBarImageScanner;

use image::GenericImageView;

let img = image::open(INPUT_IMAGE_PATH).unwrap();

let (width, height) = img.dimensions();

let luma_img = img.to_luma();

let luma_img_data: Vec<u8> = luma_img.to_vec();

let mut scanner = ZBarImageScanner::new();

let results = scanner.scan_y800(&luma_img_data, width, height).unwrap();

for result in results {
    println!("{}", String::from_utf8(result.data).unwrap())
}
```

More examples are in the `examples` folder.
*/
extern crate libc;
#[macro_use]
extern crate enum_ordinalize;

pub mod c_lib;


use image::GenericImageView;
use libc::{ c_int, c_ulong, c_void};
use c_lib::*;
use detect_image_core::{ResultType, ResultPoint, DetectCode, ResultData, ResultPolygon, ResultItem, ResultError, ResultInfo};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarSymbolType {
    ZBarNone       = 0,
    ZBarPartial    = 1,
    ZBarEAN2       = 2,
    ZBarEAN5       = 5,
    ZBarEAN8       = 8,
    ZBarUPCE       = 9,
    ZBarISBN10     = 10,
    ZBarUPCA       = 12,
    ZBarEAN13      = 13,
    ZBarISBN13     = 14,
    ZBarComposite  = 15,
    ZBarI25        = 25,
    ZBarDataBar    = 34,
    ZBarDataBarExp = 35,
    ZBarCodeBar    = 38,
    ZBarCode39     = 39,
    ZBarPDF417     = 57,
    ZBarQRCode     = 64,
    ZBarCode93     = 93,
    ZBarCode128    = 128,
    ZBarSymbol     = 0x00ff,
    ZBarAddOn2     = 0x0200,
    ZBarAddOn5     = 0x0500,
    ZBarAddOn      = 0x0700,
}

impl ResultType for ZBarSymbolType {
    fn to_int(&self)->i32{
        return *self as i32
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarConfig {
    ZBarCfgEnable    = 0,
    ZBarCfgAddCheck  = 1,
    ZBarCfgEmitCheck = 2,
    ZBarCfgASCII     = 3,
    ZBarCfgNum       = 4,
    ZBarCfgMinLen    = 0x20,
    ZBarCfgMaxLen    = 0x21,
    ZBarCfgPosition  = 0x80,
    ZBarCfgXDensity  = 0x100,
    ZBarCfgYDensity  = 0x101,
}


pub struct ZBarImage {
    image: *mut c_void,
}

impl ZBarImage {
    pub fn new() -> ZBarImage {
        let image = unsafe { zbar_image_create() };
        ZBarImage {
            image,
        }
    }

    pub fn set_format(&mut self, format: u32) {
        unsafe {
            zbar_image_set_format(self.image, c_ulong::from(format));
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            zbar_image_set_size(self.image, c_ulong::from(width), c_ulong::from(height));
        }
    }

    pub fn set_ref(&mut self, r: isize) {
        unsafe {
            zbar_image_ref(self.image, r as c_int);
        }
    }
}

impl Default for ZBarImage {
    #[inline]
    fn default() -> Self {
        ZBarImage::new()
    }
}


// impl Drop for ZBarImage {
//     fn drop(&mut self) {
//         if !self.image.is_null() {
//             unsafe {
//                 zbar_image_destroy(self.image);
//             }
//         }
//     }
// }



#[derive(Debug)]
pub struct ZBarImageScanItem {
    ponit_data:ResultPolygon,
    symbol_type: ZBarSymbolType,
    data: Vec<u8>,
}

impl ResultItem for ZBarImageScanItem{
    type RT=ZBarSymbolType;
    fn get_type(&self)-> Self::RT{
        return self.symbol_type
    }
    fn get_data(&self)->Result<&Vec<u8>,&String>{
        return Ok(&self.data)
    }
    fn get_polygon(&self)->Option<&ResultPolygon>{
        return Some(&self.ponit_data)
    }
}

#[derive(Debug)]
pub struct ZBarImageScanData{
    item:Vec<ZBarImageScanItem>,
    info:ResultInfo,
}

impl ResultData for ZBarImageScanData{
    type ITEM=ZBarImageScanItem;
    fn to_vec(&self)->&Vec<Self::ITEM>{
        return &self.item;
    }
    fn info(&self)->&ResultInfo {
        return &self.info;
    }
}

pub struct ZBarImageScanner {
    scanner: *mut c_void,
    pub format:u32,
}

impl ZBarImageScanner {
    pub fn new() -> ZBarImageScanner {
        let scanner = unsafe { zbar_image_scanner_create() };
        
        ZBarImageScanner {
            scanner,
            format:808_466_521, //1_497_453_127
        }
    }

    pub fn set_config(
        &mut self,
        symbology: ZBarSymbolType,
        config: ZBarConfig,
        value: isize,
    ) -> Result<(),ResultError<&'static str>> {
        let result = unsafe {
            zbar_image_scanner_set_config(
                self.scanner,
                symbology.ordinal() as c_int,
                config.ordinal() as c_int,
                value as c_int,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(ResultError::Detect("unsuccessfully"))
        }
    }

    pub fn scan<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32
    ) -> Result<ZBarImageScanData,ResultError<&'static str>> {
        let format=self.format;
        let data = data.as_ref();

        let mut image = ZBarImage::new();
        
        image.set_size(width, height);
        image.set_format(format);

        unsafe {
            zbar_image_set_data(
                image.image,
                data.as_ptr() as *const c_void,
                data.len() as c_ulong,
                zbar_image_free_data as *mut c_void,
            );
        }

        let n = unsafe { zbar_scan_image(self.scanner, image.image) };

        if n < 0 {
             return Err(ResultError::Detect("can't detect,plase check your param"));
        }

        let mut result_array = Vec::with_capacity(n as usize);

        let mut symbol = unsafe { zbar_image_first_symbol(image.image) };

        while !symbol.is_null() {
           
            let symbol_type = unsafe { zbar_symbol_get_type(symbol) };
            let symbol_type = unsafe { ZBarSymbolType::from_ordinal_unsafe(symbol_type as isize) };
            let data = unsafe {
                let data = zbar_symbol_get_data(symbol);
                let data_length = zbar_symbol_get_data_length(symbol) as usize;
                Vec::from_raw_parts(data as *mut u8, data_length, data_length)
            };

            let ponit_data=unsafe { 
                let len = zbar_symbol_get_loc_size(symbol);
                let mut points:Vec<ResultPoint> =Vec::with_capacity(len as usize);
                for i in 0..len{
                    points.push(ResultPoint{
                        x:zbar_symbol_get_loc_x(symbol,i),
                        y:zbar_symbol_get_loc_y(symbol,i),
                    })
                }
                points
            };            
            let result = ZBarImageScanItem {
                ponit_data:ResultPolygon::Point(ponit_data),
                symbol_type,
                data,
            };

            result_array.push(result);

            symbol = unsafe { zbar_symbol_next(symbol) };
        }

        Ok(ZBarImageScanData{
            item:result_array,
            info:ResultInfo{
                width,
                height
            }
        })
    }
}


impl Default for ZBarImageScanner {
    #[inline]
    fn default() -> Self {
        ZBarImageScanner::new()
    }
}

impl Drop for ZBarImageScanner {
    fn drop(&mut self) {
        if !self.scanner.is_null() {
            unsafe {
                zbar_image_scanner_destroy(self.scanner);
            }
        }
    }
}

impl DetectCode for ZBarImageScanner {
    type DD=ZBarImageScanData;
    type DE=&'static str;
    fn detect_scan(&self,path:&str)->Result<ZBarImageScanData,ResultError<&'static str>>{
        let image = image::open(path).map_err(|e|{
            ResultError::file(path.to_owned(),e.to_string())
        })?;
        let (width, height) = image.dimensions();
        let luma_img = image.to_luma8();
        let luma_img_data: Vec<u8> = luma_img.to_vec();
        let mut scanner = ZBarImageScanner::new();
        if self.format>0 {
            scanner.format=    self.format;
        }
        return scanner.scan(&luma_img_data, width, height);
    }
}

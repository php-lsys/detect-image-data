mod c_lib;
use std::ffi::{CString};
use std::{mem};
use std::os::raw::{c_int};
use std::slice::from_raw_parts;

use c_lib::*;
use detect_image_core::{DetectCode, ResultData, ResultType, ResultPoint, ResultItem, ResultPolygon, ResultError, ResultInfo};





#[derive(Debug,Clone, Copy)]
pub enum ZXingDecodedType{
    None            = 0,         //< Used as a return value if no valid barcode has been detected
	Codabar         = 38,  //< Codabar (1D)
	Code39          =  39,  //< Code39 (1D)
	Code93          = 93,  //< Code93 (1D)
	Code128         = 128,  //< Code128 (1D)
	DataBar         = 34,  //< GS1 DataBar, formerly known as RSS 14
	DataBarExpanded = 35,  //< GS1 DataBar Expanded, formerly known as RSS EXPANDED
	EAN8            = 8,  //< EAN-8 (1D)
	EAN13           = 13,  //< EAN-13 (1D)
	PDF417          = 57, //< PDF417 (1D) or (2D)
	QRCode          = 64, //< QR Code (2D)
	UPCA            = 12, //< UPC-A (1D)
	UPCE            = 9, //< UPC-E (1D)
    Aztec           = 201,  //< Aztec (2D)
	DataMatrix      =202,  //< DataMatrix (2D)
    ITF             = 203, //< ITF (Interleaved Two of Five) (1D)
	MaxiCode        = 204, //< MaxiCode (2D)
}

impl ResultType for ZXingDecodedType {
    fn to_int(&self)->i32{
        return *self as i32
    }
}

impl From<c_int> for ZXingDecodedType{
    fn from(c: c_int) -> Self {
        if  c==(1 << 1){ZXingDecodedType::Codabar          //< Codabar (1D)
        }else if  c==(1 << 2){ZXingDecodedType::Code39           //< Code39 (1D)
        }else if  c==(1 << 3){ZXingDecodedType::Code93           //< Code93 (1D)
        }else if  c==(1 << 4){ZXingDecodedType::Code128          //< Code128 (1D)
        }else if  c==(1 << 5){ZXingDecodedType::DataBar          //< GS1 DataBar, formerly known as RSS 14
        }else if  c==(1 << 6){ZXingDecodedType::DataBarExpanded  //< GS1 DataBar Expanded, formerly known as RSS EXPANDED
        }else if  c==(1 << 7){ZXingDecodedType::DataMatrix       //< DataMatrix (2D)
        }else if  c==(1 << 8){ZXingDecodedType::EAN8             //< EAN-8 (1D)
        }else if  c==(1 << 9){ZXingDecodedType::EAN13            //< EAN-13 (1D)
        }else if  c==(1 << 10){ZXingDecodedType::ITF             //< ITF (Interleaved Two of Five) (1D)
        }else if  c==(1 << 11){ZXingDecodedType::MaxiCode        //< MaxiCode (2D)
        }else if  c==(1 << 12){ZXingDecodedType::PDF417          //< PDF417 (1D) or (2D)
        }else if  c==(1 << 13){ZXingDecodedType::QRCode          //< QR Code (2D)
        }else if  c==(1 << 14){ZXingDecodedType::UPCA            //< UPC-A (1D)
        }else if  c==(1 << 15){ZXingDecodedType::UPCE            //< UPC-E (1D)
        }else {ZXingDecodedType::None}
    }
}

#[derive(Debug)]
pub struct ZXingDecodedItem {
    pub num_bits: i32,
    pub orientation:f64,
    pub line_count:i32,
    pub symbol_type:ZXingDecodedType,
    pub text: Vec<u8>,
    pub corners:ResultPolygon,
}

impl ResultItem for ZXingDecodedItem{
    type RT=ZXingDecodedType;
    fn get_type(&self)-> Self::RT{
        return ZXingDecodedType::QRCode
    }
    fn get_data(&self)->Result<&Vec<u8>,&String>{
        return Ok(&self.text)
    }
    fn get_polygon(&self)->Option<&ResultPolygon>{
        return Some(&self.corners)
    }
}

#[derive(Debug)]
pub struct DecodedData {
    raw_result: *mut ZxingCResult,
    list:Vec<ZXingDecodedItem>,
    info:ResultInfo,
}
impl Drop for DecodedData {
    fn drop(&mut self) {
        unsafe {
            if !self.raw_result.is_null() {
                zxing_release_result(self.raw_result);
            }
        }
    }
}


// let text = from_utf8(s).unwrap();
// println!("{}",text);
// println!("{}",(*loop_result).bytes_size);


impl ResultData for DecodedData{
    type ITEM=ZXingDecodedItem;
    fn to_vec(&self)->&Vec<Self::ITEM>{
        return &self.list;
    }
    fn info(&self)->&ResultInfo {
        return &self.info;
    }
}

pub struct ZxingDecoder{
    pub fast:bool,
    pub norotate:bool,
    pub ispure:bool,
    pub desired_channels:i8
}


impl ZxingDecoder {
    pub fn new() -> ZxingDecoder {
        ZxingDecoder {
            fast:false,
            norotate:false,
            ispure:false,
            desired_channels:4,
        }
    }
}

impl DetectCode for ZxingDecoder {
    type DD=DecodedData;
    type DE=&'static str;
    fn detect_scan(&self,path:&str)->Result<DecodedData,ResultError<&'static str>>{

        match self.desired_channels {
            0|1|3|4=>{}
            _=>{
                return Err(ResultError::Detect("desired_channels error"));
            }
        }

        let attr = std::fs::metadata(path);
        match attr {
            Ok(meta)=>{
                if !meta.is_file() {
                    return Err(ResultError::file(path.to_string(), "not a file".to_string()));    
                }
            }
            Err(e) =>{
                return Err(ResultError::file(path.to_string(), e.to_string()));
            }
        }
        
        let mut result = mem::MaybeUninit::<*mut ZxingCResult>::uninit();
        let mut width = mem::MaybeUninit::<i32>::uninit();
        let mut height = mem::MaybeUninit::<i32>::uninit();
        let mut out=vec![];

        let cpath= CString::new(path).map_err(|e|{
            ResultError::file(path.to_owned(), e.to_string())
        })?;
        unsafe {
            let ret_code = zxing_read_qrcode(
                result.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                cpath.as_ptr() as *mut i8,
                self.fast as crate::c_int,
                self.norotate as crate::c_int,
                self.ispure as crate::c_int,
                self.desired_channels as crate::c_int
            );
            if ret_code == -1 {
                std::ptr::drop_in_place(width.as_mut_ptr());
                std::ptr::drop_in_place(height.as_mut_ptr());
                std::ptr::drop_in_place(result.as_mut_ptr());
                return Err(ResultError::file(path.to_owned(), "file error".to_string()));
            }
            let width = width.assume_init();
            let height = height.assume_init();
            if ret_code == -2 {
                std::ptr::drop_in_place(result.as_mut_ptr());
                return Ok(DecodedData{
                    raw_result:std::ptr::null::<ZxingCResult>() as *mut ZxingCResult,
                    list: out,
                    info:ResultInfo{
                        width:width as u32,
                        height:height as u32,
                    }
                });
            }
            let result = result.assume_init();
            let mut loop_result=result as  *const ZxingCResult;
            loop {

                let s = from_raw_parts((*loop_result).bytes, (*loop_result).bytes_size as usize);
                let ponit_data={
                    let mut points:Vec<ResultPoint> =Vec::with_capacity((*result).corners.len()/2);
                    for (x,y) in (*result).corners.chunks(2).map(|c| (c[0], c[1])){
                        points.push(ResultPoint{
                            x,
                            y,
                        })
                    }
                    points
                };
                let result = ZXingDecodedItem {
                    num_bits:(*result).num_bits,
                    orientation:(*result).orientation,
                    line_count:(*result).line_count,
                    symbol_type:(*result).format.into(),
                    text:s.to_vec(),
                    corners:ResultPolygon::Point(ponit_data),
                };
                out.push(result);

                if (*loop_result).next.is_null() {
                    break;
                }
                loop_result=(*loop_result).next;
            }
            return Ok(DecodedData{
                raw_result:result,
                list: out,
                info:ResultInfo{
                    width:width as u32,
                    height:height as u32,
                }
            });
        }
    }
}

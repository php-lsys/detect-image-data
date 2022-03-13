

use detect_image_data::{ResultType, ResultPoint, DetectCode, ResultData, ResultPolygon, ResultItem, ResultError, ResultInfo};
#[derive(Debug,Clone, Copy)]
pub enum RqrrType {
    QRCode     = 64,
}

impl ResultType for RqrrType {
    fn to_int(&self)->i32{
        return *self as i32
    }
}


#[derive(Debug)]
pub struct RqrrScanItem {
    ponit_data:ResultPolygon,
    symbol_type: RqrrType,
    data:Result<Vec<u8>,String>,
}

impl ResultItem for RqrrScanItem{
    type RT=RqrrType;
    fn get_type(&self)-> Self::RT{
        return self.symbol_type
    }
    fn get_data(&self)->Result<&Vec<u8>,&String>{
        self.data.as_ref()
    }
    fn get_polygon(&self)->Option<&ResultPolygon>{
        return Some(&self.ponit_data)
    }
}

#[derive(Debug)]
pub struct RqrrImageScanData{
    item:Vec<RqrrScanItem>,
    info:ResultInfo,
}

impl ResultData for RqrrImageScanData{
    type ITEM=RqrrScanItem;
    fn to_vec(&self)->&Vec<Self::ITEM>{
        return &self.item;
    }
    fn info(&self)->&ResultInfo{
        return &self.info;
    }
}

pub struct RqrrScanner {}

impl DetectCode for RqrrScanner {
    type DD=RqrrImageScanData;
    type DE=&'static str;
    fn detect_scan(&self,path:&str)->Result<RqrrImageScanData,ResultError<&'static str>>{
        let img = image::open(path).map_err(|e|{
            ResultError::file(path.to_owned(),e.to_string())
        })?;
        let mut img = rqrr::PreparedImage::prepare(img.to_luma8());
        let grids = img.detect_grids();
        let mut result_array = Vec::with_capacity(grids.len());
        if grids.len()==0 {
            return  Ok(RqrrImageScanData{
                item:result_array,
                info:ResultInfo{
                    width:img.width() as u32,
                    height:img.height()as u32,
                },
            });
        }
        for res in grids.iter() {
            let data=match res.decode() {
                Ok((_,data))=>{
                   Ok(data.into_bytes())
                },
                Err(err)=>{
                    Err(err.to_string())
                }
            };
            let ponit_data={ 
                let mut points:Vec<ResultPoint> =Vec::with_capacity(res.bounds.len());
                for point in res.bounds.iter(){
                    points.push(ResultPoint{
                        x:point.x,
                        y:point.y,
                    })
                }
                points
            };            
            let result = RqrrScanItem {
                ponit_data:ResultPolygon::Point(ponit_data),
                symbol_type:RqrrType::QRCode,
                data,
            };
            result_array.push(result);
           
        }
        Ok(RqrrImageScanData{
            item:result_array,
            info:ResultInfo{
                width:img.width() as u32,
                height:img.height()as u32,
            },
        })
    }
}

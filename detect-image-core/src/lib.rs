use std::{fmt::Debug};
/// 检测结果类型
pub trait ResultType :Debug{
    fn to_int(&self)->i32;
}
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub struct ResultPoint{
    pub x:i32,
    pub y:i32,
}

#[derive(Debug)]
pub enum ResultPolygon {
    Point(Vec<ResultPoint>),
}

/// 检测结果数据
pub trait ResultItem {
    type RT:ResultType;
    fn get_type(&self)-> Self::RT;
    fn get_data(&self)->Result<&Vec<u8>,&String>;
    fn get_polygon(&self)->Option<&ResultPolygon>;
}

#[derive(Debug)]
pub struct ResultInfo{
    pub width:u32,
    pub height:u32,
}

pub trait ResultData {
    type ITEM:ResultItem;
    fn info(&self)->&ResultInfo;
    fn to_vec(&self)->&Vec<Self::ITEM>;
}




#[derive(Debug)]
pub enum ResultError<T:ToString> {
    FileError{
        path:String,
        err:String,
    },
    Detect(T)
}
impl <T:ToString>ResultError<T>{
    pub fn file(path:String,err:String) -> ResultError<T> {
        ResultError::FileError{
            path,
            err,
        }
    }
}
impl <T:ToString> ToString for ResultError<T>{
    fn to_string(&self) -> String {
        match self {
            ResultError::FileError { path, err } =>{
                format!("file error:{} [{}]",err,path)
            },
            ResultError::Detect(d) =>{
                d.to_string()
            },
        }
    }
}

/// 检测方法
pub trait DetectCode {
    type DD:ResultData;
    type DE:ToString;
    fn detect_scan(&self,image_path:&str)->Result<Self::DD,ResultError<Self::DE>>;
}

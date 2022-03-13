use detect_image_data::{DetectCode, ResultData, ResultItem};
use detect_image_data::ZxingDecoder;

fn main(){
    let zxing=ZxingDecoder::new();
    let path = std::env::args().nth(1).expect("no image path given");
    let res=zxing.detect_scan(path.as_str()).unwrap();
    println!("count:{:?} size:{:?}",res.to_vec().len(),res.info());
    for item in res.to_vec().iter(){
        println!("{:?}",item.get_type());
        println!("{}",String::from_utf8(item.get_data().unwrap().to_owned()).unwrap());
        println!("{:?}",item.get_polygon());
    }
}
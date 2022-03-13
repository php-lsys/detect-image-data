use detect_image_data::{ResultData, ResultItem, ResultType, ResultPolygon, ResultError};
use phper::{classes::{DynamicClass, Visibility, StatelessClassEntry}, objects::Object, values::Val, arrays::Array, alloc::EBox};

use super::{DetectResultError, DetectError};



const RESULT_CLASS_NAME: &'static str = class_name!("Result");
pub (crate) fn make_result_class() -> DynamicClass<()> {
    let mut class = DynamicClass::new(RESULT_CLASS_NAME);
    class.add_property("height", Visibility::Public, 0);
    class.add_property("width", Visibility::Public, 0);
    class.add_property("items", Visibility::Public, ());
    class.add_method(
        "__construct",
        Visibility::Public,
        |this, _| {
            this.set_property("items", Val::new( Array::new()));
        },
        vec![],
    );
    class
}

const RESULT_POINT_CLASS_NAME: &'static str = class_name!("ResultPoint");
pub (crate) fn make_result_point_class() -> DynamicClass<()> {
    let mut class = DynamicClass::new(RESULT_POINT_CLASS_NAME);
    class.add_property("x", Visibility::Public, 0);
    class.add_property("y", Visibility::Public, 0);
    class
}

const RESULT_ITEM_CLASS_NAME: &'static str = class_name!("ResultItem");
pub (crate) fn make_result_item_class() -> DynamicClass<()> {
    let mut class = DynamicClass::new(RESULT_ITEM_CLASS_NAME);
    class.add_property("type", Visibility::Public, 0);
    class.add_property("status", Visibility::Public, 1);
    class.add_property("error", Visibility::Public, ());
    class.add_property("text", Visibility::Public, ());
    class.add_property("points", Visibility::Public, ());
    class.add_method(
        "__construct",
        Visibility::Public,
        |this, _| {
            this.set_property("points", Val::new( Array::new()));
        },
        vec![],
    );
    class
}



pub (crate) fn create_result<DD:ResultData,DE:ToString>(res:Result<DD,ResultError<DE>>)->Result<EBox<Object<()>>,DetectError>{
    match res {
        Ok(data) =>{
            let mut out =
                StatelessClassEntry::from_globals(RESULT_CLASS_NAME)?
                    .new_object([])?;
            out.set_property("width", Val::new(data.info().width));
            out.set_property("height", Val::new(data.info().height));
        
            let mut item_data = Array::new();
        
            for (ii,item) in data.to_vec().iter().enumerate(){
                let mut out_item =
                    StatelessClassEntry::from_globals(RESULT_ITEM_CLASS_NAME)?
                        .new_object([])?;
                out_item.set_property("type", Val::new(item.get_type().to_int()));
                match item.get_data(){
                    Ok(dat)=>{
                        match String::from_utf8(dat.to_owned()) {
                            Ok(str)=>{
                                out_item.set_property("status", Val::new(1_i64));
                                out_item.set_property("text", Val::new(str));
                            }
                            Err(str)=>{
                                out_item.set_property("status", Val::new(0_i64));
                                out_item.set_property("error", Val::new(str.to_string()));
                            }
                        }
                    }
                    Err(str)=>{
                        out_item.set_property("status", Val::new(0_i64));
                        out_item.set_property("error", Val::new(str.to_owned()));
                    }
                }
                let mut point_data = Array::new();
                if let Some(points)=item.get_polygon() {
                    match points{
                        ResultPolygon::Point(tpoint)=>{
                            for (pi,point) in tpoint.iter().enumerate(){
                                let mut out_point =
                                    StatelessClassEntry::from_globals(RESULT_POINT_CLASS_NAME)?
                                        .new_object([])?;
                                out_point.set_property("x", Val::new(point.x));
                                out_point.set_property("y", Val::new(point.y));
                                point_data.insert(pi as u64, Val::new(out_point));
                            }
                        },
                    }
                }
                out_item.set_property("points", Val::new(point_data));
                item_data.insert(ii as u64, Val::new(out_item));
            }
            out.set_property("items", Val::new(item_data));
            Ok(out)
        },
        Err(err) =>{
            Err(DetectError::Detect(DetectResultError(err.to_string())))
        }
    }
}
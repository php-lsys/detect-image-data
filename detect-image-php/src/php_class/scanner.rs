#[cfg(feature = "zbar")]
use detect_image_data::ZBarImageScanner;
#[cfg(feature = "zxing")]
use detect_image_data::ZxingDecoder;
use phper::{classes::{ DynamicClass, Visibility}, objects::Object, values::Val, functions::Argument, alloc::EBox};
use detect_image_data::{DetectCode};
use super::{DetectError, result::create_result, rqrr::RqrrScanner, DetectResultError};

#[cfg(feature = "zbar")]
const ENGINE_ZBAR:i64 = 1;
const ENGINE_RQRR:i64 = 2;
#[cfg(feature = "zxing")]
const ENGINE_ZXING:i64 = 3;


const SCANNER_CLASS_NAME: &'static str = class_name!("Scanner");
pub (crate) fn make_scanner_class() -> DynamicClass<()> {
    let mut class = DynamicClass::new(SCANNER_CLASS_NAME);
    class.add_property("image", Visibility::Protected, ());
    class.add_property("engine", Visibility::Protected, 0_i64);
    #[cfg(feature = "zbar")]
    class.add_property("zbar_format", Visibility::Protected, 0_i64);
    #[cfg(feature = "zxing")]
    class.add_property("zxing_fast", Visibility::Protected, false);
    #[cfg(feature = "zxing")]
    class.add_property("zxing_norotate", Visibility::Protected, false);
    #[cfg(feature = "zxing")]
    class.add_property("zxing_ispure", Visibility::Protected, false);
    #[cfg(feature = "zxing")]
    class.add_property("zxing_desired_channels", Visibility::Protected, 4_i64);
    class.add_method(
        "__construct",
        Visibility::Public,
        |this, arguments| {
            let path=arguments[0].as_string()?;
            this.set_property("image", Val::new(path));
            Ok::<_, phper::Error>(())
        },
        vec![Argument::by_val("image_path")],
    );
    #[cfg(feature = "zbar")]
    class.add_method(
        "useZbarEngine",
        Visibility::Public,
        |this: &mut Object<()>, arguments: &mut [Val]| {
            this.set_property("zbar_format", Val::new(arguments.get(0).unwrap_or(&Val::new(0_i64)).as_long()?));
            this.set_property("engine", Val::new(ENGINE_ZBAR));
            Ok::<_, phper::Error>(unsafe {
                EBox::from_raw(this)
            })
        },
        vec![Argument::by_val_optional("format")],
    );
    #[cfg(feature = "zxing")]
    class.add_method(
        "useZxingEngine",
        Visibility::Public,
        |this: &mut Object<()>, arguments: &mut [Val]| {
            this.set_property("engine", Val::new(ENGINE_ZXING));
            this.set_property("zxing_fast",  Val::new(arguments.get(0).unwrap_or(&Val::new(false)).as_bool()?));
            this.set_property("zxing_norotate",  Val::new(arguments.get(1).unwrap_or(&Val::new(false)).as_bool()?));
            this.set_property("zxing_ispure",   Val::new(arguments.get(2).unwrap_or(&Val::new(false)).as_bool()?));
            this.set_property("zxing_desired_channels",   Val::new(arguments.get(3).unwrap_or(&Val::new(4_i64)).as_long()?));
            Ok::<_, phper::Error>(unsafe {
                EBox::from_raw(this)
            })
        },
        vec![Argument::by_val_optional("fast"),Argument::by_val_optional("norotate"),Argument::by_val_optional("ispure"),Argument::by_val_optional("desired_channels")],
    );
    class.add_method(
        "useQuircEngine",
        Visibility::Public,
        |this: &mut Object<()>, _: &mut [Val]| {
            this.set_property("engine", Val::new(ENGINE_RQRR));
            Ok::<_, phper::Error>(unsafe {
                EBox::from_raw(this)
            })
        },
        vec![],
    );
    class.add_method(
        "detect",
        Visibility::Public,
        |this: &mut Object<()>, _| {
            let path = this.get_property("image").as_str()?;
            let engine = this.get_property("engine").as_long()?;
            let out:Result<Result<EBox<Object<()>>,DetectError>,DetectError>=match engine {
                ENGINE_RQRR=>{
                    let sc=RqrrScanner{};
                    let res=sc.detect_scan(path);
                    Ok(create_result(res))
                },
                #[cfg(feature = "zbar")]
                ENGINE_ZBAR=>{
                    let mut sc=ZBarImageScanner::new();
                    let format=this.get_property("zbar_format").as_long()?;
                    if format>0 {
                        sc.format= format as u32;    
                    }
                    let res=sc.detect_scan(path);
                    Ok(create_result(res))
                },
                #[cfg(feature = "zxing")]
                ENGINE_ZXING=>{
                    let mut sc=ZxingDecoder::new();
                    sc.fast=this.get_property("zxing_fast").as_bool()?;
                    sc.norotate=this.get_property("zxing_norotate").as_bool()?;
                    sc.ispure=this.get_property("zxing_ispure").as_bool()?;
                    let chan=this.get_property("zxing_desired_channels").as_long()?;
                    if chan!=1&&chan!=2&&chan!=4 {
                        Err(DetectError::Detect(DetectResultError("desired_channels not support [1,2,4]".to_string())))
                    }else{
                        sc.desired_channels=chan as i8;
                        let res=sc.detect_scan(path);
                        Ok(create_result(res))
                    }
                },
                _=>{
                    return Err(DetectError::Detect(DetectResultError("type not match".to_string())))
                }
            };
            Ok::<_, DetectError>(out)
        },
        vec![],
    );
    class
}
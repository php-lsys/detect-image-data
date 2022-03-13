use phper::{
    modules::{Module},
    php_get_module,
};
mod php_class;

use php_class::make_exception_class;
use php_class::result::make_result_class;
use php_class::result::make_result_item_class;
use php_class::result::make_result_point_class;
use php_class::scanner::make_scanner_class;


#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
    module.add_class(make_exception_class());
    module.add_class(make_result_class());
    module.add_class(make_result_item_class());
    module.add_class(make_result_point_class());
    module.add_class(make_scanner_class());
    module
}


## zbar 的 RUST 封装

### 先安装 `zbar-devel`

> UBUNTU示例: apt install zbar-devel 

### 引入

```toml
[dependencies]
detect-image-data = { version = "0.0.0-beta.1", path = "../../",features=["zbar"],git = "https://github.com/shanliu/detect-image-data"  }
```


### 使用

> 示例: ../examples/zbar

```rust
use detect_image_data::{DetectCode, ResultData, ResultItem};
use detect_image_data::ZBarImageScanner;
fn main(){
    let zxing=ZBarImageScanner::new();
    let path = std::env::args().nth(1).expect("no image path given");
    let res=zxing.detect_scan(path.as_str()).unwrap();
    println!("count:{:?} size:{:?}",res.to_vec().len(),res.info());
    for item in res.to_vec().iter(){
        println!("{:?}",item.get_type());
        println!("{}",String::from_utf8(item.get_data().unwrap().to_owned()).unwrap());
        println!("{:?}",item.get_polygon());
    }
}
```

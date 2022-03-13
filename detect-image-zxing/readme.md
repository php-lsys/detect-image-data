
## zxing 的 RUST 封装

### 先安装 `libzxingcore-dev` 跟 `libstb-dev`

> UBUNTU示例: apt install libzxingcore-dev libstb-dev

### 引入

```toml
[dependencies]
detect-image-data = { version = "0.0.0-beta.1", path = "../../",features=["zxing"],git = "https://github.com/shanliu/detect-image-data"  }
```


### 使用

> 示例: ../examples/zxing

```rust
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
```

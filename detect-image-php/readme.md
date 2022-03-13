

### php[>7.0] 扩展封装 zbar zxing quirc 算法

> 默认 quirc 算法实现,不需要引入外部库

> zxing 依赖 `libzxingcore-dev` `libstb-dev` 库 UBUNTU安装示例: apt install libzxingcore-dev libstb-dev

> zbar 依赖 `zbar-devel` 库 UBUNTU安装示例: apt install zbar-devel 


### 安装PHP扩展,在 `./detect-image-php` 执行命令

1. 编译php扩展

```
cargo run  install # 仅安装 quirc 
cargo run  --features zbar,zxing install # 安装 quirc zxing zbar
```

2. 添加php.ini配置

> extension=detect-image-php.so

### php 调用示例

```php
try{
    $detect_data = (new \DetectImageCode\Scanner(__DIR__."/../../../examples/IMG_0184.jpg"))
    ->useQuircEngine()//使用 quirc 算法检测
    //->useZbarEngine()//使用 zxing 算法检测
    //->useZxingEngine()//使用 zbar 算法检测
    ->detect();
}catch (\DetectImageCode\Exception $e){
    print_r($e->getMessage());
    exit;
}
print_r($detect_data->items);
```
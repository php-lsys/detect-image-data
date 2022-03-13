<?php
//dome ...
if (!class_exists("DetectImageCode\Scanner")){
    die("plase install extension: cargo run install");
}

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);
try{
    $detect_data = (new \DetectImageCode\Scanner(__DIR__."/../../../examples/IMG_0184.jpg"))
    ->useQuircEngine()
    //->useZbarEngine()
    //->useZxingEngine()
    ->detect();
}catch (\DetectImageCode\Exception $e){
    print_r($e->getMessage());
    exit;
}
print_r($detect_data->items);
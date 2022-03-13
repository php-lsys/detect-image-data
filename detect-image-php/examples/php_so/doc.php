<?php
namespace DetectImageCode;
class ResultPoint
{
    /**
     * 坐标X轴
     * @var int
     */
    public $x = 0;

    /**
     * 坐标Y轴
     * @var int
     */
    public $y = 0;
}

class ResultItem
{
    /**
     * 结果包含的点坐标
     * @var ResultPoint[]
     */
    public $points = [];
    /**
     * 结果内容
     */
    public $text = "";
    /**
     * 结果类型
     */
    public $type = self::TYPE_NONE;
}

class Result
{

    /**
     * 结果列表
     * @var ResultItem[]
     */
    public $items = [];

    /**
     * 图片宽
     * @var integer
     */
    public $width = 0;

    /**
     * 图片高
     * @var integer
     */
    public $height = 0;
}
//配置或参数异常时抛出
class Exception extends \Exception
{
}

class Scanner
{

    /**
     * 检测指定路径图片中数据
     *
     * @param string $image_path
     *            图片路径
     */
    public function __construct($image_path){}
    /**
     * 使用 Zbar 算法检测
     * @return $this
     */
    public function useZbarEngine($format){}
    /**
     * 使用 Zxing 算法检测
     * @return $this
     */
    public function useZxingEngine($fast,$norotate,$ispure,$desired_channels){}
    /**
     * 使用 quirc 算法检测
     * @return $this
     */
    public function useQuircEngine(){}
    /**
     * 检测
     * @throws Exception
     * @return Result 检测结果
     */
    public function detect(){}
}

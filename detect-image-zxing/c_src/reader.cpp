

#include <iostream>
#include <cctype>
#include <chrono>
#include <clocale>
#include <cstring>
#include <memory>
#include <string>
#include <vector>
#include <ZXing/ReadBarcode.h>
#include <ZXing/TextUtfEncoding.h>
#include <ZXing/GTIN.h>
#include <stb/stb_image.h>

#include "reader.h"

using namespace ZXing;
using namespace TextUtfEncoding;
int zxing_read_qrcode(
        ZxingCResult **out_result,
        int *out_width,
        int *out_height,
        const char *path,
        int fast,
        int norotate,
        int ispure,
        int desired_channels
    ){
    int channels;
    std::unique_ptr<stbi_uc, void(*)(void*)> buffer(stbi_load(path, out_width, out_height, &channels, desired_channels), stbi_image_free);
    if (buffer == nullptr) {
        return -1;
    }
    DecodeHints hints;
	hints.setEanAddOnSymbol(EanAddOnSymbol::Read);
    hints.setTryHarder(fast?true:false);
    hints.setTryRotate(norotate?true:false);
    if (ispure)
    {
        hints.setIsPure(true);
        hints.setBinarizer(Binarizer::FixedThreshold);
    }else{
        hints.setIsPure(false);
    }
    auto fmt = ImageFormat::None;
	switch (desired_channels) {
        case 1: fmt = ImageFormat::Lum; break;
        case 3: fmt = ImageFormat::BGR; break;
        case 4: fmt = ImageFormat::BGRX; break;
	}
    ImageView image{buffer.get(), *out_width, *out_height,fmt};
    auto results = ReadBarcodes(image, hints);
    if (results.empty()){
        return -2;
    }
    ZxingCResult * prev_result = nullptr;
    for (auto&& result : results) {
        auto now_result = (ZxingCResult *)malloc(sizeof(ZxingCResult));
        now_result->num_bits = result.numBits();
        now_result->status = static_cast<int>(result.status());
        now_result->bytes = nullptr;
        now_result->bytes_size = 0;
        now_result->next = nullptr;
        if (result.status() == ZXing::DecodeStatus::NoError) {
            now_result->format = static_cast<int>(result.format());
            now_result->orientation = static_cast<double>(result.orientation());
            auto s =  ToUtf8(result.text(),false);
            now_result->bytes_size = sizeof(char) * s.size()+1;
            now_result->bytes =  (char *)malloc(now_result->bytes_size);
            std::memcpy(now_result->bytes, s.c_str(), now_result->bytes_size);
            now_result->bytes[now_result->bytes_size-1]='\0';
            auto pos=result.position();
            now_result->corners[0] = pos.topLeft().x;
            now_result->corners[1] = pos.topLeft().y;
            now_result->corners[2] = pos.topRight().x;
            now_result->corners[3] = pos.topRight().y;
            now_result->corners[4] = pos.bottomRight().x;
            now_result->corners[5] = pos.bottomRight().y;
            now_result->corners[6] = pos.bottomLeft().x;
            now_result->corners[7] = pos.bottomLeft().y;
            now_result->line_count = static_cast<int>(result.lineCount()); 
        }
        if (prev_result==nullptr){
            *out_result=now_result;
        }else{
            prev_result->next=now_result;
        }
        prev_result=now_result;
    }
    return 0;
}

int zxing_release_result(ZxingCResult *result)
{
    if(result->next!=nullptr){
        zxing_release_result(result->next);
        result->next=nullptr;
    }
    if (result->bytes != nullptr) {
        free(result->bytes);
        result->bytes=nullptr;
    }
    free(result);
    return 0;
}

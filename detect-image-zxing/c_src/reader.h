
#ifndef ZXING_C_READER_H
#define ZXING_C_READER_H

#include <cstdint>

typedef struct ZxingCResult {
    int status;
    int num_bits;
    int format;
    double orientation;
    int line_count;
    char * bytes;
    int bytes_size;
    int corners[8];
    uint width;
    uint height;
    struct ZxingCResult * next;
} ZxingCResult;

extern "C" {
    int zxing_read_qrcode(
        ZxingCResult **out_result,
        int *out_width,
        int *out_height,
        const char *path,
        int fast,
        int norotate,
        int ispure,
        int desired_channels
    );
    int zxing_release_result(ZxingCResult* result);
}

#endif //ZXING_C_READER_H

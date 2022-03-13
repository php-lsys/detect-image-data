#include <iostream>
#include <cctype>
#include <chrono>
#include <clocale>
#include <cstring>
#include <memory>
#include <string>
#include <vector>
#include "reader.h"
#include <stb/stb_image.h>

int main(int argc, char **argv){
    if (argc!=2){
         std::cout<< "plase input file path" <<std::endl;
         return 0;
    }
    int width=0, height=0; 
    auto result = (ZxingCResult **)malloc(sizeof(ZxingCResult **));
    int out=zxing_read_qrcode(result,&width,&height,argv[1],0,0,0,4);
    if (out==0)
    {
        auto res=*result;
        while (true)
        {   
            std::cout<<"Data:" << res->bytes <<std::endl;
            std::cout<<"Size:" << res->bytes_size <<std::endl;
            if(res->next==nullptr)break;
            res=res->next;
        }
        zxing_release_result(res);
    }else{
          std::cout<< "error:"<<out <<std::endl;
    }
}

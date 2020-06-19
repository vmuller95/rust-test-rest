#include "include/core/SkSurface.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkImage.h"

#include <iostream>
#include <fstream>


using namespace std;

static sk_sp<SkImage>
DecodeImage(vector<unsigned char> &data)
{
	unsigned char *imgBuffer = new unsigned char[data.size()];
	memcpy(imgBuffer, data.data(),
		   sizeof(*imgBuffer) * data.size());
	
	sk_sp<SkData> previewData = SkData::MakeWithoutCopy(imgBuffer, data.size());
	//sk_sp<SkPicture> picture = SkPicture::MakeFromData(pngData.get());
	return SkImage::MakeFromEncoded(previewData);
}

static vector<unsigned char>
ReadFile(const char *path)
{
    ifstream fs(path, std::ios::binary);
    vector<unsigned char> ret;
    const int bufsize = 1024;
    char *buffer = new char[bufsize];
    bool read = true;
    while (read) {
		fs.read(buffer, bufsize);
        size_t readReal = fs.gcount();

        if (readReal > 0) {
        	ret.insert(ret.end(), (unsigned char *)buffer,
        		(unsigned char *)buffer + readReal);
        }
        if (readReal < bufsize){
            read = false;
        }
    }

    return ret;
}

extern "C" void GeneratePreview(const char *inpath,  const char *outdir, const char *name)
{
    const int pw = 100;
    const int ph = 100;

    vector<unsigned char> imageBytes = ReadFile(inpath);
    sk_sp<SkImage> image = DecodeImage(imageBytes);
    SkImageInfo imageInfo = SkImageInfo::Make(pw, ph, kRGBA_8888_SkColorType, kPremul_SkAlphaType);
	sk_sp<SkSurface> previewSurface = SkSurface::MakeRaster(imageInfo);
	SkCanvas *previewCanvas = previewSurface->getCanvas();

    SkPaint paint;
    SkRect src = SkRect::MakeXYWH(0, 0, image->width(), image->height());
    SkRect dest = SkRect::MakeXYWH(0, 0, pw, ph);
    previewCanvas->drawImageRect(image, src, dest, &paint);

    sk_sp<SkImage> saveImage = previewSurface->makeImageSnapshot();
    auto saveData = saveImage->encodeToData(SkEncodedImageFormat::kJPEG, 100);

    string outpath = string(outdir) + "/" + string(name);
    std::fstream fs;
		fs.open (outpath,
				 std::fstream::out | std::fstream::trunc);
		auto data = (const char *)saveData->data();
		size_t size = saveData->size();
		fs.write(data, size);
		fs.close();
}

#ifdef MAIN
int 
main()
{
    GeneratePreview("images/test1.jpg", "previews/", "test-check.jpg");
    return 0;
}
#endif
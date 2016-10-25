#include "test.hpp"
#include <assert.h>
#include "core.hpp"

typedef BytesC<char, 3> XBytes;

extern "C" {
    XBytes call_me2(void* ptr, char param1, char param2, char param3) {
        assert(ptr != 0);
    	XBytes b = XBytes(param1, param2, param3);
    	return b;
    }

    cv::Vec3b call_me3(void* ptr, char param1, char param2, char param3) {
        assert(ptr != 0);
        cv::Vec3b b = cv::Vec3b(param1, param2, param3);
        return b;
    }
}

extern crate gcc;

fn main() {
    gcc::Config::new().cpp(true).file("native/test.cpp").include("/usr/include/opencv2/core/").compile("libtest.a")
}
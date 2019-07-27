#![feature(proc_macro_hygiene, decl_macro)]

use qrcode::QrCode;
use image::Luma;
//use qr2term::print_qr;

fn main() {

    let code = QrCode::new("http://192.168.1.32:8000").unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("/tmp/qrcode.png").unwrap();

    //print_qr("http://192.168.1.32:8000");

}

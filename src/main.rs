// #[link(name = "libcrypt256")]
// extern "C" {
//     pub fn crypt_sha256(phrase: *const libc::c_char, setting: *const libc::c_char, output: *const libc::c_char);
// }

use std::ffi::{CString, CStr};

use  crypt3_sys::crypt_sha256;
fn main() {
    let password = "mypassword";
    let sha256_setting = "$5$";
    let salt = "rDxsrps6AeTwJLRK";
    let settings = format!("{sha256_setting}{salt}");

    let mut output = vec![0_i8; 64];

    let ret_str = unsafe {
        // put phrase to cstring called ret
        let csetting = CString::new(settings).unwrap();
        let cpassword = CString::new(password).unwrap();

        let ret = crypt_sha256(cpassword.as_ptr(), csetting.as_ptr(), output.as_mut_ptr());

        let ret_cstr = CStr::from_ptr(ret);
        ret_cstr.to_str().unwrap()
    };

    // mkpasswd -m sha256crypt "mypassword"
    // $5$rDxsrps6AeTwJLRK$CHafsXkpg6bi5Z.kdTYhlWzmhqe9Q.RRPm0LWi/bckC

    let ret_assumed = format!(
        "{}{}${}",
        sha256_setting, salt, "CHafsXkpg6bi5Z.kdTYhlWzmhqe9Q.RRPm0LWi/bckC"
    );
    assert!(ret_str == ret_assumed);

    println!("ret_str {ret_str:?}");
}

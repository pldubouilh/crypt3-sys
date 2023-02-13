# crypt3-sys

statically linkable bindings against locally pulled crypt(3) from musl.

[crate.io](https://crates.io/crates/crypt3-sys)

```rust
use std::ffi::{CString, CStr};

fn main() {
    let password = "mypassword";
    let sha256_setting = "$5$";
    let salt = "rDxsrps6AeTwJLRK";
    let settings = format!("{sha256_setting}{salt}");

    let mut output = vec![0_i8; 256];

    let ret_str = unsafe {
        let csetting = CString::new(settings).unwrap();
        let cpassword = CString::new(password).unwrap();

        let _ret = crypt3_sys::crypt_r(cpassword.as_ptr(), csetting.as_ptr(), output.as_mut_ptr());

        let ret_cstr = CStr::from_ptr(output.as_ptr());
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
```

use super::libgbm;
use libc;

pub struct GbmDevice {
    lib: libgbm::LibGbm,
    device: *mut libc::c_void,
    //surface: *mut libc::c_void,
    //egl: Context,
}

impl GbmDevice {
    pub fn open() -> GbmDevice {
        let lib = libgbm::LibGbm::open().unwrap();

        let device = unsafe { 
            let fd = libc::open(b"/dev/dri/card0\0".as_ptr() as *const _, libc::O_RDWR, 0);

            if fd < 0 {
                panic!("Failed to open /dev/dri/card0");
            }

            let device = lib.create_device(fd);
            if device.is_null() {
                panic!("gbm_create_device failed");
            }
            device
        };

        GbmDevice {
            lib: lib,
            device: device,
        }
    }
}

impl Drop for GbmDevice {
    fn drop(&mut self) {
        unsafe {
            self.lib.device_destroy(self.device)
        };
    }
}

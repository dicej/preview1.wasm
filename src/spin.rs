#[link(wasm_import_module = "__main_module__")]
extern "C" {
    pub fn canonical_abi_realloc(
        old_ptr: *mut u8,
        old_len: usize,
        align: usize,
        new_len: usize,
    ) -> *mut u8;
}

unsafe fn dealloc(ptr: i32, size: usize, align: usize) {
    #[link(wasm_import_module = "__main_module__")]
    extern "C" {
        fn canonical_abi_free(ptr: *mut u8, len: usize, align: usize);
    }

    canonical_abi_free(ptr as _, size, align);
}

#[export_name = "request"]
unsafe extern "C" fn __wit_bindgen_wasi_outbound_http_request(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    arg6: i32,
    arg7: i32,
    arg8: i32,
    arg9: i32,
    arg10: i32,
) {
    #[link(wasm_import_module = "outbound-http")]
    extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "send-request")]
        fn wit_import(
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
        );
    }
    wit_import(
        arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10,
    );
}

#[doc(hidden)]
#[export_name = "inbound-http#handle-request"]
#[allow(non_snake_case)]
unsafe extern "C" fn __export_inbound_http_handle_request(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    arg6: i32,
    arg7: i32,
    arg8: i32,
    arg9: i32,
) -> i32 {
    #[link(wasm_import_module = "__main_module__")]
    extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "handle-http-request")]
        fn wit_import(
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
        ) -> i32;
    }

    let mut ret = 0;
    super::State::with(|state| {
        ret = state
            .import_alloc
            .with_main(|| wit_import(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9));
        Ok(())
    });
    ret
}

#[doc(hidden)]
#[export_name = "cabi_post_inbound-http#handle-request"]
#[allow(non_snake_case)]
unsafe extern "C" fn __post_return_inbound_http_handle_request(arg0: i32) {
    match i32::from(*((arg0 + 4) as *const u8)) {
        0 => (),
        _ => {
            let base0 = *((arg0 + 8) as *const i32);
            let len0 = *((arg0 + 12) as *const i32);
            for i in 0..len0 {
                let base = base0 + i * 16;
                {
                    dealloc(
                        *((base + 0) as *const i32),
                        (*((base + 4) as *const i32)) as usize,
                        1,
                    );
                    dealloc(
                        *((base + 8) as *const i32),
                        (*((base + 12) as *const i32)) as usize,
                        1,
                    );
                }
            }
            dealloc(base0, (len0 as usize) * 16, 4);
        }
    }
    match i32::from(*((arg0 + 16) as *const u8)) {
        0 => (),
        _ => {
            let base1 = *((arg0 + 20) as *const i32);
            let len1 = *((arg0 + 24) as *const i32);
            dealloc(base1, (len1 as usize) * 1, 1);
        }
    }
}

use {
    crate::{Descriptor, StreamType, Streams},
    core::{arch::wasm32, cell::Cell},
};

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

fn init() {
    super::State::with_mut(|state| {
        let descriptors = state.descriptors_mut();
        if descriptors.len() < 3 {
            wasm32::unreachable()
        }
        descriptors[0] = Descriptor::Streams(Streams {
            input: Cell::new(Some(0)),
            output: Cell::new(None),
            type_: StreamType::Unknown,
        });
        descriptors[1] = Descriptor::Streams(Streams {
            input: Cell::new(None),
            output: Cell::new(Some(1)),
            type_: StreamType::Unknown,
        });
        descriptors[2] = Descriptor::Streams(Streams {
            input: Cell::new(None),
            output: Cell::new(Some(2)),
            type_: StreamType::Unknown,
        });

        Ok(())
    });
}

#[doc(hidden)]
#[export_name = "inbound-redis#handle-message"]
#[allow(non_snake_case)]
unsafe extern "C" fn inbound_redis_handle_message(arg0: i32, arg1: i32) -> i32 {
    #[link(wasm_import_module = "__main_module__")]
    extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "handle-redis-message")]
        fn wit_import(_: i32, _: i32) -> i32;
    }

    init();
    wit_import(arg0, arg1)
}

#[doc(hidden)]
#[export_name = "inbound-http#handle-request"]
#[allow(non_snake_case)]
unsafe extern "C" fn inbound_http_handle_request(
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

    init();
    wit_import(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

#[doc(hidden)]
#[export_name = "cabi_post_inbound-http#handle-request"]
#[allow(non_snake_case)]
unsafe extern "C" fn post_return_inbound_http_handle_request(arg0: i32) {
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

macro_rules! export {
    ($export_name:literal $name:ident $import_module:literal $import_name:literal $( $arg:ident )*) => {
        #[export_name = $export_name]
        unsafe extern "C" fn $name($( $arg: i32 ),*) {
            #[link(wasm_import_module = $import_module)]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = $import_name)]
                fn wit_import($( $arg: i32 ),*);
            }
            super::State::with(|state| {
                state.import_alloc.with_main(|| wit_import($( $arg ),*));
                Ok(())
            });
        }
    }
}

export!("wasi-outbound-http:request" wasi_outbound_http_request "http" "send-request"
        a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10);

export!("spin-config:get-config" config_get_config "config" "get-config"
        a0 a1 a2);

export!("outbound-redis:publish" outbound_redis_publish "redis" "publish"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-redis:set" outbound_redis_set "redis" "set"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-redis:get" outbound_redis_get "redis" "get"
        a0 a1 a2 a3 a4);

export!("outbound-redis:incr" outbound_redis_incr "redis" "incr"
        a0 a1 a2 a3 a4);

export!("outbound-redis:del" outbound_redis_del "redis" "del"
        a0 a1 a2 a3 a4);

export!("outbound-redis:sadd" outbound_redis_sadd "redis" "sadd"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-redis:smembers" outbound_redis_smembers "redis" "smembers"
        a0 a1 a2 a3 a4);

export!("outbound-redis:srem" outbound_redis_srem "redis" "srem"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-redis:execute" outbound_redis_execute "redis" "execute"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-pg:query" outbound_pg_query "postgres" "query"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-pg:execute" outbound_pg_execute "postgres" "execute"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-mysql:query" outbound_mysql_query "mysql" "query"
        a0 a1 a2 a3 a4 a5 a6);

export!("outbound-mysql:execute" outbound_mysql_execute "mysql" "execute"
        a0 a1 a2 a3 a4 a5 a6);

export!("key-value:open" key_value_open "key-value" "open"
        a0 a1 a2);

export!("key-value:get" key_value_get "key-value" "get"
        a0 a1 a2 a3);

export!("key-value:set" key_value_set "key-value" "set"
        a0 a1 a2 a3 a4 a5);

export!("key-value:delete" key_value_delete "key-value" "delete"
        a0 a1 a2 a3);

export!("key-value:exists" key_value_exists "key-value" "exists"
        a0 a1 a2 a3);

export!("key-value:get-keys" key_value_get_keys "key-value" "get-keys"
        a0 a1);

export!("key-value:close" key_value_close "key-value" "close"
        a0);

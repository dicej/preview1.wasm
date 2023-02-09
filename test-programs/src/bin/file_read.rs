use std::{
    error::Error,
    fs::File,
    io::{self, Seek, SeekFrom},
};

#[export_name = "cabi_realloc"]
#[no_mangle]
unsafe extern "C" fn cabi_realloc(
    old_ptr: *mut u8,
    old_len: usize,
    align: usize,
    new_len: usize,
) -> *mut u8 {
    use std::alloc::{self, Layout};

    let layout;
    let ptr = if old_len == 0 {
        if new_len == 0 {
            return align as *mut u8;
        }
        layout = Layout::from_size_align_unchecked(new_len, align);
        alloc::alloc(layout)
    } else {
        debug_assert_ne!(new_len, 0, "non-zero old_len requires non-zero new_len!");
        layout = Layout::from_size_align_unchecked(old_len, align);
        alloc::realloc(old_ptr, layout, new_len)
    };
    if ptr.is_null() {
        core::arch::wasm32::unreachable();
    }
    return ptr;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("bar.txt")?;

    assert_eq!(27, file.metadata()?.len());

    assert_eq!(
        "And stood awhile in thought",
        &io::read_to_string(&mut file)?
    );

    file.seek(SeekFrom::Start(11))?;

    assert_eq!("while in thought", &io::read_to_string(&mut file)?);

    Ok(())
}

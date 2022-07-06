use bkshared::graphics::{Framebuffer, PixelFormat};
use efi::{println, proto::Protocol, raw::text::TextOutput, status::EfiResult, GraphicsOutput};

pub fn find_framebuffer() -> EfiResult<Framebuffer> {
    let gop = GraphicsOutput::locate()?;
    Ok(Framebuffer {
        format: PixelFormat::BitMask, // TODO
        base: gop.0.mode.base,
        size: gop.0.mode.size,
        width: gop.0.mode.info.horizontal_res as u64,
        height: gop.0.mode.info.vertical_res as u64,
        stride: gop.0.mode.info.stride as u64,
    })
}

/// Search for the maximum resolution and use it
pub fn make_max_mode(proto: &TextOutput) -> EfiResult<()> {
    let (mut max_width, mut max_height, mut index_of_max): (usize, usize, isize) = (0, 0, -1);
    for i in 0..proto.mode.max {
        let mut width = 0;
        let mut height = 0;
        if (proto.query_mode)(proto, i as usize, &mut width, &mut height)
            .result()
            .is_ok()
        {
            if width >= max_width && height >= max_height {
                index_of_max = i as isize;
                max_width = width;
                max_height = height;
            }
        }
    }

    if index_of_max >= 0 {
        (proto.set_mode)(proto, index_of_max as usize).result()?
    }

    println!(
        "Found maximum mode at {}; Width {}; Height {};",
        index_of_max, max_width, max_height
    );
    Ok(())
}

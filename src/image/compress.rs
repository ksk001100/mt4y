use byte_unit::Byte;
use image::{self, imageops::FilterType::Lanczos3, DynamicImage, RgbImage};
use mozjpeg::{ColorSpace, Compress, Decompress, Marker, ScanMode, ALL_MARKERS};
use seahorse::{Command, Context, Flag, FlagType};
use std::fs;

pub fn command() -> Command {
    Command::new("compress")
        .alias("c")
        .usage(format!(
            "{} image compress [input file] [flags]",
            env!("CARGO_PKG_NAME")
        ))
        .description("Compression command")
        .flag(
            Flag::new("width", FlagType::Int)
                .alias("w")
                .description("Resize width"),
        )
        .flag(
            Flag::new("height", FlagType::Int)
                .alias("h")
                .description("Resize height"),
        )
        .flag(
            Flag::new("quality", FlagType::Float)
                .alias("q")
                .description("Image quality"),
        )
        .flag(
            Flag::new("output", FlagType::String)
                .alias("o")
                .description("Output file name"),
        )
        .action(action)
}

fn action(c: &Context) {
    let input_file = &c.args[0];

    let output = c
        .string_flag("output")
        .unwrap_or_else(|_| "output.jpg".to_string());

    let quality = c.float_flag("quality").unwrap_or_else(|_| 70.0) as f32;

    let raw_data = fs::read(input_file).unwrap();
    let decomp = Decompress::with_markers(ALL_MARKERS)
        .from_mem(&raw_data)
        .unwrap();

    let markers: Vec<(Marker, Vec<u8>)> = decomp
        .markers()
        .into_iter()
        .map(|m| (m.marker, m.data.to_owned()))
        .collect();

    let mut decomp_started = decomp.rgb().unwrap();

    let width = decomp_started.width();
    let height = decomp_started.height();

    let file_size = Byte::from(raw_data.len())
        .get_appropriate_unit(false)
        .to_string();

    println!(
        "===Start compression===\nPath: {}\nSize: {}x{}\nFileSize: {}",
        &input_file, &width, &height, file_size
    );

    let data = decomp_started
        .read_scanlines::<[u8; 3]>()
        .unwrap()
        .iter()
        .flatten()
        .cloned()
        .collect();

    decomp_started.finish_decompress();

    let output_width = c.int_flag("width").unwrap_or_else(|_| width as isize) as u32;
    let output_height = c.int_flag("height").unwrap_or_else(|_| height as isize) as u32;

    let image_buffer = RgbImage::from_raw(width as u32, height as u32, data).unwrap();
    let img = DynamicImage::ImageRgb8(image_buffer);

    let img = img
        .resize(output_width, output_height, Lanczos3)
        .unsharpen(0.5, 10);

    let width = img.width() as usize;
    let height = img.height() as usize;

    let data = img.to_rgb8().to_vec();

    let mut comp = Compress::new(ColorSpace::JCS_RGB);
    comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
    comp.set_quality(quality);
    comp.set_size(width, height);
    comp.set_mem_dest();
    comp.start_compress();

    markers.into_iter().for_each(|m| {
        comp.write_marker(m.0, &m.1);
    });

    let mut line = 0;
    loop {
        if line > height - 1 {
            break;
        }
        let buf = unsafe { data.get_unchecked(line * width * 3..(line + 1) * width * 3) };
        comp.write_scanlines(buf);
        line += 1;
    }

    comp.finish_compress();

    let buf = comp.data_to_vec().unwrap();
    fs::write(&output, &buf).unwrap();

    let file_size = Byte::from(buf.len())
        .get_appropriate_unit(false)
        .to_string();

    println!(
        "\n===Finish compression===\nPath: {}\nSize: {}x{}\nFileSize: {}",
        &output, &width, &height, file_size
    );
}

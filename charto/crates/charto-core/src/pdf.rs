//! Print-ready PDF output.
//!
//! The GUI renders the chart to a PNG image (exactly what is on screen,
//! symbols and labels included); this module decodes it, places it on one
//! page of the paper size chosen in the print dialog and writes a minimal
//! PDF 1.4 file containing the image. No Qt, no external PDF library.

use std::io::Write;

const PT_PER_MM: f64 = 72.0 / 25.4;

/// 8-bit RGB pixels, row by row, without alpha.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbImage {
    pub width: u32,
    pub height: u32,
    pub rgb: Vec<u8>,
}

/// Where the image goes on the page, in PDF points (origin bottom left).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub page_width_pt: f64,
    pub page_height_pt: f64,
    pub x_pt: f64,
    pub y_pt: f64,
    pub width_pt: f64,
    pub height_pt: f64,
}

fn over_white(channel: u8, alpha: u8) -> u8 {
    let (c, a) = (channel as u32, alpha as u32);
    ((c * a + 255 * (255 - a) + 127) / 255) as u8
}

/// Decodes a PNG and flattens transparency onto white paper.
pub fn decode_png_rgb(bytes: &[u8]) -> Result<RgbImage, String> {
    let mut decoder = png::Decoder::new(bytes);
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().map_err(|e| format!("not a valid PNG image: {e}"))?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| format!("cannot decode PNG image: {e}"))?;
    if info.bit_depth != png::BitDepth::Eight {
        return Err("unsupported PNG bit depth".into());
    }
    let data = &buf[..info.buffer_size()];
    let mut rgb = Vec::with_capacity(info.width as usize * info.height as usize * 3);
    match info.color_type {
        png::ColorType::Rgba => {
            for px in data.chunks_exact(4) {
                let a = px[3];
                rgb.extend_from_slice(&[over_white(px[0], a), over_white(px[1], a), over_white(px[2], a)]);
            }
        }
        png::ColorType::Rgb => rgb.extend_from_slice(data),
        png::ColorType::Grayscale => {
            for &g in data {
                rgb.extend_from_slice(&[g, g, g]);
            }
        }
        png::ColorType::GrayscaleAlpha => {
            for px in data.chunks_exact(2) {
                let g = over_white(px[0], px[1]);
                rgb.extend_from_slice(&[g, g, g]);
            }
        }
        png::ColorType::Indexed => return Err("unsupported indexed PNG image".into()),
    }
    if info.width == 0 || info.height == 0 || rgb.len() != info.width as usize * info.height as usize * 3 {
        return Err("the PNG image is empty or damaged".into());
    }
    Ok(RgbImage { width: info.width, height: info.height, rgb })
}

/// Fits an image of `image_w` x `image_h` pixels onto one page, centred,
/// inside `margin_mm`. The page orientation follows the image: wide charts
/// get a landscape page, tall ones a portrait page, whatever orientation the
/// paper was given in.
pub fn layout(image_w: u32, image_h: u32, paper_w_mm: f64, paper_h_mm: f64, margin_mm: f64) -> Placement {
    let long = paper_w_mm.max(paper_h_mm);
    let short = paper_w_mm.min(paper_h_mm);
    let (page_w, page_h) = if image_w >= image_h { (long, short) } else { (short, long) };

    // keep at least 10 mm of printable area in each direction
    let margin = margin_mm.max(0.0).min(((page_w.min(page_h)) - 10.0).max(0.0) / 2.0);
    let avail_w = page_w - 2.0 * margin;
    let avail_h = page_h - 2.0 * margin;

    let scale = (avail_w / image_w.max(1) as f64).min(avail_h / image_h.max(1) as f64);
    let w = image_w as f64 * scale;
    let h = image_h as f64 * scale;
    let x = margin + (avail_w - w) / 2.0;
    let y_from_top = margin + (avail_h - h) / 2.0;

    Placement {
        page_width_pt: page_w * PT_PER_MM,
        page_height_pt: page_h * PT_PER_MM,
        x_pt: x * PT_PER_MM,
        y_pt: (page_h - y_from_top - h) * PT_PER_MM,
        width_pt: w * PT_PER_MM,
        height_pt: h * PT_PER_MM,
    }
}

fn push_object(out: &mut Vec<u8>, offsets: &mut Vec<usize>, body: &[u8]) {
    offsets.push(out.len());
    let number = offsets.len();
    out.extend_from_slice(format!("{number} 0 obj\n").as_bytes());
    out.extend_from_slice(body);
    out.extend_from_slice(b"\nendobj\n");
}

/// Writes a one-page PDF with `img` drawn at `place`.
pub fn build_pdf(img: &RgbImage, place: &Placement) -> Result<Vec<u8>, String> {
    if img.rgb.len() != img.width as usize * img.height as usize * 3 {
        return Err("image size does not match its pixel data".into());
    }
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    encoder.write_all(&img.rgb).map_err(|e| e.to_string())?;
    let compressed = encoder.finish().map_err(|e| e.to_string())?;

    let content = format!(
        "q {:.3} 0 0 {:.3} {:.3} {:.3} cm /Im0 Do Q\n",
        place.width_pt, place.height_pt, place.x_pt, place.y_pt
    );

    let mut out: Vec<u8> = Vec::new();
    let mut offsets: Vec<usize> = Vec::new();
    out.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    push_object(&mut out, &mut offsets, b"<< /Type /Catalog /Pages 2 0 R >>");
    push_object(&mut out, &mut offsets, b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>");
    push_object(
        &mut out,
        &mut offsets,
        format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {:.2} {:.2}] \
             /Resources << /XObject << /Im0 4 0 R >> >> /Contents 5 0 R >>",
            place.page_width_pt, place.page_height_pt
        )
        .as_bytes(),
    );

    let mut image_obj = format!(
        "<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB \
         /BitsPerComponent 8 /Filter /FlateDecode /Length {} >>\nstream\n",
        img.width,
        img.height,
        compressed.len()
    )
    .into_bytes();
    image_obj.extend_from_slice(&compressed);
    image_obj.extend_from_slice(b"\nendstream");
    push_object(&mut out, &mut offsets, &image_obj);

    push_object(
        &mut out,
        &mut offsets,
        format!("<< /Length {} >>\nstream\n{}endstream", content.len(), content).as_bytes(),
    );

    let xref_pos = out.len();
    let count = offsets.len() + 1;
    out.extend_from_slice(format!("xref\n0 {count}\n0000000000 65535 f \n").as_bytes());
    for off in &offsets {
        out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!("trailer\n<< /Size {count} /Root 1 0 R >>\nstartxref\n{xref_pos}\n%%EOF\n").as_bytes(),
    );
    Ok(out)
}

/// PNG bytes -> PDF bytes for the given paper (millimetres) and margin.
pub fn png_to_pdf(png: &[u8], paper_w_mm: f64, paper_h_mm: f64, margin_mm: f64) -> Result<Vec<u8>, String> {
    let img = decode_png_rgb(png)?;
    let place = layout(img.width, img.height, paper_w_mm, paper_h_mm, margin_mm);
    build_pdf(&img, &place)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_bytes(width: u32, height: u32, color: png::ColorType, data: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, width, height);
            encoder.set_color(color);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(data).unwrap();
        }
        bytes
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn decodes_rgba_and_flattens_transparency_onto_white() {
        let data = [
            255, 0, 0, 255, // opaque red
            0, 0, 0, 0, // fully transparent -> white
            0, 0, 255, 128, // half transparent blue
            0, 255, 0, 255, // opaque green
        ];
        let img = decode_png_rgb(&png_bytes(2, 2, png::ColorType::Rgba, &data)).unwrap();
        assert_eq!((img.width, img.height), (2, 2));
        assert_eq!(&img.rgb[0..3], &[255, 0, 0]);
        assert_eq!(&img.rgb[3..6], &[255, 255, 255]);
        assert!((img.rgb[6] as i32 - 127).abs() <= 1);
        assert_eq!(img.rgb[8], 255);
        assert_eq!(&img.rgb[9..12], &[0, 255, 0]);
    }

    #[test]
    fn decodes_plain_rgb_and_gray() {
        let rgb = decode_png_rgb(&png_bytes(1, 1, png::ColorType::Rgb, &[1, 2, 3])).unwrap();
        assert_eq!(rgb.rgb, vec![1, 2, 3]);
        let gray = decode_png_rgb(&png_bytes(2, 1, png::ColorType::Grayscale, &[10, 20])).unwrap();
        assert_eq!(gray.rgb, vec![10, 10, 10, 20, 20, 20]);
    }

    #[test]
    fn rejects_garbage() {
        assert!(decode_png_rgb(b"definitely not a png").is_err());
        assert!(decode_png_rgb(&[]).is_err());
    }

    #[test]
    fn wide_chart_gets_a_centred_landscape_page() {
        let p = layout(200, 100, 210.0, 297.0, 10.0);
        let mm = 72.0 / 25.4;
        assert!(close(p.page_width_pt, 297.0 * mm));
        assert!(close(p.page_height_pt, 210.0 * mm));
        assert!(close(p.x_pt, 10.0 * mm));
        assert!(close(p.width_pt, 277.0 * mm));
        assert!(close(p.height_pt, 138.5 * mm));
        assert!(close(p.y_pt, 35.75 * mm));
    }

    #[test]
    fn tall_chart_gets_a_portrait_page_whatever_the_paper_orientation() {
        let p = layout(100, 200, 297.0, 210.0, 10.0); // paper given as landscape
        let mm = 72.0 / 25.4;
        assert!(close(p.page_width_pt, 210.0 * mm));
        assert!(close(p.page_height_pt, 297.0 * mm));
        // 190 x 277 available, image 1:2 -> limited by width 190? no: 277/200 < 190/100
        assert!(close(p.height_pt, 277.0 * mm));
        assert!(close(p.width_pt, 138.5 * mm));
        assert!(close(p.x_pt, (10.0 + (190.0 - 138.5) / 2.0) * mm));
    }

    #[test]
    fn huge_margins_are_limited_to_keep_printable_area() {
        let p = layout(100, 100, 100.0, 100.0, 80.0);
        assert!(p.width_pt > 0.0 && p.height_pt > 0.0);
        assert!(p.x_pt >= 0.0 && p.y_pt >= 0.0);
        assert!(p.x_pt + p.width_pt <= p.page_width_pt + 1e-6);
    }

    #[test]
    fn pdf_has_a_valid_cross_reference_table() {
        let data = vec![200u8; 4 * 3 * 3];
        let pdf = png_to_pdf(&png_bytes(4, 3, png::ColorType::Rgb, &data), 210.0, 297.0, 10.0).unwrap();
        assert!(pdf.starts_with(b"%PDF-1.4\n"));
        assert!(pdf.ends_with(b"%%EOF\n"));

        let text = String::from_utf8_lossy(&pdf).into_owned();
        let start = text.rfind("startxref\n").unwrap() + "startxref\n".len();
        let xref_pos: usize = text[start..].lines().next().unwrap().trim().parse().unwrap();
        assert!(pdf[xref_pos..].starts_with(b"xref\n0 6\n"));

        // every in-use entry points at "<n> 0 obj"
        let table = &pdf[xref_pos + "xref\n0 6\n".len()..];
        for n in 0..6usize {
            let entry = &table[n * 20..n * 20 + 20];
            assert_eq!(entry.len(), 20);
            assert!(entry.ends_with(b" \n"), "entry {n} must be 20 bytes");
            if n == 0 {
                assert!(entry.starts_with(b"0000000000 65535 f"));
                continue;
            }
            let off: usize = std::str::from_utf8(&entry[..10]).unwrap().parse().unwrap();
            let expected = format!("{n} 0 obj\n");
            assert!(pdf[off..].starts_with(expected.as_bytes()), "object {n} at {off}");
        }
        assert!(text.contains("/Subtype /Image"));
        assert!(text.contains("/Width 4 /Height 3"));
    }

    #[test]
    fn mismatched_pixel_data_is_an_error() {
        let img = RgbImage { width: 2, height: 2, rgb: vec![0; 5] };
        let place = layout(2, 2, 210.0, 297.0, 10.0);
        assert!(build_pdf(&img, &place).is_err());
    }
}

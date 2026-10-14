//! Printing through the freedesktop Print portal
//! (`org.freedesktop.portal.Print`, see the xdg-desktop-portal docs).
//!
//! Flow:
//! 1. `PreparePrint` shows the desktop's own print dialog (GTK, KDE, ...)
//!    and returns the chosen printer settings and paper size;
//! 2. the chart image is laid out on that paper and written as a PDF;
//! 3. `Print` hands the PDF to the portal, which sends it to the printer.
//!
//! If there is no print portal (minimal desktop, no portal service) the PDF
//! is opened with `xdg-open` instead, i.e. the user's default PDF viewer,
//! from which it can be printed.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use ashpd::desktop::print::{PageSetup, PreparePrintOptions, PrintOptions, PrintProxy, Settings};
use charto_core::pdf::{self, RgbImage};

const DEFAULT_PAPER_MM: (f64, f64) = (210.0, 297.0); // A4
const DEFAULT_MARGIN_MM: f64 = 10.0;

enum PortalError {
    /// The user closed the print dialog.
    Cancelled,
    /// The portal cannot be reached or does not offer printing.
    Unavailable(String),
    /// Printing was attempted and failed.
    Failed(String),
}

/// Prints the PNG rendering of the chart. Returns the message to show in
/// the status bar.
pub fn print_png(png_path: &Path, title: &str) -> Result<String, String> {
    let png = std::fs::read(png_path).map_err(|e| format!("cannot read the rendered chart: {e}"))?;
    let image = pdf::decode_png_rgb(&png)?;
    let title = if title.trim().is_empty() { "Charto pattern" } else { title };

    match pollster::block_on(portal_print(&image, title)) {
        Ok(message) => Ok(message),
        Err(PortalError::Cancelled) => Ok("Printing cancelled".to_string()),
        Err(PortalError::Unavailable(why)) => open_in_viewer(&image, &why),
        Err(PortalError::Failed(why)) => Err(why),
    }
}

fn is_cancel(text: &str) -> bool {
    text.to_lowercase().contains("cancel")
}

/// Paper size in millimetres from the dialog's answer, as (width, height).
fn paper_mm(setup: &PageSetup, settings: &Settings) -> (f64, f64) {
    let from_setup = setup.width.zip(setup.height);
    let from_settings = settings
        .paper_width
        .as_deref()
        .and_then(|w| w.parse::<f64>().ok())
        .zip(settings.paper_height.as_deref().and_then(|h| h.parse::<f64>().ok()));
    match from_setup.or(from_settings) {
        Some((w, h)) if w > 0.0 && h > 0.0 => (w, h),
        _ => DEFAULT_PAPER_MM,
    }
}

fn margin_mm(setup: &PageSetup) -> f64 {
    [setup.margin_left, setup.margin_right, setup.margin_top, setup.margin_bottom]
        .iter()
        .flatten()
        .copied()
        .fold(DEFAULT_MARGIN_MM, f64::max)
        .min(30.0)
}

/// Writes the PDF to a temporary file, reopens it for reading and removes
/// the name again: the open descriptor is all the portal needs.
fn pdf_descriptor(bytes: &[u8]) -> Result<File, String> {
    let path = std::env::temp_dir().join(format!("charto-print-{}.pdf", std::process::id()));
    let mut out = File::create(&path).map_err(|e| format!("cannot write the print file: {e}"))?;
    out.write_all(bytes).map_err(|e| format!("cannot write the print file: {e}"))?;
    drop(out);
    let file = File::open(&path).map_err(|e| format!("cannot read the print file: {e}"))?;
    let _ = std::fs::remove_file(&path);
    Ok(file)
}

async fn portal_print(image: &RgbImage, title: &str) -> Result<String, PortalError> {
    let proxy = PrintProxy::new()
        .await
        .map_err(|e| PortalError::Unavailable(e.to_string()))?;

    let request = proxy
        .prepare_print(
            None,
            title,
            Settings::default(),
            PageSetup::default(),
            PreparePrintOptions::default().set_modal(true),
        )
        .await
        .map_err(|e| PortalError::Unavailable(e.to_string()))?;

    let prepared = request.response().map_err(|e| {
        let text = e.to_string();
        if is_cancel(&text) {
            PortalError::Cancelled
        } else {
            PortalError::Failed(format!("the print dialog failed: {text}"))
        }
    })?;

    let (paper_w, paper_h) = paper_mm(&prepared.page_setup, &prepared.settings);
    let place = pdf::layout(image.width, image.height, paper_w, paper_h, margin_mm(&prepared.page_setup));
    let bytes = pdf::build_pdf(image, &place).map_err(PortalError::Failed)?;
    let file = pdf_descriptor(&bytes).map_err(PortalError::Failed)?;

    proxy
        .print(
            None,
            title,
            &file,
            PrintOptions::default().set_token(prepared.token).set_modal(true),
        )
        .await
        .map_err(|e| PortalError::Failed(format!("sending the pattern to the printer failed: {e}")))?;

    Ok("Pattern sent to the printer".to_string())
}

/// Fallback without a print portal: open the PDF in the default viewer.
fn open_in_viewer(image: &RgbImage, why: &str) -> Result<String, String> {
    let (w, h) = DEFAULT_PAPER_MM;
    let place = pdf::layout(image.width, image.height, w, h, DEFAULT_MARGIN_MM);
    let bytes = pdf::build_pdf(image, &place)?;
    let path = std::env::temp_dir().join(format!("charto-print-{}.pdf", std::process::id()));
    std::fs::write(&path, bytes).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    std::process::Command::new("xdg-open").arg(&path).spawn().map_err(|e| {
        format!("no print portal ({why}) and xdg-open failed ({e}); the PDF was saved as {}", path.display())
    })?;
    Ok(format!("No print portal available ({why}); opened the PDF in your default viewer – print it from there"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paper_comes_from_the_page_setup_then_the_settings_then_a4() {
        let setup = PageSetup::default().set_width(215.9).set_height(279.4);
        assert_eq!(paper_mm(&setup, &Settings::default()), (215.9, 279.4));

        let settings = Settings::default().set_paper_width("148").set_paper_height("210");
        assert_eq!(paper_mm(&PageSetup::default(), &settings), (148.0, 210.0));

        assert_eq!(paper_mm(&PageSetup::default(), &Settings::default()), DEFAULT_PAPER_MM);
    }

    #[test]
    fn margins_default_to_ten_millimetres_and_are_capped() {
        assert_eq!(margin_mm(&PageSetup::default()), DEFAULT_MARGIN_MM);
        assert_eq!(margin_mm(&PageSetup::default().set_margin_left(12.5)), 12.5);
        assert_eq!(margin_mm(&PageSetup::default().set_margin_top(99.0)), 30.0);
    }

    #[test]
    fn cancel_detection() {
        assert!(is_cancel("Cancelled"));
        assert!(is_cancel("the user canceled the request"));
        assert!(!is_cancel("D-Bus error"));
    }
}

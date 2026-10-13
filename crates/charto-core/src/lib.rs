//! Qt-free core of *Charto*, a Rust port of *sconcho*, the knitting-chart
//! designer originally written by Markus Dittrich (GPL-3.0).
//!
//! * [`qdatastream`] – byte-exact reader/writer for the subset of Qt's
//!   `QDataStream` (stream version Qt_4_5) that `.spf` files use.
//! * [`spf`]         – the sconcho project file format, API versions 1–3.
//! * [`symbols`]     – loader for the knitting symbol library on disk.
//! * [`geometry`]    – small geometry helpers (outline sorting for repeats).
//! * [`labels`]      – row/column numbering and row highlighting.
//! * [`config`]      – per-user files following the XDG base directory spec.
//! * [`fileuri`]     – helpers for the desktop file dialogs (`file://` URIs, suggested names).
//! * [`svg`]         – vector (SVG) export of a chart.
//! * [`chart`]       – the editable chart: cells, selection, painting, copy/paste,
//!                     row/column insert/delete, labels, row repeats.
//! * [`document`]    – an editing session with clipboard and unlimited undo/redo.
//! * [`pdf`]         – turns a rendered chart image into a print-ready PDF page.
//! * [`session`]     – everything the GUI needs: files, symbols, tools, JSON snapshots.

pub mod chart;
pub mod config;
pub mod document;
pub mod fileuri;
pub mod geometry;
pub mod labels;
pub mod pdf;
pub mod qdatastream;
pub mod session;
pub mod svg;
pub mod spf;
pub mod symbols;

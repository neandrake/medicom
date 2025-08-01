/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! This is an auto-generated file. Do not make modifications here.
//!
//! This contains definitions of DICOM UIDs.

#![allow(non_upper_case_globals, clippy::doc_markdown)]

use crate::core::defn::uid::UID;

/// Verification SOP Class
///
/// - **UID:** 1.2.840.10008.1.1
/// - **UID Type:** SOP Class
pub static VerificationSOPClass: UID = UID::new(
    "VerificationSOPClass",
    "1.2.840.10008.1.1",
    "Verification SOP Class",
);

/// Implicit VR Little Endian: Default Transfer Syntax for DICOM
///
/// - **UID:** 1.2.840.10008.1.2
/// - **UID Type:** Transfer Syntax
pub static ImplicitVRLittleEndian: UID = UID::new(
    "ImplicitVRLittleEndian",
    "1.2.840.10008.1.2",
    "Implicit VR Little Endian: Default Transfer Syntax for DICOM",
);

/// Explicit VR Little Endian
///
/// - **UID:** 1.2.840.10008.1.2.1
/// - **UID Type:** Transfer Syntax
pub static ExplicitVRLittleEndian: UID = UID::new(
    "ExplicitVRLittleEndian",
    "1.2.840.10008.1.2.1",
    "Explicit VR Little Endian",
);

/// Encapsulated Uncompressed Explicit VR Little Endian
///
/// - **UID:** 1.2.840.10008.1.2.1.98
/// - **UID Type:** Transfer Syntax
pub static EncapsulatedUncompressedExplicitVRLittleEndian: UID = UID::new(
    "EncapsulatedUncompressedExplicitVRLittleEndian",
    "1.2.840.10008.1.2.1.98",
    "Encapsulated Uncompressed Explicit VR Little Endian",
);

/// Deflated Explicit VR Little Endian
///
/// - **UID:** 1.2.840.10008.1.2.1.99
/// - **UID Type:** Transfer Syntax
pub static DeflatedExplicitVRLittleEndian: UID = UID::new(
    "DeflatedExplicitVRLittleEndian",
    "1.2.840.10008.1.2.1.99",
    "Deflated Explicit VR Little Endian",
);

/// Explicit VR Big Endian (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.2
/// - **UID Type:** Transfer Syntax
pub static ExplicitVRBigEndian: UID = UID::new(
    "ExplicitVRBigEndian",
    "1.2.840.10008.1.2.2",
    "Explicit VR Big Endian (Retired)",
);

/// JPEG Baseline (Process 1): Default Transfer Syntax for Lossy JPEG 8 Bit Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.50
/// - **UID Type:** Transfer Syntax
pub static JPEGBaselineProcess1: UID = UID::new(
    "JPEGBaselineProcess1",
    "1.2.840.10008.1.2.4.50",
    "JPEG Baseline (Process 1): Default Transfer Syntax for Lossy JPEG 8 Bit Image Compression",
);

/// JPEG Extended (Process 2&4): Default Transfer Syntax for Lossy JPEG 12 Bit Image Compression (Process 4 only)
///
/// - **UID:** 1.2.840.10008.1.2.4.51
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedProcess2_and_4: UID = UID::new(
    "JPEGExtendedProcess2_and_4",
    "1.2.840.10008.1.2.4.51",
    "JPEG Extended (Process 2&4): Default Transfer Syntax for Lossy JPEG 12 Bit Image Compression (Process 4 only)",
);

/// JPEG Extended (Process 3&5) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.52
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedProcess3_and_5: UID = UID::new(
    "JPEGExtendedProcess3_and_5",
    "1.2.840.10008.1.2.4.52",
    "JPEG Extended (Process 3&5) (Retired)",
);

/// JPEG Spectral Selection, Non-Hierarchical (Process 6&8) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.53
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionNonHierarchicalProcess6_and_8: UID = UID::new(
    "JPEGSpectralSelectionNonHierarchicalProcess6_and_8",
    "1.2.840.10008.1.2.4.53",
    "JPEG Spectral Selection, Non-Hierarchical (Process 6&8) (Retired)",
);

/// JPEG Spectral Selection, Non-Hierarchical (Process 7&9) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.54
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionNonHierarchicalProcess7_and_9: UID = UID::new(
    "JPEGSpectralSelectionNonHierarchicalProcess7_and_9",
    "1.2.840.10008.1.2.4.54",
    "JPEG Spectral Selection, Non-Hierarchical (Process 7&9) (Retired)",
);

/// JPEG Full Progression, Non-Hierarchical (Process 10&12) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.55
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionNonHierarchicalProcess10_and_12: UID = UID::new(
    "JPEGFullProgressionNonHierarchicalProcess10_and_12",
    "1.2.840.10008.1.2.4.55",
    "JPEG Full Progression, Non-Hierarchical (Process 10&12) (Retired)",
);

/// JPEG Full Progression, Non-Hierarchical (Process 11&13) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.56
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionNonHierarchicalProcess11_and_13: UID = UID::new(
    "JPEGFullProgressionNonHierarchicalProcess11_and_13",
    "1.2.840.10008.1.2.4.56",
    "JPEG Full Progression, Non-Hierarchical (Process 11&13) (Retired)",
);

/// JPEG Lossless, Non-Hierarchical (Process 14)
///
/// - **UID:** 1.2.840.10008.1.2.4.57
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessNonHierarchicalProcess14: UID = UID::new(
    "JPEGLosslessNonHierarchicalProcess14",
    "1.2.840.10008.1.2.4.57",
    "JPEG Lossless, Non-Hierarchical (Process 14)",
);

/// JPEG Lossless, Non-Hierarchical (Process 15) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.58
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessNonHierarchicalProcess15: UID = UID::new(
    "JPEGLosslessNonHierarchicalProcess15",
    "1.2.840.10008.1.2.4.58",
    "JPEG Lossless, Non-Hierarchical (Process 15) (Retired)",
);

/// JPEG Extended, Hierarchical (Process 16&18) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.59
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedHierarchicalProcess16_and_18: UID = UID::new(
    "JPEGExtendedHierarchicalProcess16_and_18",
    "1.2.840.10008.1.2.4.59",
    "JPEG Extended, Hierarchical (Process 16&18) (Retired)",
);

/// JPEG Extended, Hierarchical (Process 17&19) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.60
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedHierarchicalProcess17_and_19: UID = UID::new(
    "JPEGExtendedHierarchicalProcess17_and_19",
    "1.2.840.10008.1.2.4.60",
    "JPEG Extended, Hierarchical (Process 17&19) (Retired)",
);

/// JPEG Spectral Selection, Hierarchical (Process 20&22) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.61
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionHierarchicalProcess20_and_22: UID = UID::new(
    "JPEGSpectralSelectionHierarchicalProcess20_and_22",
    "1.2.840.10008.1.2.4.61",
    "JPEG Spectral Selection, Hierarchical (Process 20&22) (Retired)",
);

/// JPEG Spectral Selection, Hierarchical (Process 21&23) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.62
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionHierarchicalProcess21_and_23: UID = UID::new(
    "JPEGSpectralSelectionHierarchicalProcess21_and_23",
    "1.2.840.10008.1.2.4.62",
    "JPEG Spectral Selection, Hierarchical (Process 21&23) (Retired)",
);

/// JPEG Full Progression, Hierarchical (Process 24&26) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.63
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionHierarchicalProcess24_and_26: UID = UID::new(
    "JPEGFullProgressionHierarchicalProcess24_and_26",
    "1.2.840.10008.1.2.4.63",
    "JPEG Full Progression, Hierarchical (Process 24&26) (Retired)",
);

/// JPEG Full Progression, Hierarchical (Process 25&27) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.64
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionHierarchicalProcess25_and_27: UID = UID::new(
    "JPEGFullProgressionHierarchicalProcess25_and_27",
    "1.2.840.10008.1.2.4.64",
    "JPEG Full Progression, Hierarchical (Process 25&27) (Retired)",
);

/// JPEG Lossless, Hierarchical (Process 28) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.65
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessHierarchicalProcess28: UID = UID::new(
    "JPEGLosslessHierarchicalProcess28",
    "1.2.840.10008.1.2.4.65",
    "JPEG Lossless, Hierarchical (Process 28) (Retired)",
);

/// JPEG Lossless, Hierarchical (Process 29) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.66
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessHierarchicalProcess29: UID = UID::new(
    "JPEGLosslessHierarchicalProcess29",
    "1.2.840.10008.1.2.4.66",
    "JPEG Lossless, Hierarchical (Process 29) (Retired)",
);

/// JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1]): Default Transfer Syntax for Lossless JPEG Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.70
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1: UID = UID::new(
    "JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1",
    "1.2.840.10008.1.2.4.70",
    "JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1]): Default Transfer Syntax for Lossless JPEG Image Compression",
);

/// JPEG-LS Lossless Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.80
/// - **UID Type:** Transfer Syntax
pub static JPEGLSLosslessImageCompression: UID = UID::new(
    "JPEGLSLosslessImageCompression",
    "1.2.840.10008.1.2.4.80",
    "JPEG-LS Lossless Image Compression",
);

/// JPEG-LS Lossy (Near-Lossless) Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.81
/// - **UID Type:** Transfer Syntax
pub static JPEGLSLossyNearLosslessImageCompression: UID = UID::new(
    "JPEGLSLossyNearLosslessImageCompression",
    "1.2.840.10008.1.2.4.81",
    "JPEG-LS Lossy (Near-Lossless) Image Compression",
);

/// JPEG 2000 Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.90
/// - **UID Type:** Transfer Syntax
pub static JPEG2000ImageCompressionLosslessOnly: UID = UID::new(
    "JPEG2000ImageCompressionLosslessOnly",
    "1.2.840.10008.1.2.4.90",
    "JPEG 2000 Image Compression (Lossless Only)",
);

/// JPEG 2000 Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.91
/// - **UID Type:** Transfer Syntax
pub static JPEG2000ImageCompression: UID = UID::new(
    "JPEG2000ImageCompression",
    "1.2.840.10008.1.2.4.91",
    "JPEG 2000 Image Compression",
);

/// JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.92
/// - **UID Type:** Transfer Syntax
pub static JPEG2000Part2MulticomponentImageCompressionLosslessOnly: UID = UID::new(
    "JPEG2000Part2MulticomponentImageCompressionLosslessOnly",
    "1.2.840.10008.1.2.4.92",
    "JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)",
);

/// JPEG 2000 Part 2 Multi-component Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.93
/// - **UID Type:** Transfer Syntax
pub static JPEG2000Part2MulticomponentImageCompression: UID = UID::new(
    "JPEG2000Part2MulticomponentImageCompression",
    "1.2.840.10008.1.2.4.93",
    "JPEG 2000 Part 2 Multi-component Image Compression",
);

/// JPIP Referenced
///
/// - **UID:** 1.2.840.10008.1.2.4.94
/// - **UID Type:** Transfer Syntax
pub static JPIPReferenced: UID = UID::new(
    "JPIPReferenced",
    "1.2.840.10008.1.2.4.94",
    "JPIP Referenced",
);

/// JPIP Referenced Deflate
///
/// - **UID:** 1.2.840.10008.1.2.4.95
/// - **UID Type:** Transfer Syntax
pub static JPIPReferencedDeflate: UID = UID::new(
    "JPIPReferencedDeflate",
    "1.2.840.10008.1.2.4.95",
    "JPIP Referenced Deflate",
);

/// MPEG2 Main Profile / Main Level
///
/// - **UID:** 1.2.840.10008.1.2.4.100
/// - **UID Type:** Transfer Syntax
pub static MPEG2MainProfileMainLevel: UID = UID::new(
    "MPEG2MainProfileMainLevel",
    "1.2.840.10008.1.2.4.100",
    "MPEG2 Main Profile / Main Level",
);

/// Fragmentable MPEG2 Main Profile / Main Level
///
/// - **UID:** 1.2.840.10008.1.2.4.100.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG2MainProfileMainLevel: UID = UID::new(
    "FragmentableMPEG2MainProfileMainLevel",
    "1.2.840.10008.1.2.4.100.1",
    "Fragmentable MPEG2 Main Profile / Main Level",
);

/// MPEG2 Main Profile / High Level
///
/// - **UID:** 1.2.840.10008.1.2.4.101
/// - **UID Type:** Transfer Syntax
pub static MPEG2MainProfileHighLevel: UID = UID::new(
    "MPEG2MainProfileHighLevel",
    "1.2.840.10008.1.2.4.101",
    "MPEG2 Main Profile / High Level",
);

/// Fragmentable MPEG2 Main Profile / High Level
///
/// - **UID:** 1.2.840.10008.1.2.4.101.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG2MainProfileHighLevel: UID = UID::new(
    "FragmentableMPEG2MainProfileHighLevel",
    "1.2.840.10008.1.2.4.101.1",
    "Fragmentable MPEG2 Main Profile / High Level",
);

/// MPEG-4 AVC/H.264 High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.102
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264HighProfileLevel41: UID = UID::new(
    "MPEG4AVCH264HighProfileLevel41",
    "1.2.840.10008.1.2.4.102",
    "MPEG-4 AVC/H.264 High Profile / Level 4.1",
);

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.102.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264HighProfileLevel41: UID = UID::new(
    "FragmentableMPEG4AVCH264HighProfileLevel41",
    "1.2.840.10008.1.2.4.102.1",
    "Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.1",
);

/// MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.103
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264BDcompatibleHighProfileLevel41: UID = UID::new(
    "MPEG4AVCH264BDcompatibleHighProfileLevel41",
    "1.2.840.10008.1.2.4.103",
    "MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1",
);

/// Fragmentable MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.103.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264BDcompatibleHighProfileLevel41: UID = UID::new(
    "FragmentableMPEG4AVCH264BDcompatibleHighProfileLevel41",
    "1.2.840.10008.1.2.4.103.1",
    "Fragmentable MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1",
);

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.104
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264HighProfileLevel42For2DVideo: UID = UID::new(
    "MPEG4AVCH264HighProfileLevel42For2DVideo",
    "1.2.840.10008.1.2.4.104",
    "MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video",
);

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.104.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264HighProfileLevel42For2DVideo: UID = UID::new(
    "FragmentableMPEG4AVCH264HighProfileLevel42For2DVideo",
    "1.2.840.10008.1.2.4.104.1",
    "Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video",
);

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.105
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264HighProfileLevel42For3DVideo: UID = UID::new(
    "MPEG4AVCH264HighProfileLevel42For3DVideo",
    "1.2.840.10008.1.2.4.105",
    "MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video",
);

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.105.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264HighProfileLevel42For3DVideo: UID = UID::new(
    "FragmentableMPEG4AVCH264HighProfileLevel42For3DVideo",
    "1.2.840.10008.1.2.4.105.1",
    "Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video",
);

/// MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2
///
/// - **UID:** 1.2.840.10008.1.2.4.106
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264StereoHighProfileLevel42: UID = UID::new(
    "MPEG4AVCH264StereoHighProfileLevel42",
    "1.2.840.10008.1.2.4.106",
    "MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2",
);

/// Fragmentable MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2
///
/// - **UID:** 1.2.840.10008.1.2.4.106.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264StereoHighProfileLevel42: UID = UID::new(
    "FragmentableMPEG4AVCH264StereoHighProfileLevel42",
    "1.2.840.10008.1.2.4.106.1",
    "Fragmentable MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2",
);

/// HEVC/H.265 Main Profile / Level 5.1
///
/// - **UID:** 1.2.840.10008.1.2.4.107
/// - **UID Type:** Transfer Syntax
pub static HEVCH265MainProfileLevel51: UID = UID::new(
    "HEVCH265MainProfileLevel51",
    "1.2.840.10008.1.2.4.107",
    "HEVC/H.265 Main Profile / Level 5.1",
);

/// HEVC/H.265 Main 10 Profile / Level 5.1
///
/// - **UID:** 1.2.840.10008.1.2.4.108
/// - **UID Type:** Transfer Syntax
pub static HEVCH265Main10ProfileLevel51: UID = UID::new(
    "HEVCH265Main10ProfileLevel51",
    "1.2.840.10008.1.2.4.108",
    "HEVC/H.265 Main 10 Profile / Level 5.1",
);

/// High-Throughput JPEG 2000 Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.201
/// - **UID Type:** Transfer Syntax
pub static HighThroughputJPEG2000ImageCompressionLosslessOnly: UID = UID::new(
    "HighThroughputJPEG2000ImageCompressionLosslessOnly",
    "1.2.840.10008.1.2.4.201",
    "High-Throughput JPEG 2000 Image Compression (Lossless Only)",
);

/// High-Throughput JPEG 2000 with RPCL Options Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.202
/// - **UID Type:** Transfer Syntax
pub static HighThroughputJPEG2000withRPCLOptionsImageCompressionLosslessOnly: UID = UID::new(
    "HighThroughputJPEG2000withRPCLOptionsImageCompressionLosslessOnly",
    "1.2.840.10008.1.2.4.202",
    "High-Throughput JPEG 2000 with RPCL Options Image Compression (Lossless Only)",
);

/// High-Throughput JPEG 2000 Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.203
/// - **UID Type:** Transfer Syntax
pub static HighThroughputJPEG2000ImageCompression: UID = UID::new(
    "HighThroughputJPEG2000ImageCompression",
    "1.2.840.10008.1.2.4.203",
    "High-Throughput JPEG 2000 Image Compression",
);

/// JPIP HTJ2K Referenced
///
/// - **UID:** 1.2.840.10008.1.2.4.204
/// - **UID Type:** Transfer Syntax
pub static JPIPHTJ2KReferenced: UID = UID::new(
    "JPIPHTJ2KReferenced",
    "1.2.840.10008.1.2.4.204",
    "JPIP HTJ2K Referenced",
);

/// JPIP HTJ2K Referenced Deflate
///
/// - **UID:** 1.2.840.10008.1.2.4.205
/// - **UID Type:** Transfer Syntax
pub static JPIPHTJ2KReferencedDeflate: UID = UID::new(
    "JPIPHTJ2KReferencedDeflate",
    "1.2.840.10008.1.2.4.205",
    "JPIP HTJ2K Referenced Deflate",
);

/// RLE Lossless
///
/// - **UID:** 1.2.840.10008.1.2.5
/// - **UID Type:** Transfer Syntax
pub static RLELossless: UID = UID::new(
    "RLELossless",
    "1.2.840.10008.1.2.5",
    "RLE Lossless",
);

/// RFC 2557 MIME encapsulation (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.6.1
/// - **UID Type:** Transfer Syntax
pub static RFC2557MIMEencapsulation: UID = UID::new(
    "RFC2557MIMEencapsulation",
    "1.2.840.10008.1.2.6.1",
    "RFC 2557 MIME encapsulation (Retired)",
);

/// XML Encoding (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.6.2
/// - **UID Type:** Transfer Syntax
pub static XMLEncoding: UID = UID::new(
    "XMLEncoding",
    "1.2.840.10008.1.2.6.2",
    "XML Encoding (Retired)",
);

/// SMPTE ST 2110-20 Uncompressed Progressive Active Video
///
/// - **UID:** 1.2.840.10008.1.2.7.1
/// - **UID Type:** Transfer Syntax
pub static SMPTEST211020UncompressedProgressiveActiveVideo: UID = UID::new(
    "SMPTEST211020UncompressedProgressiveActiveVideo",
    "1.2.840.10008.1.2.7.1",
    "SMPTE ST 2110-20 Uncompressed Progressive Active Video",
);

/// SMPTE ST 2110-20 Uncompressed Interlaced Active Video
///
/// - **UID:** 1.2.840.10008.1.2.7.2
/// - **UID Type:** Transfer Syntax
pub static SMPTEST211020UncompressedInterlacedActiveVideo: UID = UID::new(
    "SMPTEST211020UncompressedInterlacedActiveVideo",
    "1.2.840.10008.1.2.7.2",
    "SMPTE ST 2110-20 Uncompressed Interlaced Active Video",
);

/// SMPTE ST 2110-30 PCM Digital Audio
///
/// - **UID:** 1.2.840.10008.1.2.7.3
/// - **UID Type:** Transfer Syntax
pub static SMPTEST211030PCMDigitalAudio: UID = UID::new(
    "SMPTEST211030PCMDigitalAudio",
    "1.2.840.10008.1.2.7.3",
    "SMPTE ST 2110-30 PCM Digital Audio",
);

/// Media Storage Directory Storage
///
/// - **UID:** 1.2.840.10008.1.3.10
/// - **UID Type:** SOP Class
pub static MediaStorageDirectoryStorage: UID = UID::new(
    "MediaStorageDirectoryStorage",
    "1.2.840.10008.1.3.10",
    "Media Storage Directory Storage",
);

/// Hot Iron Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.1
/// - **UID Type:** Well-known SOP Instance
pub static HotIronColorPaletteSOPInstance: UID = UID::new(
    "HotIronColorPaletteSOPInstance",
    "1.2.840.10008.1.5.1",
    "Hot Iron Color Palette SOP Instance",
);

/// PET Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.2
/// - **UID Type:** Well-known SOP Instance
pub static PETColorPaletteSOPInstance: UID = UID::new(
    "PETColorPaletteSOPInstance",
    "1.2.840.10008.1.5.2",
    "PET Color Palette SOP Instance",
);

/// Hot Metal Blue Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.3
/// - **UID Type:** Well-known SOP Instance
pub static HotMetalBlueColorPaletteSOPInstance: UID = UID::new(
    "HotMetalBlueColorPaletteSOPInstance",
    "1.2.840.10008.1.5.3",
    "Hot Metal Blue Color Palette SOP Instance",
);

/// PET 20 Step Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.4
/// - **UID Type:** Well-known SOP Instance
pub static PET20StepColorPaletteSOPInstance: UID = UID::new(
    "PET20StepColorPaletteSOPInstance",
    "1.2.840.10008.1.5.4",
    "PET 20 Step Color Palette SOP Instance",
);

/// Spring Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.5
/// - **UID Type:** Well-known SOP Instance
pub static SpringColorPaletteSOPInstance: UID = UID::new(
    "SpringColorPaletteSOPInstance",
    "1.2.840.10008.1.5.5",
    "Spring Color Palette SOP Instance",
);

/// Summer Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.6
/// - **UID Type:** Well-known SOP Instance
pub static SummerColorPaletteSOPInstance: UID = UID::new(
    "SummerColorPaletteSOPInstance",
    "1.2.840.10008.1.5.6",
    "Summer Color Palette SOP Instance",
);

/// Fall Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.7
/// - **UID Type:** Well-known SOP Instance
pub static FallColorPaletteSOPInstance: UID = UID::new(
    "FallColorPaletteSOPInstance",
    "1.2.840.10008.1.5.7",
    "Fall Color Palette SOP Instance",
);

/// Winter Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.8
/// - **UID Type:** Well-known SOP Instance
pub static WinterColorPaletteSOPInstance: UID = UID::new(
    "WinterColorPaletteSOPInstance",
    "1.2.840.10008.1.5.8",
    "Winter Color Palette SOP Instance",
);

/// Basic Study Content Notification SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.1.9
/// - **UID Type:** SOP Class
pub static BasicStudyContentNotificationSOPClass: UID = UID::new(
    "BasicStudyContentNotificationSOPClass",
    "1.2.840.10008.1.9",
    "Basic Study Content Notification SOP Class (Retired)",
);

/// Papyrus 3 Implicit VR Little Endian (Retired)
///
/// - **UID:** 1.2.840.10008.1.20
/// - **UID Type:** Transfer Syntax
pub static Papyrus3ImplicitVRLittleEndian: UID = UID::new(
    "Papyrus3ImplicitVRLittleEndian",
    "1.2.840.10008.1.20",
    "Papyrus 3 Implicit VR Little Endian (Retired)",
);

/// Storage Commitment Push Model SOP Class
///
/// - **UID:** 1.2.840.10008.1.20.1
/// - **UID Type:** SOP Class
pub static StorageCommitmentPushModelSOPClass: UID = UID::new(
    "StorageCommitmentPushModelSOPClass",
    "1.2.840.10008.1.20.1",
    "Storage Commitment Push Model SOP Class",
);

/// Storage Commitment Push Model SOP Instance
///
/// - **UID:** 1.2.840.10008.1.20.1.1
/// - **UID Type:** Well-known SOP Instance
pub static StorageCommitmentPushModelSOPInstance: UID = UID::new(
    "StorageCommitmentPushModelSOPInstance",
    "1.2.840.10008.1.20.1.1",
    "Storage Commitment Push Model SOP Instance",
);

/// Storage Commitment Pull Model SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.1.20.2
/// - **UID Type:** SOP Class
pub static StorageCommitmentPullModelSOPClass: UID = UID::new(
    "StorageCommitmentPullModelSOPClass",
    "1.2.840.10008.1.20.2",
    "Storage Commitment Pull Model SOP Class (Retired)",
);

/// Storage Commitment Pull Model SOP Instance (Retired)
///
/// - **UID:** 1.2.840.10008.1.20.2.1
/// - **UID Type:** Well-known SOP Instance
pub static StorageCommitmentPullModelSOPInstance: UID = UID::new(
    "StorageCommitmentPullModelSOPInstance",
    "1.2.840.10008.1.20.2.1",
    "Storage Commitment Pull Model SOP Instance (Retired)",
);

/// Procedural Event Logging SOP Class
///
/// - **UID:** 1.2.840.10008.1.40
/// - **UID Type:** SOP Class
pub static ProceduralEventLoggingSOPClass: UID = UID::new(
    "ProceduralEventLoggingSOPClass",
    "1.2.840.10008.1.40",
    "Procedural Event Logging SOP Class",
);

/// Procedural Event Logging SOP Instance
///
/// - **UID:** 1.2.840.10008.1.40.1
/// - **UID Type:** Well-known SOP Instance
pub static ProceduralEventLoggingSOPInstance: UID = UID::new(
    "ProceduralEventLoggingSOPInstance",
    "1.2.840.10008.1.40.1",
    "Procedural Event Logging SOP Instance",
);

/// Substance Administration Logging SOP Class
///
/// - **UID:** 1.2.840.10008.1.42
/// - **UID Type:** SOP Class
pub static SubstanceAdministrationLoggingSOPClass: UID = UID::new(
    "SubstanceAdministrationLoggingSOPClass",
    "1.2.840.10008.1.42",
    "Substance Administration Logging SOP Class",
);

/// Substance Administration Logging SOP Instance
///
/// - **UID:** 1.2.840.10008.1.42.1
/// - **UID Type:** Well-known SOP Instance
pub static SubstanceAdministrationLoggingSOPInstance: UID = UID::new(
    "SubstanceAdministrationLoggingSOPInstance",
    "1.2.840.10008.1.42.1",
    "Substance Administration Logging SOP Instance",
);

/// DICOM UID Registry
///
/// - **UID:** 1.2.840.10008.2.6.1
/// - **UID Type:** DICOM UIDs as a Coding Scheme
pub static DICOMUIDRegistry: UID = UID::new(
    "DICOMUIDRegistry",
    "1.2.840.10008.2.6.1",
    "DICOM UID Registry",
);

/// DICOM Controlled Terminology
///
/// - **UID:** 1.2.840.10008.2.16.4
/// - **UID Type:** Coding Scheme
pub static DICOMControlledTerminology: UID = UID::new(
    "DICOMControlledTerminology",
    "1.2.840.10008.2.16.4",
    "DICOM Controlled Terminology",
);

/// Adult Mouse Anatomy Ontology
///
/// - **UID:** 1.2.840.10008.2.16.5
/// - **UID Type:** Coding Scheme
pub static AdultMouseAnatomyOntology: UID = UID::new(
    "AdultMouseAnatomyOntology",
    "1.2.840.10008.2.16.5",
    "Adult Mouse Anatomy Ontology",
);

/// Uberon Ontology
///
/// - **UID:** 1.2.840.10008.2.16.6
/// - **UID Type:** Coding Scheme
pub static UberonOntology: UID = UID::new(
    "UberonOntology",
    "1.2.840.10008.2.16.6",
    "Uberon Ontology",
);

/// Integrated Taxonomic Information System (ITIS) Taxonomic Serial Number (TSN)
///
/// - **UID:** 1.2.840.10008.2.16.7
/// - **UID Type:** Coding Scheme
pub static IntegratedTaxonomicInformationSystemITISTaxonomicSerialNumberTSN: UID = UID::new(
    "IntegratedTaxonomicInformationSystemITISTaxonomicSerialNumberTSN",
    "1.2.840.10008.2.16.7",
    "Integrated Taxonomic Information System (ITIS) Taxonomic Serial Number (TSN)",
);

/// Mouse Genome Initiative (MGI)
///
/// - **UID:** 1.2.840.10008.2.16.8
/// - **UID Type:** Coding Scheme
pub static MouseGenomeInitiativeMGI: UID = UID::new(
    "MouseGenomeInitiativeMGI",
    "1.2.840.10008.2.16.8",
    "Mouse Genome Initiative (MGI)",
);

/// PubChem Compound CID
///
/// - **UID:** 1.2.840.10008.2.16.9
/// - **UID Type:** Coding Scheme
pub static PubChemCompoundCID: UID = UID::new(
    "PubChemCompoundCID",
    "1.2.840.10008.2.16.9",
    "PubChem Compound CID",
);

/// Dublin Core
///
/// - **UID:** 1.2.840.10008.2.16.10
/// - **UID Type:** Coding Scheme
pub static DublinCore: UID = UID::new(
    "DublinCore",
    "1.2.840.10008.2.16.10",
    "Dublin Core",
);

/// New York University Melanoma Clinical Cooperative Group
///
/// - **UID:** 1.2.840.10008.2.16.11
/// - **UID Type:** Coding Scheme
pub static NewYorkUniversityMelanomaClinicalCooperativeGroup: UID = UID::new(
    "NewYorkUniversityMelanomaClinicalCooperativeGroup",
    "1.2.840.10008.2.16.11",
    "New York University Melanoma Clinical Cooperative Group",
);

/// Mayo Clinic Non-radiological Images Specific Body Structure Anatomical Surface Region Guide
///
/// - **UID:** 1.2.840.10008.2.16.12
/// - **UID Type:** Coding Scheme
pub static MayoClinicNonradiologicalImagesSpecificBodyStructureAnatomicalSurfaceRegionGuide: UID = UID::new(
    "MayoClinicNonradiologicalImagesSpecificBodyStructureAnatomicalSurfaceRegionGuide",
    "1.2.840.10008.2.16.12",
    "Mayo Clinic Non-radiological Images Specific Body Structure Anatomical Surface Region Guide",
);

/// Image Biomarker Standardisation Initiative
///
/// - **UID:** 1.2.840.10008.2.16.13
/// - **UID Type:** Coding Scheme
pub static ImageBiomarkerStandardisationInitiative: UID = UID::new(
    "ImageBiomarkerStandardisationInitiative",
    "1.2.840.10008.2.16.13",
    "Image Biomarker Standardisation Initiative",
);

/// Radiomics Ontology
///
/// - **UID:** 1.2.840.10008.2.16.14
/// - **UID Type:** Coding Scheme
pub static RadiomicsOntology: UID = UID::new(
    "RadiomicsOntology",
    "1.2.840.10008.2.16.14",
    "Radiomics Ontology",
);

/// RadElement
///
/// - **UID:** 1.2.840.10008.2.16.15
/// - **UID Type:** Coding Scheme
pub static RadElement: UID = UID::new(
    "RadElement",
    "1.2.840.10008.2.16.15",
    "RadElement",
);

/// ICD-11
///
/// - **UID:** 1.2.840.10008.2.16.16
/// - **UID Type:** Coding Scheme
pub static ICD11: UID = UID::new(
    "ICD11",
    "1.2.840.10008.2.16.16",
    "ICD-11",
);

/// Unified numbering system (UNS) for metals and alloys
///
/// - **UID:** 1.2.840.10008.2.16.17
/// - **UID Type:** Coding Scheme
pub static UnifiednumberingsystemUNSformetalsandalloys: UID = UID::new(
    "UnifiednumberingsystemUNSformetalsandalloys",
    "1.2.840.10008.2.16.17",
    "Unified numbering system (UNS) for metals and alloys",
);

/// Research Resource Identification
///
/// - **UID:** 1.2.840.10008.2.16.18
/// - **UID Type:** Coding Scheme
pub static ResearchResourceIdentification: UID = UID::new(
    "ResearchResourceIdentification",
    "1.2.840.10008.2.16.18",
    "Research Resource Identification",
);

/// DICOM Application Context Name
///
/// - **UID:** 1.2.840.10008.3.1.1.1
/// - **UID Type:** Application Context Name
pub static DICOMApplicationContextName: UID = UID::new(
    "DICOMApplicationContextName",
    "1.2.840.10008.3.1.1.1",
    "DICOM Application Context Name",
);

/// Detached Patient Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.1.1
/// - **UID Type:** SOP Class
pub static DetachedPatientManagementSOPClass: UID = UID::new(
    "DetachedPatientManagementSOPClass",
    "1.2.840.10008.3.1.2.1.1",
    "Detached Patient Management SOP Class (Retired)",
);

/// Detached Patient Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.1.4
/// - **UID Type:** Meta SOP Class
pub static DetachedPatientManagementMetaSOPClass: UID = UID::new(
    "DetachedPatientManagementMetaSOPClass",
    "1.2.840.10008.3.1.2.1.4",
    "Detached Patient Management Meta SOP Class (Retired)",
);

/// Detached Visit Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.2.1
/// - **UID Type:** SOP Class
pub static DetachedVisitManagementSOPClass: UID = UID::new(
    "DetachedVisitManagementSOPClass",
    "1.2.840.10008.3.1.2.2.1",
    "Detached Visit Management SOP Class (Retired)",
);

/// Detached Study Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.3.1
/// - **UID Type:** SOP Class
pub static DetachedStudyManagementSOPClass: UID = UID::new(
    "DetachedStudyManagementSOPClass",
    "1.2.840.10008.3.1.2.3.1",
    "Detached Study Management SOP Class (Retired)",
);

/// Study Component Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.3.2
/// - **UID Type:** SOP Class
pub static StudyComponentManagementSOPClass: UID = UID::new(
    "StudyComponentManagementSOPClass",
    "1.2.840.10008.3.1.2.3.2",
    "Study Component Management SOP Class (Retired)",
);

/// Modality Performed Procedure Step SOP Class
///
/// - **UID:** 1.2.840.10008.3.1.2.3.3
/// - **UID Type:** SOP Class
pub static ModalityPerformedProcedureStepSOPClass: UID = UID::new(
    "ModalityPerformedProcedureStepSOPClass",
    "1.2.840.10008.3.1.2.3.3",
    "Modality Performed Procedure Step SOP Class",
);

/// Modality Performed Procedure Step Retrieve SOP Class
///
/// - **UID:** 1.2.840.10008.3.1.2.3.4
/// - **UID Type:** SOP Class
pub static ModalityPerformedProcedureStepRetrieveSOPClass: UID = UID::new(
    "ModalityPerformedProcedureStepRetrieveSOPClass",
    "1.2.840.10008.3.1.2.3.4",
    "Modality Performed Procedure Step Retrieve SOP Class",
);

/// Modality Performed Procedure Step Notification SOP Class
///
/// - **UID:** 1.2.840.10008.3.1.2.3.5
/// - **UID Type:** SOP Class
pub static ModalityPerformedProcedureStepNotificationSOPClass: UID = UID::new(
    "ModalityPerformedProcedureStepNotificationSOPClass",
    "1.2.840.10008.3.1.2.3.5",
    "Modality Performed Procedure Step Notification SOP Class",
);

/// Detached Results Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.5.1
/// - **UID Type:** SOP Class
pub static DetachedResultsManagementSOPClass: UID = UID::new(
    "DetachedResultsManagementSOPClass",
    "1.2.840.10008.3.1.2.5.1",
    "Detached Results Management SOP Class (Retired)",
);

/// Detached Results Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.5.4
/// - **UID Type:** Meta SOP Class
pub static DetachedResultsManagementMetaSOPClass: UID = UID::new(
    "DetachedResultsManagementMetaSOPClass",
    "1.2.840.10008.3.1.2.5.4",
    "Detached Results Management Meta SOP Class (Retired)",
);

/// Detached Study Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.5.5
/// - **UID Type:** Meta SOP Class
pub static DetachedStudyManagementMetaSOPClass: UID = UID::new(
    "DetachedStudyManagementMetaSOPClass",
    "1.2.840.10008.3.1.2.5.5",
    "Detached Study Management Meta SOP Class (Retired)",
);

/// Detached Interpretation Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.6.1
/// - **UID Type:** SOP Class
pub static DetachedInterpretationManagementSOPClass: UID = UID::new(
    "DetachedInterpretationManagementSOPClass",
    "1.2.840.10008.3.1.2.6.1",
    "Detached Interpretation Management SOP Class (Retired)",
);

/// Storage Service Class
///
/// - **UID:** 1.2.840.10008.4.2
/// - **UID Type:** Service Class
pub static StorageServiceClass: UID = UID::new(
    "StorageServiceClass",
    "1.2.840.10008.4.2",
    "Storage Service Class",
);

/// Basic Film Session SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.1
/// - **UID Type:** SOP Class
pub static BasicFilmSessionSOPClass: UID = UID::new(
    "BasicFilmSessionSOPClass",
    "1.2.840.10008.5.1.1.1",
    "Basic Film Session SOP Class",
);

/// Basic Film Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.2
/// - **UID Type:** SOP Class
pub static BasicFilmBoxSOPClass: UID = UID::new(
    "BasicFilmBoxSOPClass",
    "1.2.840.10008.5.1.1.2",
    "Basic Film Box SOP Class",
);

/// Basic Grayscale Image Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.4
/// - **UID Type:** SOP Class
pub static BasicGrayscaleImageBoxSOPClass: UID = UID::new(
    "BasicGrayscaleImageBoxSOPClass",
    "1.2.840.10008.5.1.1.4",
    "Basic Grayscale Image Box SOP Class",
);

/// Basic Color Image Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.4.1
/// - **UID Type:** SOP Class
pub static BasicColorImageBoxSOPClass: UID = UID::new(
    "BasicColorImageBoxSOPClass",
    "1.2.840.10008.5.1.1.4.1",
    "Basic Color Image Box SOP Class",
);

/// Referenced Image Box SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.4.2
/// - **UID Type:** SOP Class
pub static ReferencedImageBoxSOPClass: UID = UID::new(
    "ReferencedImageBoxSOPClass",
    "1.2.840.10008.5.1.1.4.2",
    "Referenced Image Box SOP Class (Retired)",
);

/// Basic Grayscale Print Management Meta SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.9
/// - **UID Type:** Meta SOP Class
pub static BasicGrayscalePrintManagementMetaSOPClass: UID = UID::new(
    "BasicGrayscalePrintManagementMetaSOPClass",
    "1.2.840.10008.5.1.1.9",
    "Basic Grayscale Print Management Meta SOP Class",
);

/// Referenced Grayscale Print Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.9.1
/// - **UID Type:** Meta SOP Class
pub static ReferencedGrayscalePrintManagementMetaSOPClass: UID = UID::new(
    "ReferencedGrayscalePrintManagementMetaSOPClass",
    "1.2.840.10008.5.1.1.9.1",
    "Referenced Grayscale Print Management Meta SOP Class (Retired)",
);

/// Print Job SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.14
/// - **UID Type:** SOP Class
pub static PrintJobSOPClass: UID = UID::new(
    "PrintJobSOPClass",
    "1.2.840.10008.5.1.1.14",
    "Print Job SOP Class",
);

/// Basic Annotation Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.15
/// - **UID Type:** SOP Class
pub static BasicAnnotationBoxSOPClass: UID = UID::new(
    "BasicAnnotationBoxSOPClass",
    "1.2.840.10008.5.1.1.15",
    "Basic Annotation Box SOP Class",
);

/// Printer SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.16
/// - **UID Type:** SOP Class
pub static PrinterSOPClass: UID = UID::new(
    "PrinterSOPClass",
    "1.2.840.10008.5.1.1.16",
    "Printer SOP Class",
);

/// Printer Configuration Retrieval SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.16.376
/// - **UID Type:** SOP Class
pub static PrinterConfigurationRetrievalSOPClass: UID = UID::new(
    "PrinterConfigurationRetrievalSOPClass",
    "1.2.840.10008.5.1.1.16.376",
    "Printer Configuration Retrieval SOP Class",
);

/// Printer SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.1.17
/// - **UID Type:** Well-known SOP Instance
pub static PrinterSOPInstance: UID = UID::new(
    "PrinterSOPInstance",
    "1.2.840.10008.5.1.1.17",
    "Printer SOP Instance",
);

/// Printer Configuration Retrieval SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.1.17.376
/// - **UID Type:** Well-known SOP Instance
pub static PrinterConfigurationRetrievalSOPInstance: UID = UID::new(
    "PrinterConfigurationRetrievalSOPInstance",
    "1.2.840.10008.5.1.1.17.376",
    "Printer Configuration Retrieval SOP Instance",
);

/// Basic Color Print Management Meta SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.18
/// - **UID Type:** Meta SOP Class
pub static BasicColorPrintManagementMetaSOPClass: UID = UID::new(
    "BasicColorPrintManagementMetaSOPClass",
    "1.2.840.10008.5.1.1.18",
    "Basic Color Print Management Meta SOP Class",
);

/// Referenced Color Print Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.18.1
/// - **UID Type:** Meta SOP Class
pub static ReferencedColorPrintManagementMetaSOPClass: UID = UID::new(
    "ReferencedColorPrintManagementMetaSOPClass",
    "1.2.840.10008.5.1.1.18.1",
    "Referenced Color Print Management Meta SOP Class (Retired)",
);

/// VOI LUT Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.22
/// - **UID Type:** SOP Class
pub static VOILUTBoxSOPClass: UID = UID::new(
    "VOILUTBoxSOPClass",
    "1.2.840.10008.5.1.1.22",
    "VOI LUT Box SOP Class",
);

/// Presentation LUT SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.23
/// - **UID Type:** SOP Class
pub static PresentationLUTSOPClass: UID = UID::new(
    "PresentationLUTSOPClass",
    "1.2.840.10008.5.1.1.23",
    "Presentation LUT SOP Class",
);

/// Image Overlay Box SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.24
/// - **UID Type:** SOP Class
pub static ImageOverlayBoxSOPClass: UID = UID::new(
    "ImageOverlayBoxSOPClass",
    "1.2.840.10008.5.1.1.24",
    "Image Overlay Box SOP Class (Retired)",
);

/// Basic Print Image Overlay Box SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.24.1
/// - **UID Type:** SOP Class
pub static BasicPrintImageOverlayBoxSOPClass: UID = UID::new(
    "BasicPrintImageOverlayBoxSOPClass",
    "1.2.840.10008.5.1.1.24.1",
    "Basic Print Image Overlay Box SOP Class (Retired)",
);

/// Print Queue SOP Instance (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.25
/// - **UID Type:** Well-known SOP Instance
pub static PrintQueueSOPInstance: UID = UID::new(
    "PrintQueueSOPInstance",
    "1.2.840.10008.5.1.1.25",
    "Print Queue SOP Instance (Retired)",
);

/// Print Queue Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.26
/// - **UID Type:** SOP Class
pub static PrintQueueManagementSOPClass: UID = UID::new(
    "PrintQueueManagementSOPClass",
    "1.2.840.10008.5.1.1.26",
    "Print Queue Management SOP Class (Retired)",
);

/// Stored Print Storage SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.27
/// - **UID Type:** SOP Class
pub static StoredPrintStorageSOPClass: UID = UID::new(
    "StoredPrintStorageSOPClass",
    "1.2.840.10008.5.1.1.27",
    "Stored Print Storage SOP Class (Retired)",
);

/// Hardcopy Grayscale Image Storage SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.29
/// - **UID Type:** SOP Class
pub static HardcopyGrayscaleImageStorageSOPClass: UID = UID::new(
    "HardcopyGrayscaleImageStorageSOPClass",
    "1.2.840.10008.5.1.1.29",
    "Hardcopy Grayscale Image Storage SOP Class (Retired)",
);

/// Hardcopy Color Image Storage SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.30
/// - **UID Type:** SOP Class
pub static HardcopyColorImageStorageSOPClass: UID = UID::new(
    "HardcopyColorImageStorageSOPClass",
    "1.2.840.10008.5.1.1.30",
    "Hardcopy Color Image Storage SOP Class (Retired)",
);

/// Pull Print Request SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.31
/// - **UID Type:** SOP Class
pub static PullPrintRequestSOPClass: UID = UID::new(
    "PullPrintRequestSOPClass",
    "1.2.840.10008.5.1.1.31",
    "Pull Print Request SOP Class (Retired)",
);

/// Pull Stored Print Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.32
/// - **UID Type:** Meta SOP Class
pub static PullStoredPrintManagementMetaSOPClass: UID = UID::new(
    "PullStoredPrintManagementMetaSOPClass",
    "1.2.840.10008.5.1.1.32",
    "Pull Stored Print Management Meta SOP Class (Retired)",
);

/// Media Creation Management SOP Class UID
///
/// - **UID:** 1.2.840.10008.5.1.1.33
/// - **UID Type:** SOP Class
pub static MediaCreationManagementSOPClassUID: UID = UID::new(
    "MediaCreationManagementSOPClassUID",
    "1.2.840.10008.5.1.1.33",
    "Media Creation Management SOP Class UID",
);

/// Display System SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.40
/// - **UID Type:** SOP Class
pub static DisplaySystemSOPClass: UID = UID::new(
    "DisplaySystemSOPClass",
    "1.2.840.10008.5.1.1.40",
    "Display System SOP Class",
);

/// Display System SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.1.40.1
/// - **UID Type:** Well-known SOP Instance
pub static DisplaySystemSOPInstance: UID = UID::new(
    "DisplaySystemSOPInstance",
    "1.2.840.10008.5.1.1.40.1",
    "Display System SOP Instance",
);

/// Computed Radiography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1
/// - **UID Type:** SOP Class
pub static ComputedRadiographyImageStorage: UID = UID::new(
    "ComputedRadiographyImageStorage",
    "1.2.840.10008.5.1.4.1.1.1",
    "Computed Radiography Image Storage",
);

/// Digital X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.1
/// - **UID Type:** SOP Class
pub static DigitalXRayImageStorageForPresentation: UID = UID::new(
    "DigitalXRayImageStorageForPresentation",
    "1.2.840.10008.5.1.4.1.1.1.1",
    "Digital X-Ray Image Storage - For Presentation",
);

/// Digital X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.1.1
/// - **UID Type:** SOP Class
pub static DigitalXRayImageStorageForProcessing: UID = UID::new(
    "DigitalXRayImageStorageForProcessing",
    "1.2.840.10008.5.1.4.1.1.1.1.1",
    "Digital X-Ray Image Storage - For Processing",
);

/// Digital Mammography X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.2
/// - **UID Type:** SOP Class
pub static DigitalMammographyXRayImageStorageForPresentation: UID = UID::new(
    "DigitalMammographyXRayImageStorageForPresentation",
    "1.2.840.10008.5.1.4.1.1.1.2",
    "Digital Mammography X-Ray Image Storage - For Presentation",
);

/// Digital Mammography X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.2.1
/// - **UID Type:** SOP Class
pub static DigitalMammographyXRayImageStorageForProcessing: UID = UID::new(
    "DigitalMammographyXRayImageStorageForProcessing",
    "1.2.840.10008.5.1.4.1.1.1.2.1",
    "Digital Mammography X-Ray Image Storage - For Processing",
);

/// Digital Intra-Oral X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.3
/// - **UID Type:** SOP Class
pub static DigitalIntraOralXRayImageStorageForPresentation: UID = UID::new(
    "DigitalIntraOralXRayImageStorageForPresentation",
    "1.2.840.10008.5.1.4.1.1.1.3",
    "Digital Intra-Oral X-Ray Image Storage - For Presentation",
);

/// Digital Intra-Oral X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.3.1
/// - **UID Type:** SOP Class
pub static DigitalIntraOralXRayImageStorageForProcessing: UID = UID::new(
    "DigitalIntraOralXRayImageStorageForProcessing",
    "1.2.840.10008.5.1.4.1.1.1.3.1",
    "Digital Intra-Oral X-Ray Image Storage - For Processing",
);

/// CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.2
/// - **UID Type:** SOP Class
pub static CTImageStorage: UID = UID::new(
    "CTImageStorage",
    "1.2.840.10008.5.1.4.1.1.2",
    "CT Image Storage",
);

/// Enhanced CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.2.1
/// - **UID Type:** SOP Class
pub static EnhancedCTImageStorage: UID = UID::new(
    "EnhancedCTImageStorage",
    "1.2.840.10008.5.1.4.1.1.2.1",
    "Enhanced CT Image Storage",
);

/// Legacy Converted Enhanced CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.2.2
/// - **UID Type:** SOP Class
pub static LegacyConvertedEnhancedCTImageStorage: UID = UID::new(
    "LegacyConvertedEnhancedCTImageStorage",
    "1.2.840.10008.5.1.4.1.1.2.2",
    "Legacy Converted Enhanced CT Image Storage",
);

/// Ultrasound Multi-frame Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.3
/// - **UID Type:** SOP Class
pub static UltrasoundMultiframeImageStorage_Retired: UID = UID::new(
    "UltrasoundMultiframeImageStorage_Retired",
    "1.2.840.10008.5.1.4.1.1.3",
    "Ultrasound Multi-frame Image Storage (Retired)",
);

/// Ultrasound Multi-frame Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.3.1
/// - **UID Type:** SOP Class
pub static UltrasoundMultiframeImageStorage: UID = UID::new(
    "UltrasoundMultiframeImageStorage",
    "1.2.840.10008.5.1.4.1.1.3.1",
    "Ultrasound Multi-frame Image Storage",
);

/// MR Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4
/// - **UID Type:** SOP Class
pub static MRImageStorage: UID = UID::new(
    "MRImageStorage",
    "1.2.840.10008.5.1.4.1.1.4",
    "MR Image Storage",
);

/// Enhanced MR Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.1
/// - **UID Type:** SOP Class
pub static EnhancedMRImageStorage: UID = UID::new(
    "EnhancedMRImageStorage",
    "1.2.840.10008.5.1.4.1.1.4.1",
    "Enhanced MR Image Storage",
);

/// MR Spectroscopy Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.2
/// - **UID Type:** SOP Class
pub static MRSpectroscopyStorage: UID = UID::new(
    "MRSpectroscopyStorage",
    "1.2.840.10008.5.1.4.1.1.4.2",
    "MR Spectroscopy Storage",
);

/// Enhanced MR Color Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.3
/// - **UID Type:** SOP Class
pub static EnhancedMRColorImageStorage: UID = UID::new(
    "EnhancedMRColorImageStorage",
    "1.2.840.10008.5.1.4.1.1.4.3",
    "Enhanced MR Color Image Storage",
);

/// Legacy Converted Enhanced MR Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.4
/// - **UID Type:** SOP Class
pub static LegacyConvertedEnhancedMRImageStorage: UID = UID::new(
    "LegacyConvertedEnhancedMRImageStorage",
    "1.2.840.10008.5.1.4.1.1.4.4",
    "Legacy Converted Enhanced MR Image Storage",
);

/// Nuclear Medicine Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.5
/// - **UID Type:** SOP Class
pub static NuclearMedicineImageStorage_Retired: UID = UID::new(
    "NuclearMedicineImageStorage_Retired",
    "1.2.840.10008.5.1.4.1.1.5",
    "Nuclear Medicine Image Storage (Retired)",
);

/// Ultrasound Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6
/// - **UID Type:** SOP Class
pub static UltrasoundImageStorage_Retired: UID = UID::new(
    "UltrasoundImageStorage_Retired",
    "1.2.840.10008.5.1.4.1.1.6",
    "Ultrasound Image Storage (Retired)",
);

/// Ultrasound Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6.1
/// - **UID Type:** SOP Class
pub static UltrasoundImageStorage: UID = UID::new(
    "UltrasoundImageStorage",
    "1.2.840.10008.5.1.4.1.1.6.1",
    "Ultrasound Image Storage",
);

/// Enhanced US Volume Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6.2
/// - **UID Type:** SOP Class
pub static EnhancedUSVolumeStorage: UID = UID::new(
    "EnhancedUSVolumeStorage",
    "1.2.840.10008.5.1.4.1.1.6.2",
    "Enhanced US Volume Storage",
);

/// Photoacoustic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6.3
/// - **UID Type:** SOP Class
pub static PhotoacousticImageStorage: UID = UID::new(
    "PhotoacousticImageStorage",
    "1.2.840.10008.5.1.4.1.1.6.3",
    "Photoacoustic Image Storage",
);

/// Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7
/// - **UID Type:** SOP Class
pub static SecondaryCaptureImageStorage: UID = UID::new(
    "SecondaryCaptureImageStorage",
    "1.2.840.10008.5.1.4.1.1.7",
    "Secondary Capture Image Storage",
);

/// Multi-frame Single Bit Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.1
/// - **UID Type:** SOP Class
pub static MultiframeSingleBitSecondaryCaptureImageStorage: UID = UID::new(
    "MultiframeSingleBitSecondaryCaptureImageStorage",
    "1.2.840.10008.5.1.4.1.1.7.1",
    "Multi-frame Single Bit Secondary Capture Image Storage",
);

/// Multi-frame Grayscale Byte Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.2
/// - **UID Type:** SOP Class
pub static MultiframeGrayscaleByteSecondaryCaptureImageStorage: UID = UID::new(
    "MultiframeGrayscaleByteSecondaryCaptureImageStorage",
    "1.2.840.10008.5.1.4.1.1.7.2",
    "Multi-frame Grayscale Byte Secondary Capture Image Storage",
);

/// Multi-frame Grayscale Word Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.3
/// - **UID Type:** SOP Class
pub static MultiframeGrayscaleWordSecondaryCaptureImageStorage: UID = UID::new(
    "MultiframeGrayscaleWordSecondaryCaptureImageStorage",
    "1.2.840.10008.5.1.4.1.1.7.3",
    "Multi-frame Grayscale Word Secondary Capture Image Storage",
);

/// Multi-frame True Color Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.4
/// - **UID Type:** SOP Class
pub static MultiframeTrueColorSecondaryCaptureImageStorage: UID = UID::new(
    "MultiframeTrueColorSecondaryCaptureImageStorage",
    "1.2.840.10008.5.1.4.1.1.7.4",
    "Multi-frame True Color Secondary Capture Image Storage",
);

/// Standalone Overlay Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.8
/// - **UID Type:** SOP Class
pub static StandaloneOverlayStorage: UID = UID::new(
    "StandaloneOverlayStorage",
    "1.2.840.10008.5.1.4.1.1.8",
    "Standalone Overlay Storage (Retired)",
);

/// Standalone Curve Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9
/// - **UID Type:** SOP Class
pub static StandaloneCurveStorage: UID = UID::new(
    "StandaloneCurveStorage",
    "1.2.840.10008.5.1.4.1.1.9",
    "Standalone Curve Storage (Retired)",
);

/// Waveform Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1
/// - **UID Type:** SOP Class
pub static WaveformStorageTrial: UID = UID::new(
    "WaveformStorageTrial",
    "1.2.840.10008.5.1.4.1.1.9.1",
    "Waveform Storage - Trial (Retired)",
);

/// 12-lead ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.1
/// - **UID Type:** SOP Class
pub static Tag_12leadECGWaveformStorage: UID = UID::new(
    "Tag_12leadECGWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.1.1",
    "12-lead ECG Waveform Storage",
);

/// General ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.2
/// - **UID Type:** SOP Class
pub static GeneralECGWaveformStorage: UID = UID::new(
    "GeneralECGWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.1.2",
    "General ECG Waveform Storage",
);

/// Ambulatory ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.3
/// - **UID Type:** SOP Class
pub static AmbulatoryECGWaveformStorage: UID = UID::new(
    "AmbulatoryECGWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.1.3",
    "Ambulatory ECG Waveform Storage",
);

/// General 32-bit ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.4
/// - **UID Type:** SOP Class
pub static General32bitECGWaveformStorage: UID = UID::new(
    "General32bitECGWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.1.4",
    "General 32-bit ECG Waveform Storage",
);

/// Hemodynamic Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.2.1
/// - **UID Type:** SOP Class
pub static HemodynamicWaveformStorage: UID = UID::new(
    "HemodynamicWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.2.1",
    "Hemodynamic Waveform Storage",
);

/// Cardiac Electrophysiology Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.3.1
/// - **UID Type:** SOP Class
pub static CardiacElectrophysiologyWaveformStorage: UID = UID::new(
    "CardiacElectrophysiologyWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.3.1",
    "Cardiac Electrophysiology Waveform Storage",
);

/// Basic Voice Audio Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.4.1
/// - **UID Type:** SOP Class
pub static BasicVoiceAudioWaveformStorage: UID = UID::new(
    "BasicVoiceAudioWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.4.1",
    "Basic Voice Audio Waveform Storage",
);

/// General Audio Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.4.2
/// - **UID Type:** SOP Class
pub static GeneralAudioWaveformStorage: UID = UID::new(
    "GeneralAudioWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.4.2",
    "General Audio Waveform Storage",
);

/// Arterial Pulse Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.5.1
/// - **UID Type:** SOP Class
pub static ArterialPulseWaveformStorage: UID = UID::new(
    "ArterialPulseWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.5.1",
    "Arterial Pulse Waveform Storage",
);

/// Respiratory Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.6.1
/// - **UID Type:** SOP Class
pub static RespiratoryWaveformStorage: UID = UID::new(
    "RespiratoryWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.6.1",
    "Respiratory Waveform Storage",
);

/// Multi-channel Respiratory Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.6.2
/// - **UID Type:** SOP Class
pub static MultichannelRespiratoryWaveformStorage: UID = UID::new(
    "MultichannelRespiratoryWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.6.2",
    "Multi-channel Respiratory Waveform Storage",
);

/// Routine Scalp Electroencephalogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.1
/// - **UID Type:** SOP Class
pub static RoutineScalpElectroencephalogramWaveformStorage: UID = UID::new(
    "RoutineScalpElectroencephalogramWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.7.1",
    "Routine Scalp Electroencephalogram Waveform Storage",
);

/// Electromyogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.2
/// - **UID Type:** SOP Class
pub static ElectromyogramWaveformStorage: UID = UID::new(
    "ElectromyogramWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.7.2",
    "Electromyogram Waveform Storage",
);

/// Electrooculogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.3
/// - **UID Type:** SOP Class
pub static ElectrooculogramWaveformStorage: UID = UID::new(
    "ElectrooculogramWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.7.3",
    "Electrooculogram Waveform Storage",
);

/// Sleep Electroencephalogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.4
/// - **UID Type:** SOP Class
pub static SleepElectroencephalogramWaveformStorage: UID = UID::new(
    "SleepElectroencephalogramWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.7.4",
    "Sleep Electroencephalogram Waveform Storage",
);

/// Body Position Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.8.1
/// - **UID Type:** SOP Class
pub static BodyPositionWaveformStorage: UID = UID::new(
    "BodyPositionWaveformStorage",
    "1.2.840.10008.5.1.4.1.1.9.8.1",
    "Body Position Waveform Storage",
);

/// Standalone Modality LUT Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.10
/// - **UID Type:** SOP Class
pub static StandaloneModalityLUTStorage: UID = UID::new(
    "StandaloneModalityLUTStorage",
    "1.2.840.10008.5.1.4.1.1.10",
    "Standalone Modality LUT Storage (Retired)",
);

/// Standalone VOI LUT Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11
/// - **UID Type:** SOP Class
pub static StandaloneVOILUTStorage: UID = UID::new(
    "StandaloneVOILUTStorage",
    "1.2.840.10008.5.1.4.1.1.11",
    "Standalone VOI LUT Storage (Retired)",
);

/// Grayscale Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.1
/// - **UID Type:** SOP Class
pub static GrayscaleSoftcopyPresentationStateStorage: UID = UID::new(
    "GrayscaleSoftcopyPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.1",
    "Grayscale Softcopy Presentation State Storage",
);

/// Color Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.2
/// - **UID Type:** SOP Class
pub static ColorSoftcopyPresentationStateStorage: UID = UID::new(
    "ColorSoftcopyPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.2",
    "Color Softcopy Presentation State Storage",
);

/// Pseudo-Color Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.3
/// - **UID Type:** SOP Class
pub static PseudoColorSoftcopyPresentationStateStorage: UID = UID::new(
    "PseudoColorSoftcopyPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.3",
    "Pseudo-Color Softcopy Presentation State Storage",
);

/// Blending Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.4
/// - **UID Type:** SOP Class
pub static BlendingSoftcopyPresentationStateStorage: UID = UID::new(
    "BlendingSoftcopyPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.4",
    "Blending Softcopy Presentation State Storage",
);

/// XA/XRF Grayscale Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.5
/// - **UID Type:** SOP Class
pub static XAXRFGrayscaleSoftcopyPresentationStateStorage: UID = UID::new(
    "XAXRFGrayscaleSoftcopyPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.5",
    "XA/XRF Grayscale Softcopy Presentation State Storage",
);

/// Grayscale Planar MPR Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.6
/// - **UID Type:** SOP Class
pub static GrayscalePlanarMPRVolumetricPresentationStateStorage: UID = UID::new(
    "GrayscalePlanarMPRVolumetricPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.6",
    "Grayscale Planar MPR Volumetric Presentation State Storage",
);

/// Compositing Planar MPR Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.7
/// - **UID Type:** SOP Class
pub static CompositingPlanarMPRVolumetricPresentationStateStorage: UID = UID::new(
    "CompositingPlanarMPRVolumetricPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.7",
    "Compositing Planar MPR Volumetric Presentation State Storage",
);

/// Advanced Blending Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.8
/// - **UID Type:** SOP Class
pub static AdvancedBlendingPresentationStateStorage: UID = UID::new(
    "AdvancedBlendingPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.8",
    "Advanced Blending Presentation State Storage",
);

/// Volume Rendering Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.9
/// - **UID Type:** SOP Class
pub static VolumeRenderingVolumetricPresentationStateStorage: UID = UID::new(
    "VolumeRenderingVolumetricPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.9",
    "Volume Rendering Volumetric Presentation State Storage",
);

/// Segmented Volume Rendering Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.10
/// - **UID Type:** SOP Class
pub static SegmentedVolumeRenderingVolumetricPresentationStateStorage: UID = UID::new(
    "SegmentedVolumeRenderingVolumetricPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.10",
    "Segmented Volume Rendering Volumetric Presentation State Storage",
);

/// Multiple Volume Rendering Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.11
/// - **UID Type:** SOP Class
pub static MultipleVolumeRenderingVolumetricPresentationStateStorage: UID = UID::new(
    "MultipleVolumeRenderingVolumetricPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.11",
    "Multiple Volume Rendering Volumetric Presentation State Storage",
);

/// Variable Modality LUT Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.12
/// - **UID Type:** SOP Class
pub static VariableModalityLUTSoftcopyPresentationStateStorage: UID = UID::new(
    "VariableModalityLUTSoftcopyPresentationStateStorage",
    "1.2.840.10008.5.1.4.1.1.11.12",
    "Variable Modality LUT Softcopy Presentation State Storage",
);

/// X-Ray Angiographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.1
/// - **UID Type:** SOP Class
pub static XRayAngiographicImageStorage: UID = UID::new(
    "XRayAngiographicImageStorage",
    "1.2.840.10008.5.1.4.1.1.12.1",
    "X-Ray Angiographic Image Storage",
);

/// Enhanced XA Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.1.1
/// - **UID Type:** SOP Class
pub static EnhancedXAImageStorage: UID = UID::new(
    "EnhancedXAImageStorage",
    "1.2.840.10008.5.1.4.1.1.12.1.1",
    "Enhanced XA Image Storage",
);

/// X-Ray Radiofluoroscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.2
/// - **UID Type:** SOP Class
pub static XRayRadiofluoroscopicImageStorage: UID = UID::new(
    "XRayRadiofluoroscopicImageStorage",
    "1.2.840.10008.5.1.4.1.1.12.2",
    "X-Ray Radiofluoroscopic Image Storage",
);

/// Enhanced XRF Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.2.1
/// - **UID Type:** SOP Class
pub static EnhancedXRFImageStorage: UID = UID::new(
    "EnhancedXRFImageStorage",
    "1.2.840.10008.5.1.4.1.1.12.2.1",
    "Enhanced XRF Image Storage",
);

/// X-Ray Angiographic Bi-Plane Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.3
/// - **UID Type:** SOP Class
pub static XRayAngiographicBiPlaneImageStorage: UID = UID::new(
    "XRayAngiographicBiPlaneImageStorage",
    "1.2.840.10008.5.1.4.1.1.12.3",
    "X-Ray Angiographic Bi-Plane Image Storage (Retired)",
);

/// X-Ray 3D Angiographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.1
/// - **UID Type:** SOP Class
pub static XRay3DAngiographicImageStorage: UID = UID::new(
    "XRay3DAngiographicImageStorage",
    "1.2.840.10008.5.1.4.1.1.13.1.1",
    "X-Ray 3D Angiographic Image Storage",
);

/// X-Ray 3D Craniofacial Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.2
/// - **UID Type:** SOP Class
pub static XRay3DCraniofacialImageStorage: UID = UID::new(
    "XRay3DCraniofacialImageStorage",
    "1.2.840.10008.5.1.4.1.1.13.1.2",
    "X-Ray 3D Craniofacial Image Storage",
);

/// Breast Tomosynthesis Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.3
/// - **UID Type:** SOP Class
pub static BreastTomosynthesisImageStorage: UID = UID::new(
    "BreastTomosynthesisImageStorage",
    "1.2.840.10008.5.1.4.1.1.13.1.3",
    "Breast Tomosynthesis Image Storage",
);

/// Breast Projection X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.4
/// - **UID Type:** SOP Class
pub static BreastProjectionXRayImageStorageForPresentation: UID = UID::new(
    "BreastProjectionXRayImageStorageForPresentation",
    "1.2.840.10008.5.1.4.1.1.13.1.4",
    "Breast Projection X-Ray Image Storage - For Presentation",
);

/// Breast Projection X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.5
/// - **UID Type:** SOP Class
pub static BreastProjectionXRayImageStorageForProcessing: UID = UID::new(
    "BreastProjectionXRayImageStorageForProcessing",
    "1.2.840.10008.5.1.4.1.1.13.1.5",
    "Breast Projection X-Ray Image Storage - For Processing",
);

/// Intravascular Optical Coherence Tomography Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.14.1
/// - **UID Type:** SOP Class
pub static IntravascularOpticalCoherenceTomographyImageStorageForPresentation: UID = UID::new(
    "IntravascularOpticalCoherenceTomographyImageStorageForPresentation",
    "1.2.840.10008.5.1.4.1.1.14.1",
    "Intravascular Optical Coherence Tomography Image Storage - For Presentation",
);

/// Intravascular Optical Coherence Tomography Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.14.2
/// - **UID Type:** SOP Class
pub static IntravascularOpticalCoherenceTomographyImageStorageForProcessing: UID = UID::new(
    "IntravascularOpticalCoherenceTomographyImageStorageForProcessing",
    "1.2.840.10008.5.1.4.1.1.14.2",
    "Intravascular Optical Coherence Tomography Image Storage - For Processing",
);

/// Nuclear Medicine Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.20
/// - **UID Type:** SOP Class
pub static NuclearMedicineImageStorage: UID = UID::new(
    "NuclearMedicineImageStorage",
    "1.2.840.10008.5.1.4.1.1.20",
    "Nuclear Medicine Image Storage",
);

/// Parametric Map Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.30
/// - **UID Type:** SOP Class
pub static ParametricMapStorage: UID = UID::new(
    "ParametricMapStorage",
    "1.2.840.10008.5.1.4.1.1.30",
    "Parametric Map Storage",
);

/// Raw Data Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66
/// - **UID Type:** SOP Class
pub static RawDataStorage: UID = UID::new(
    "RawDataStorage",
    "1.2.840.10008.5.1.4.1.1.66",
    "Raw Data Storage",
);

/// Spatial Registration Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.1
/// - **UID Type:** SOP Class
pub static SpatialRegistrationStorage: UID = UID::new(
    "SpatialRegistrationStorage",
    "1.2.840.10008.5.1.4.1.1.66.1",
    "Spatial Registration Storage",
);

/// Spatial Fiducials Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.2
/// - **UID Type:** SOP Class
pub static SpatialFiducialsStorage: UID = UID::new(
    "SpatialFiducialsStorage",
    "1.2.840.10008.5.1.4.1.1.66.2",
    "Spatial Fiducials Storage",
);

/// Deformable Spatial Registration Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.3
/// - **UID Type:** SOP Class
pub static DeformableSpatialRegistrationStorage: UID = UID::new(
    "DeformableSpatialRegistrationStorage",
    "1.2.840.10008.5.1.4.1.1.66.3",
    "Deformable Spatial Registration Storage",
);

/// Segmentation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.4
/// - **UID Type:** SOP Class
pub static SegmentationStorage: UID = UID::new(
    "SegmentationStorage",
    "1.2.840.10008.5.1.4.1.1.66.4",
    "Segmentation Storage",
);

/// Surface Segmentation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.5
/// - **UID Type:** SOP Class
pub static SurfaceSegmentationStorage: UID = UID::new(
    "SurfaceSegmentationStorage",
    "1.2.840.10008.5.1.4.1.1.66.5",
    "Surface Segmentation Storage",
);

/// Tractography Results Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.6
/// - **UID Type:** SOP Class
pub static TractographyResultsStorage: UID = UID::new(
    "TractographyResultsStorage",
    "1.2.840.10008.5.1.4.1.1.66.6",
    "Tractography Results Storage",
);

/// Real World Value Mapping Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.67
/// - **UID Type:** SOP Class
pub static RealWorldValueMappingStorage: UID = UID::new(
    "RealWorldValueMappingStorage",
    "1.2.840.10008.5.1.4.1.1.67",
    "Real World Value Mapping Storage",
);

/// Surface Scan Mesh Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.68.1
/// - **UID Type:** SOP Class
pub static SurfaceScanMeshStorage: UID = UID::new(
    "SurfaceScanMeshStorage",
    "1.2.840.10008.5.1.4.1.1.68.1",
    "Surface Scan Mesh Storage",
);

/// Surface Scan Point Cloud Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.68.2
/// - **UID Type:** SOP Class
pub static SurfaceScanPointCloudStorage: UID = UID::new(
    "SurfaceScanPointCloudStorage",
    "1.2.840.10008.5.1.4.1.1.68.2",
    "Surface Scan Point Cloud Storage",
);

/// VL Image Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1
/// - **UID Type:** SOP Class
pub static VLImageStorageTrial: UID = UID::new(
    "VLImageStorageTrial",
    "1.2.840.10008.5.1.4.1.1.77.1",
    "VL Image Storage - Trial (Retired)",
);

/// VL Multi-frame Image Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.2
/// - **UID Type:** SOP Class
pub static VLMultiframeImageStorageTrial: UID = UID::new(
    "VLMultiframeImageStorageTrial",
    "1.2.840.10008.5.1.4.1.1.77.2",
    "VL Multi-frame Image Storage - Trial (Retired)",
);

/// VL Endoscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.1
/// - **UID Type:** SOP Class
pub static VLEndoscopicImageStorage: UID = UID::new(
    "VLEndoscopicImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.1",
    "VL Endoscopic Image Storage",
);

/// Video Endoscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.1.1
/// - **UID Type:** SOP Class
pub static VideoEndoscopicImageStorage: UID = UID::new(
    "VideoEndoscopicImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.1.1",
    "Video Endoscopic Image Storage",
);

/// VL Microscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.2
/// - **UID Type:** SOP Class
pub static VLMicroscopicImageStorage: UID = UID::new(
    "VLMicroscopicImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.2",
    "VL Microscopic Image Storage",
);

/// Video Microscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.2.1
/// - **UID Type:** SOP Class
pub static VideoMicroscopicImageStorage: UID = UID::new(
    "VideoMicroscopicImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.2.1",
    "Video Microscopic Image Storage",
);

/// VL Slide-Coordinates Microscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.3
/// - **UID Type:** SOP Class
pub static VLSlideCoordinatesMicroscopicImageStorage: UID = UID::new(
    "VLSlideCoordinatesMicroscopicImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.3",
    "VL Slide-Coordinates Microscopic Image Storage",
);

/// VL Photographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.4
/// - **UID Type:** SOP Class
pub static VLPhotographicImageStorage: UID = UID::new(
    "VLPhotographicImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.4",
    "VL Photographic Image Storage",
);

/// Video Photographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.4.1
/// - **UID Type:** SOP Class
pub static VideoPhotographicImageStorage: UID = UID::new(
    "VideoPhotographicImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.4.1",
    "Video Photographic Image Storage",
);

/// Ophthalmic Photography 8 Bit Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.1
/// - **UID Type:** SOP Class
pub static OphthalmicPhotography8BitImageStorage: UID = UID::new(
    "OphthalmicPhotography8BitImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.1",
    "Ophthalmic Photography 8 Bit Image Storage",
);

/// Ophthalmic Photography 16 Bit Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.2
/// - **UID Type:** SOP Class
pub static OphthalmicPhotography16BitImageStorage: UID = UID::new(
    "OphthalmicPhotography16BitImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.2",
    "Ophthalmic Photography 16 Bit Image Storage",
);

/// Stereometric Relationship Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.3
/// - **UID Type:** SOP Class
pub static StereometricRelationshipStorage: UID = UID::new(
    "StereometricRelationshipStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.3",
    "Stereometric Relationship Storage",
);

/// Ophthalmic Tomography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.4
/// - **UID Type:** SOP Class
pub static OphthalmicTomographyImageStorage: UID = UID::new(
    "OphthalmicTomographyImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.4",
    "Ophthalmic Tomography Image Storage",
);

/// Wide Field Ophthalmic Photography Stereographic Projection Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.5
/// - **UID Type:** SOP Class
pub static WideFieldOphthalmicPhotographyStereographicProjectionImageStorage: UID = UID::new(
    "WideFieldOphthalmicPhotographyStereographicProjectionImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.5",
    "Wide Field Ophthalmic Photography Stereographic Projection Image Storage",
);

/// Wide Field Ophthalmic Photography 3D Coordinates Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.6
/// - **UID Type:** SOP Class
pub static WideFieldOphthalmicPhotography3DCoordinatesImageStorage: UID = UID::new(
    "WideFieldOphthalmicPhotography3DCoordinatesImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.6",
    "Wide Field Ophthalmic Photography 3D Coordinates Image Storage",
);

/// Ophthalmic Optical Coherence Tomography En Face Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.7
/// - **UID Type:** SOP Class
pub static OphthalmicOpticalCoherenceTomographyEnFaceImageStorage: UID = UID::new(
    "OphthalmicOpticalCoherenceTomographyEnFaceImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.7",
    "Ophthalmic Optical Coherence Tomography En Face Image Storage",
);

/// Ophthalmic Optical Coherence Tomography B-scan Volume Analysis Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.8
/// - **UID Type:** SOP Class
pub static OphthalmicOpticalCoherenceTomographyBscanVolumeAnalysisStorage: UID = UID::new(
    "OphthalmicOpticalCoherenceTomographyBscanVolumeAnalysisStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.5.8",
    "Ophthalmic Optical Coherence Tomography B-scan Volume Analysis Storage",
);

/// VL Whole Slide Microscopy Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.6
/// - **UID Type:** SOP Class
pub static VLWholeSlideMicroscopyImageStorage: UID = UID::new(
    "VLWholeSlideMicroscopyImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.6",
    "VL Whole Slide Microscopy Image Storage",
);

/// Dermoscopic Photography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.7
/// - **UID Type:** SOP Class
pub static DermoscopicPhotographyImageStorage: UID = UID::new(
    "DermoscopicPhotographyImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.7",
    "Dermoscopic Photography Image Storage",
);

/// Confocal Microscopy Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.8
/// - **UID Type:** SOP Class
pub static ConfocalMicroscopyImageStorage: UID = UID::new(
    "ConfocalMicroscopyImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.8",
    "Confocal Microscopy Image Storage",
);

/// Confocal Microscopy Tiled Pyramidal Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.9
/// - **UID Type:** SOP Class
pub static ConfocalMicroscopyTiledPyramidalImageStorage: UID = UID::new(
    "ConfocalMicroscopyTiledPyramidalImageStorage",
    "1.2.840.10008.5.1.4.1.1.77.1.9",
    "Confocal Microscopy Tiled Pyramidal Image Storage",
);

/// Lensometry Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.1
/// - **UID Type:** SOP Class
pub static LensometryMeasurementsStorage: UID = UID::new(
    "LensometryMeasurementsStorage",
    "1.2.840.10008.5.1.4.1.1.78.1",
    "Lensometry Measurements Storage",
);

/// Autorefraction Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.2
/// - **UID Type:** SOP Class
pub static AutorefractionMeasurementsStorage: UID = UID::new(
    "AutorefractionMeasurementsStorage",
    "1.2.840.10008.5.1.4.1.1.78.2",
    "Autorefraction Measurements Storage",
);

/// Keratometry Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.3
/// - **UID Type:** SOP Class
pub static KeratometryMeasurementsStorage: UID = UID::new(
    "KeratometryMeasurementsStorage",
    "1.2.840.10008.5.1.4.1.1.78.3",
    "Keratometry Measurements Storage",
);

/// Subjective Refraction Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.4
/// - **UID Type:** SOP Class
pub static SubjectiveRefractionMeasurementsStorage: UID = UID::new(
    "SubjectiveRefractionMeasurementsStorage",
    "1.2.840.10008.5.1.4.1.1.78.4",
    "Subjective Refraction Measurements Storage",
);

/// Visual Acuity Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.5
/// - **UID Type:** SOP Class
pub static VisualAcuityMeasurementsStorage: UID = UID::new(
    "VisualAcuityMeasurementsStorage",
    "1.2.840.10008.5.1.4.1.1.78.5",
    "Visual Acuity Measurements Storage",
);

/// Spectacle Prescription Report Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.6
/// - **UID Type:** SOP Class
pub static SpectaclePrescriptionReportStorage: UID = UID::new(
    "SpectaclePrescriptionReportStorage",
    "1.2.840.10008.5.1.4.1.1.78.6",
    "Spectacle Prescription Report Storage",
);

/// Ophthalmic Axial Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.7
/// - **UID Type:** SOP Class
pub static OphthalmicAxialMeasurementsStorage: UID = UID::new(
    "OphthalmicAxialMeasurementsStorage",
    "1.2.840.10008.5.1.4.1.1.78.7",
    "Ophthalmic Axial Measurements Storage",
);

/// Intraocular Lens Calculations Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.8
/// - **UID Type:** SOP Class
pub static IntraocularLensCalculationsStorage: UID = UID::new(
    "IntraocularLensCalculationsStorage",
    "1.2.840.10008.5.1.4.1.1.78.8",
    "Intraocular Lens Calculations Storage",
);

/// Macular Grid Thickness and Volume Report Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.79.1
/// - **UID Type:** SOP Class
pub static MacularGridThicknessandVolumeReportStorage: UID = UID::new(
    "MacularGridThicknessandVolumeReportStorage",
    "1.2.840.10008.5.1.4.1.1.79.1",
    "Macular Grid Thickness and Volume Report Storage",
);

/// Ophthalmic Visual Field Static Perimetry Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.80.1
/// - **UID Type:** SOP Class
pub static OphthalmicVisualFieldStaticPerimetryMeasurementsStorage: UID = UID::new(
    "OphthalmicVisualFieldStaticPerimetryMeasurementsStorage",
    "1.2.840.10008.5.1.4.1.1.80.1",
    "Ophthalmic Visual Field Static Perimetry Measurements Storage",
);

/// Ophthalmic Thickness Map Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.81.1
/// - **UID Type:** SOP Class
pub static OphthalmicThicknessMapStorage: UID = UID::new(
    "OphthalmicThicknessMapStorage",
    "1.2.840.10008.5.1.4.1.1.81.1",
    "Ophthalmic Thickness Map Storage",
);

/// Corneal Topography Map Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.82.1
/// - **UID Type:** SOP Class
pub static CornealTopographyMapStorage: UID = UID::new(
    "CornealTopographyMapStorage",
    "1.2.840.10008.5.1.4.1.1.82.1",
    "Corneal Topography Map Storage",
);

/// Text SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.1
/// - **UID Type:** SOP Class
pub static TextSRStorageTrial: UID = UID::new(
    "TextSRStorageTrial",
    "1.2.840.10008.5.1.4.1.1.88.1",
    "Text SR Storage - Trial (Retired)",
);

/// Audio SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.2
/// - **UID Type:** SOP Class
pub static AudioSRStorageTrial: UID = UID::new(
    "AudioSRStorageTrial",
    "1.2.840.10008.5.1.4.1.1.88.2",
    "Audio SR Storage - Trial (Retired)",
);

/// Detail SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.3
/// - **UID Type:** SOP Class
pub static DetailSRStorageTrial: UID = UID::new(
    "DetailSRStorageTrial",
    "1.2.840.10008.5.1.4.1.1.88.3",
    "Detail SR Storage - Trial (Retired)",
);

/// Comprehensive SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.4
/// - **UID Type:** SOP Class
pub static ComprehensiveSRStorageTrial: UID = UID::new(
    "ComprehensiveSRStorageTrial",
    "1.2.840.10008.5.1.4.1.1.88.4",
    "Comprehensive SR Storage - Trial (Retired)",
);

/// Basic Text SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.11
/// - **UID Type:** SOP Class
pub static BasicTextSRStorage: UID = UID::new(
    "BasicTextSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.11",
    "Basic Text SR Storage",
);

/// Enhanced SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.22
/// - **UID Type:** SOP Class
pub static EnhancedSRStorage: UID = UID::new(
    "EnhancedSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.22",
    "Enhanced SR Storage",
);

/// Comprehensive SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.33
/// - **UID Type:** SOP Class
pub static ComprehensiveSRStorage: UID = UID::new(
    "ComprehensiveSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.33",
    "Comprehensive SR Storage",
);

/// Comprehensive 3D SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.34
/// - **UID Type:** SOP Class
pub static Comprehensive3DSRStorage: UID = UID::new(
    "Comprehensive3DSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.34",
    "Comprehensive 3D SR Storage",
);

/// Extensible SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.35
/// - **UID Type:** SOP Class
pub static ExtensibleSRStorage: UID = UID::new(
    "ExtensibleSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.35",
    "Extensible SR Storage",
);

/// Procedure Log Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.40
/// - **UID Type:** SOP Class
pub static ProcedureLogStorage: UID = UID::new(
    "ProcedureLogStorage",
    "1.2.840.10008.5.1.4.1.1.88.40",
    "Procedure Log Storage",
);

/// Mammography CAD SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.50
/// - **UID Type:** SOP Class
pub static MammographyCADSRStorage: UID = UID::new(
    "MammographyCADSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.50",
    "Mammography CAD SR Storage",
);

/// Key Object Selection Document Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.59
/// - **UID Type:** SOP Class
pub static KeyObjectSelectionDocumentStorage: UID = UID::new(
    "KeyObjectSelectionDocumentStorage",
    "1.2.840.10008.5.1.4.1.1.88.59",
    "Key Object Selection Document Storage",
);

/// Chest CAD SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.65
/// - **UID Type:** SOP Class
pub static ChestCADSRStorage: UID = UID::new(
    "ChestCADSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.65",
    "Chest CAD SR Storage",
);

/// X-Ray Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.67
/// - **UID Type:** SOP Class
pub static XRayRadiationDoseSRStorage: UID = UID::new(
    "XRayRadiationDoseSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.67",
    "X-Ray Radiation Dose SR Storage",
);

/// Radiopharmaceutical Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.68
/// - **UID Type:** SOP Class
pub static RadiopharmaceuticalRadiationDoseSRStorage: UID = UID::new(
    "RadiopharmaceuticalRadiationDoseSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.68",
    "Radiopharmaceutical Radiation Dose SR Storage",
);

/// Colon CAD SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.69
/// - **UID Type:** SOP Class
pub static ColonCADSRStorage: UID = UID::new(
    "ColonCADSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.69",
    "Colon CAD SR Storage",
);

/// Implantation Plan SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.70
/// - **UID Type:** SOP Class
pub static ImplantationPlanSRStorage: UID = UID::new(
    "ImplantationPlanSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.70",
    "Implantation Plan SR Storage",
);

/// Acquisition Context SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.71
/// - **UID Type:** SOP Class
pub static AcquisitionContextSRStorage: UID = UID::new(
    "AcquisitionContextSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.71",
    "Acquisition Context SR Storage",
);

/// Simplified Adult Echo SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.72
/// - **UID Type:** SOP Class
pub static SimplifiedAdultEchoSRStorage: UID = UID::new(
    "SimplifiedAdultEchoSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.72",
    "Simplified Adult Echo SR Storage",
);

/// Patient Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.73
/// - **UID Type:** SOP Class
pub static PatientRadiationDoseSRStorage: UID = UID::new(
    "PatientRadiationDoseSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.73",
    "Patient Radiation Dose SR Storage",
);

/// Planned Imaging Agent Administration SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.74
/// - **UID Type:** SOP Class
pub static PlannedImagingAgentAdministrationSRStorage: UID = UID::new(
    "PlannedImagingAgentAdministrationSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.74",
    "Planned Imaging Agent Administration SR Storage",
);

/// Performed Imaging Agent Administration SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.75
/// - **UID Type:** SOP Class
pub static PerformedImagingAgentAdministrationSRStorage: UID = UID::new(
    "PerformedImagingAgentAdministrationSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.75",
    "Performed Imaging Agent Administration SR Storage",
);

/// Enhanced X-Ray Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.76
/// - **UID Type:** SOP Class
pub static EnhancedXRayRadiationDoseSRStorage: UID = UID::new(
    "EnhancedXRayRadiationDoseSRStorage",
    "1.2.840.10008.5.1.4.1.1.88.76",
    "Enhanced X-Ray Radiation Dose SR Storage",
);

/// Content Assessment Results Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.90.1
/// - **UID Type:** SOP Class
pub static ContentAssessmentResultsStorage: UID = UID::new(
    "ContentAssessmentResultsStorage",
    "1.2.840.10008.5.1.4.1.1.90.1",
    "Content Assessment Results Storage",
);

/// Microscopy Bulk Simple Annotations Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.91.1
/// - **UID Type:** SOP Class
pub static MicroscopyBulkSimpleAnnotationsStorage: UID = UID::new(
    "MicroscopyBulkSimpleAnnotationsStorage",
    "1.2.840.10008.5.1.4.1.1.91.1",
    "Microscopy Bulk Simple Annotations Storage",
);

/// Encapsulated PDF Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.1
/// - **UID Type:** SOP Class
pub static EncapsulatedPDFStorage: UID = UID::new(
    "EncapsulatedPDFStorage",
    "1.2.840.10008.5.1.4.1.1.104.1",
    "Encapsulated PDF Storage",
);

/// Encapsulated CDA Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.2
/// - **UID Type:** SOP Class
pub static EncapsulatedCDAStorage: UID = UID::new(
    "EncapsulatedCDAStorage",
    "1.2.840.10008.5.1.4.1.1.104.2",
    "Encapsulated CDA Storage",
);

/// Encapsulated STL Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.3
/// - **UID Type:** SOP Class
pub static EncapsulatedSTLStorage: UID = UID::new(
    "EncapsulatedSTLStorage",
    "1.2.840.10008.5.1.4.1.1.104.3",
    "Encapsulated STL Storage",
);

/// Encapsulated OBJ Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.4
/// - **UID Type:** SOP Class
pub static EncapsulatedOBJStorage: UID = UID::new(
    "EncapsulatedOBJStorage",
    "1.2.840.10008.5.1.4.1.1.104.4",
    "Encapsulated OBJ Storage",
);

/// Encapsulated MTL Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.5
/// - **UID Type:** SOP Class
pub static EncapsulatedMTLStorage: UID = UID::new(
    "EncapsulatedMTLStorage",
    "1.2.840.10008.5.1.4.1.1.104.5",
    "Encapsulated MTL Storage",
);

/// Positron Emission Tomography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.128
/// - **UID Type:** SOP Class
pub static PositronEmissionTomographyImageStorage: UID = UID::new(
    "PositronEmissionTomographyImageStorage",
    "1.2.840.10008.5.1.4.1.1.128",
    "Positron Emission Tomography Image Storage",
);

/// Legacy Converted Enhanced PET Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.128.1
/// - **UID Type:** SOP Class
pub static LegacyConvertedEnhancedPETImageStorage: UID = UID::new(
    "LegacyConvertedEnhancedPETImageStorage",
    "1.2.840.10008.5.1.4.1.1.128.1",
    "Legacy Converted Enhanced PET Image Storage",
);

/// Standalone PET Curve Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.129
/// - **UID Type:** SOP Class
pub static StandalonePETCurveStorage: UID = UID::new(
    "StandalonePETCurveStorage",
    "1.2.840.10008.5.1.4.1.1.129",
    "Standalone PET Curve Storage (Retired)",
);

/// Enhanced PET Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.130
/// - **UID Type:** SOP Class
pub static EnhancedPETImageStorage: UID = UID::new(
    "EnhancedPETImageStorage",
    "1.2.840.10008.5.1.4.1.1.130",
    "Enhanced PET Image Storage",
);

/// Basic Structured Display Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.131
/// - **UID Type:** SOP Class
pub static BasicStructuredDisplayStorage: UID = UID::new(
    "BasicStructuredDisplayStorage",
    "1.2.840.10008.5.1.4.1.1.131",
    "Basic Structured Display Storage",
);

/// CT Defined Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.1
/// - **UID Type:** SOP Class
pub static CTDefinedProcedureProtocolStorage: UID = UID::new(
    "CTDefinedProcedureProtocolStorage",
    "1.2.840.10008.5.1.4.1.1.200.1",
    "CT Defined Procedure Protocol Storage",
);

/// CT Performed Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.2
/// - **UID Type:** SOP Class
pub static CTPerformedProcedureProtocolStorage: UID = UID::new(
    "CTPerformedProcedureProtocolStorage",
    "1.2.840.10008.5.1.4.1.1.200.2",
    "CT Performed Procedure Protocol Storage",
);

/// Protocol Approval Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.3
/// - **UID Type:** SOP Class
pub static ProtocolApprovalStorage: UID = UID::new(
    "ProtocolApprovalStorage",
    "1.2.840.10008.5.1.4.1.1.200.3",
    "Protocol Approval Storage",
);

/// Protocol Approval Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.4
/// - **UID Type:** SOP Class
pub static ProtocolApprovalInformationModelFIND: UID = UID::new(
    "ProtocolApprovalInformationModelFIND",
    "1.2.840.10008.5.1.4.1.1.200.4",
    "Protocol Approval Information Model - FIND",
);

/// Protocol Approval Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.5
/// - **UID Type:** SOP Class
pub static ProtocolApprovalInformationModelMOVE: UID = UID::new(
    "ProtocolApprovalInformationModelMOVE",
    "1.2.840.10008.5.1.4.1.1.200.5",
    "Protocol Approval Information Model - MOVE",
);

/// Protocol Approval Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.6
/// - **UID Type:** SOP Class
pub static ProtocolApprovalInformationModelGET: UID = UID::new(
    "ProtocolApprovalInformationModelGET",
    "1.2.840.10008.5.1.4.1.1.200.6",
    "Protocol Approval Information Model - GET",
);

/// XA Defined Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.7
/// - **UID Type:** SOP Class
pub static XADefinedProcedureProtocolStorage: UID = UID::new(
    "XADefinedProcedureProtocolStorage",
    "1.2.840.10008.5.1.4.1.1.200.7",
    "XA Defined Procedure Protocol Storage",
);

/// XA Performed Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.8
/// - **UID Type:** SOP Class
pub static XAPerformedProcedureProtocolStorage: UID = UID::new(
    "XAPerformedProcedureProtocolStorage",
    "1.2.840.10008.5.1.4.1.1.200.8",
    "XA Performed Procedure Protocol Storage",
);

/// Inventory Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.1
/// - **UID Type:** SOP Class
pub static InventoryStorage: UID = UID::new(
    "InventoryStorage",
    "1.2.840.10008.5.1.4.1.1.201.1",
    "Inventory Storage",
);

/// Inventory - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.2
/// - **UID Type:** SOP Class
pub static InventoryFIND: UID = UID::new(
    "InventoryFIND",
    "1.2.840.10008.5.1.4.1.1.201.2",
    "Inventory - FIND",
);

/// Inventory - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.3
/// - **UID Type:** SOP Class
pub static InventoryMOVE: UID = UID::new(
    "InventoryMOVE",
    "1.2.840.10008.5.1.4.1.1.201.3",
    "Inventory - MOVE",
);

/// Inventory - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.4
/// - **UID Type:** SOP Class
pub static InventoryGET: UID = UID::new(
    "InventoryGET",
    "1.2.840.10008.5.1.4.1.1.201.4",
    "Inventory - GET",
);

/// Inventory Creation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.5
/// - **UID Type:** SOP Class
pub static InventoryCreation: UID = UID::new(
    "InventoryCreation",
    "1.2.840.10008.5.1.4.1.1.201.5",
    "Inventory Creation",
);

/// Repository Query
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.6
/// - **UID Type:** SOP Class
pub static RepositoryQuery: UID = UID::new(
    "RepositoryQuery",
    "1.2.840.10008.5.1.4.1.1.201.6",
    "Repository Query",
);

/// Storage Management SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.1.1
/// - **UID Type:** Well-known SOP Instance
pub static StorageManagementSOPInstance: UID = UID::new(
    "StorageManagementSOPInstance",
    "1.2.840.10008.5.1.4.1.1.201.1.1",
    "Storage Management SOP Instance",
);

/// RT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.1
/// - **UID Type:** SOP Class
pub static RTImageStorage: UID = UID::new(
    "RTImageStorage",
    "1.2.840.10008.5.1.4.1.1.481.1",
    "RT Image Storage",
);

/// RT Dose Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.2
/// - **UID Type:** SOP Class
pub static RTDoseStorage: UID = UID::new(
    "RTDoseStorage",
    "1.2.840.10008.5.1.4.1.1.481.2",
    "RT Dose Storage",
);

/// RT Structure Set Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.3
/// - **UID Type:** SOP Class
pub static RTStructureSetStorage: UID = UID::new(
    "RTStructureSetStorage",
    "1.2.840.10008.5.1.4.1.1.481.3",
    "RT Structure Set Storage",
);

/// RT Beams Treatment Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.4
/// - **UID Type:** SOP Class
pub static RTBeamsTreatmentRecordStorage: UID = UID::new(
    "RTBeamsTreatmentRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.4",
    "RT Beams Treatment Record Storage",
);

/// RT Plan Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.5
/// - **UID Type:** SOP Class
pub static RTPlanStorage: UID = UID::new(
    "RTPlanStorage",
    "1.2.840.10008.5.1.4.1.1.481.5",
    "RT Plan Storage",
);

/// RT Brachy Treatment Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.6
/// - **UID Type:** SOP Class
pub static RTBrachyTreatmentRecordStorage: UID = UID::new(
    "RTBrachyTreatmentRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.6",
    "RT Brachy Treatment Record Storage",
);

/// RT Treatment Summary Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.7
/// - **UID Type:** SOP Class
pub static RTTreatmentSummaryRecordStorage: UID = UID::new(
    "RTTreatmentSummaryRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.7",
    "RT Treatment Summary Record Storage",
);

/// RT Ion Plan Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.8
/// - **UID Type:** SOP Class
pub static RTIonPlanStorage: UID = UID::new(
    "RTIonPlanStorage",
    "1.2.840.10008.5.1.4.1.1.481.8",
    "RT Ion Plan Storage",
);

/// RT Ion Beams Treatment Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.9
/// - **UID Type:** SOP Class
pub static RTIonBeamsTreatmentRecordStorage: UID = UID::new(
    "RTIonBeamsTreatmentRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.9",
    "RT Ion Beams Treatment Record Storage",
);

/// RT Physician Intent Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.10
/// - **UID Type:** SOP Class
pub static RTPhysicianIntentStorage: UID = UID::new(
    "RTPhysicianIntentStorage",
    "1.2.840.10008.5.1.4.1.1.481.10",
    "RT Physician Intent Storage",
);

/// RT Segment Annotation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.11
/// - **UID Type:** SOP Class
pub static RTSegmentAnnotationStorage: UID = UID::new(
    "RTSegmentAnnotationStorage",
    "1.2.840.10008.5.1.4.1.1.481.11",
    "RT Segment Annotation Storage",
);

/// RT Radiation Set Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.12
/// - **UID Type:** SOP Class
pub static RTRadiationSetStorage: UID = UID::new(
    "RTRadiationSetStorage",
    "1.2.840.10008.5.1.4.1.1.481.12",
    "RT Radiation Set Storage",
);

/// C-Arm Photon-Electron Radiation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.13
/// - **UID Type:** SOP Class
pub static CArmPhotonElectronRadiationStorage: UID = UID::new(
    "CArmPhotonElectronRadiationStorage",
    "1.2.840.10008.5.1.4.1.1.481.13",
    "C-Arm Photon-Electron Radiation Storage",
);

/// Tomotherapeutic Radiation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.14
/// - **UID Type:** SOP Class
pub static TomotherapeuticRadiationStorage: UID = UID::new(
    "TomotherapeuticRadiationStorage",
    "1.2.840.10008.5.1.4.1.1.481.14",
    "Tomotherapeutic Radiation Storage",
);

/// Robotic-Arm Radiation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.15
/// - **UID Type:** SOP Class
pub static RoboticArmRadiationStorage: UID = UID::new(
    "RoboticArmRadiationStorage",
    "1.2.840.10008.5.1.4.1.1.481.15",
    "Robotic-Arm Radiation Storage",
);

/// RT Radiation Record Set Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.16
/// - **UID Type:** SOP Class
pub static RTRadiationRecordSetStorage: UID = UID::new(
    "RTRadiationRecordSetStorage",
    "1.2.840.10008.5.1.4.1.1.481.16",
    "RT Radiation Record Set Storage",
);

/// RT Radiation Salvage Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.17
/// - **UID Type:** SOP Class
pub static RTRadiationSalvageRecordStorage: UID = UID::new(
    "RTRadiationSalvageRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.17",
    "RT Radiation Salvage Record Storage",
);

/// Tomotherapeutic Radiation Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.18
/// - **UID Type:** SOP Class
pub static TomotherapeuticRadiationRecordStorage: UID = UID::new(
    "TomotherapeuticRadiationRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.18",
    "Tomotherapeutic Radiation Record Storage",
);

/// C-Arm Photon-Electron Radiation Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.19
/// - **UID Type:** SOP Class
pub static CArmPhotonElectronRadiationRecordStorage: UID = UID::new(
    "CArmPhotonElectronRadiationRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.19",
    "C-Arm Photon-Electron Radiation Record Storage",
);

/// Robotic Radiation Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.20
/// - **UID Type:** SOP Class
pub static RoboticRadiationRecordStorage: UID = UID::new(
    "RoboticRadiationRecordStorage",
    "1.2.840.10008.5.1.4.1.1.481.20",
    "Robotic Radiation Record Storage",
);

/// RT Radiation Set Delivery Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.21
/// - **UID Type:** SOP Class
pub static RTRadiationSetDeliveryInstructionStorage: UID = UID::new(
    "RTRadiationSetDeliveryInstructionStorage",
    "1.2.840.10008.5.1.4.1.1.481.21",
    "RT Radiation Set Delivery Instruction Storage",
);

/// RT Treatment Preparation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.22
/// - **UID Type:** SOP Class
pub static RTTreatmentPreparationStorage: UID = UID::new(
    "RTTreatmentPreparationStorage",
    "1.2.840.10008.5.1.4.1.1.481.22",
    "RT Treatment Preparation Storage",
);

/// Enhanced RT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.23
/// - **UID Type:** SOP Class
pub static EnhancedRTImageStorage: UID = UID::new(
    "EnhancedRTImageStorage",
    "1.2.840.10008.5.1.4.1.1.481.23",
    "Enhanced RT Image Storage",
);

/// Enhanced Continuous RT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.24
/// - **UID Type:** SOP Class
pub static EnhancedContinuousRTImageStorage: UID = UID::new(
    "EnhancedContinuousRTImageStorage",
    "1.2.840.10008.5.1.4.1.1.481.24",
    "Enhanced Continuous RT Image Storage",
);

/// RT Patient Position Acquisition Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.25
/// - **UID Type:** SOP Class
pub static RTPatientPositionAcquisitionInstructionStorage: UID = UID::new(
    "RTPatientPositionAcquisitionInstructionStorage",
    "1.2.840.10008.5.1.4.1.1.481.25",
    "RT Patient Position Acquisition Instruction Storage",
);

/// DICOS CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.1
/// - **UID Type:** SOP Class
pub static DICOSCTImageStorage: UID = UID::new(
    "DICOSCTImageStorage",
    "1.2.840.10008.5.1.4.1.1.501.1",
    "DICOS CT Image Storage",
);

/// DICOS Digital X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.2.1
/// - **UID Type:** SOP Class
pub static DICOSDigitalXRayImageStorageForPresentation: UID = UID::new(
    "DICOSDigitalXRayImageStorageForPresentation",
    "1.2.840.10008.5.1.4.1.1.501.2.1",
    "DICOS Digital X-Ray Image Storage - For Presentation",
);

/// DICOS Digital X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.2.2
/// - **UID Type:** SOP Class
pub static DICOSDigitalXRayImageStorageForProcessing: UID = UID::new(
    "DICOSDigitalXRayImageStorageForProcessing",
    "1.2.840.10008.5.1.4.1.1.501.2.2",
    "DICOS Digital X-Ray Image Storage - For Processing",
);

/// DICOS Threat Detection Report Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.3
/// - **UID Type:** SOP Class
pub static DICOSThreatDetectionReportStorage: UID = UID::new(
    "DICOSThreatDetectionReportStorage",
    "1.2.840.10008.5.1.4.1.1.501.3",
    "DICOS Threat Detection Report Storage",
);

/// DICOS 2D AIT Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.4
/// - **UID Type:** SOP Class
pub static DICOS2DAITStorage: UID = UID::new(
    "DICOS2DAITStorage",
    "1.2.840.10008.5.1.4.1.1.501.4",
    "DICOS 2D AIT Storage",
);

/// DICOS 3D AIT Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.5
/// - **UID Type:** SOP Class
pub static DICOS3DAITStorage: UID = UID::new(
    "DICOS3DAITStorage",
    "1.2.840.10008.5.1.4.1.1.501.5",
    "DICOS 3D AIT Storage",
);

/// DICOS Quadrupole Resonance (QR) Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.6
/// - **UID Type:** SOP Class
pub static DICOSQuadrupoleResonanceQRStorage: UID = UID::new(
    "DICOSQuadrupoleResonanceQRStorage",
    "1.2.840.10008.5.1.4.1.1.501.6",
    "DICOS Quadrupole Resonance (QR) Storage",
);

/// Eddy Current Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.601.1
/// - **UID Type:** SOP Class
pub static EddyCurrentImageStorage: UID = UID::new(
    "EddyCurrentImageStorage",
    "1.2.840.10008.5.1.4.1.1.601.1",
    "Eddy Current Image Storage",
);

/// Eddy Current Multi-frame Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.601.2
/// - **UID Type:** SOP Class
pub static EddyCurrentMultiframeImageStorage: UID = UID::new(
    "EddyCurrentMultiframeImageStorage",
    "1.2.840.10008.5.1.4.1.1.601.2",
    "Eddy Current Multi-frame Image Storage",
);

/// Patient Root Query/Retrieve Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.1.1
/// - **UID Type:** SOP Class
pub static PatientRootQueryRetrieveInformationModelFIND: UID = UID::new(
    "PatientRootQueryRetrieveInformationModelFIND",
    "1.2.840.10008.5.1.4.1.2.1.1",
    "Patient Root Query/Retrieve Information Model - FIND",
);

/// Patient Root Query/Retrieve Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.1.2
/// - **UID Type:** SOP Class
pub static PatientRootQueryRetrieveInformationModelMOVE: UID = UID::new(
    "PatientRootQueryRetrieveInformationModelMOVE",
    "1.2.840.10008.5.1.4.1.2.1.2",
    "Patient Root Query/Retrieve Information Model - MOVE",
);

/// Patient Root Query/Retrieve Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.1.3
/// - **UID Type:** SOP Class
pub static PatientRootQueryRetrieveInformationModelGET: UID = UID::new(
    "PatientRootQueryRetrieveInformationModelGET",
    "1.2.840.10008.5.1.4.1.2.1.3",
    "Patient Root Query/Retrieve Information Model - GET",
);

/// Study Root Query/Retrieve Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.2.1
/// - **UID Type:** SOP Class
pub static StudyRootQueryRetrieveInformationModelFIND: UID = UID::new(
    "StudyRootQueryRetrieveInformationModelFIND",
    "1.2.840.10008.5.1.4.1.2.2.1",
    "Study Root Query/Retrieve Information Model - FIND",
);

/// Study Root Query/Retrieve Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.2.2
/// - **UID Type:** SOP Class
pub static StudyRootQueryRetrieveInformationModelMOVE: UID = UID::new(
    "StudyRootQueryRetrieveInformationModelMOVE",
    "1.2.840.10008.5.1.4.1.2.2.2",
    "Study Root Query/Retrieve Information Model - MOVE",
);

/// Study Root Query/Retrieve Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.2.3
/// - **UID Type:** SOP Class
pub static StudyRootQueryRetrieveInformationModelGET: UID = UID::new(
    "StudyRootQueryRetrieveInformationModelGET",
    "1.2.840.10008.5.1.4.1.2.2.3",
    "Study Root Query/Retrieve Information Model - GET",
);

/// Patient/Study Only Query/Retrieve Information Model - FIND (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.3.1
/// - **UID Type:** SOP Class
pub static PatientStudyOnlyQueryRetrieveInformationModelFIND: UID = UID::new(
    "PatientStudyOnlyQueryRetrieveInformationModelFIND",
    "1.2.840.10008.5.1.4.1.2.3.1",
    "Patient/Study Only Query/Retrieve Information Model - FIND (Retired)",
);

/// Patient/Study Only Query/Retrieve Information Model - MOVE (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.3.2
/// - **UID Type:** SOP Class
pub static PatientStudyOnlyQueryRetrieveInformationModelMOVE: UID = UID::new(
    "PatientStudyOnlyQueryRetrieveInformationModelMOVE",
    "1.2.840.10008.5.1.4.1.2.3.2",
    "Patient/Study Only Query/Retrieve Information Model - MOVE (Retired)",
);

/// Patient/Study Only Query/Retrieve Information Model - GET (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.3.3
/// - **UID Type:** SOP Class
pub static PatientStudyOnlyQueryRetrieveInformationModelGET: UID = UID::new(
    "PatientStudyOnlyQueryRetrieveInformationModelGET",
    "1.2.840.10008.5.1.4.1.2.3.3",
    "Patient/Study Only Query/Retrieve Information Model - GET (Retired)",
);

/// Composite Instance Root Retrieve - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.4.2
/// - **UID Type:** SOP Class
pub static CompositeInstanceRootRetrieveMOVE: UID = UID::new(
    "CompositeInstanceRootRetrieveMOVE",
    "1.2.840.10008.5.1.4.1.2.4.2",
    "Composite Instance Root Retrieve - MOVE",
);

/// Composite Instance Root Retrieve - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.4.3
/// - **UID Type:** SOP Class
pub static CompositeInstanceRootRetrieveGET: UID = UID::new(
    "CompositeInstanceRootRetrieveGET",
    "1.2.840.10008.5.1.4.1.2.4.3",
    "Composite Instance Root Retrieve - GET",
);

/// Composite Instance Retrieve Without Bulk Data - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.5.3
/// - **UID Type:** SOP Class
pub static CompositeInstanceRetrieveWithoutBulkDataGET: UID = UID::new(
    "CompositeInstanceRetrieveWithoutBulkDataGET",
    "1.2.840.10008.5.1.4.1.2.5.3",
    "Composite Instance Retrieve Without Bulk Data - GET",
);

/// Defined Procedure Protocol Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.20.1
/// - **UID Type:** SOP Class
pub static DefinedProcedureProtocolInformationModelFIND: UID = UID::new(
    "DefinedProcedureProtocolInformationModelFIND",
    "1.2.840.10008.5.1.4.20.1",
    "Defined Procedure Protocol Information Model - FIND",
);

/// Defined Procedure Protocol Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.20.2
/// - **UID Type:** SOP Class
pub static DefinedProcedureProtocolInformationModelMOVE: UID = UID::new(
    "DefinedProcedureProtocolInformationModelMOVE",
    "1.2.840.10008.5.1.4.20.2",
    "Defined Procedure Protocol Information Model - MOVE",
);

/// Defined Procedure Protocol Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.20.3
/// - **UID Type:** SOP Class
pub static DefinedProcedureProtocolInformationModelGET: UID = UID::new(
    "DefinedProcedureProtocolInformationModelGET",
    "1.2.840.10008.5.1.4.20.3",
    "Defined Procedure Protocol Information Model - GET",
);

/// Modality Worklist Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.31
/// - **UID Type:** SOP Class
pub static ModalityWorklistInformationModelFIND: UID = UID::new(
    "ModalityWorklistInformationModelFIND",
    "1.2.840.10008.5.1.4.31",
    "Modality Worklist Information Model - FIND",
);

/// General Purpose Worklist Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32
/// - **UID Type:** Meta SOP Class
pub static GeneralPurposeWorklistManagementMetaSOPClass: UID = UID::new(
    "GeneralPurposeWorklistManagementMetaSOPClass",
    "1.2.840.10008.5.1.4.32",
    "General Purpose Worklist Management Meta SOP Class (Retired)",
);

/// General Purpose Worklist Information Model - FIND (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32.1
/// - **UID Type:** SOP Class
pub static GeneralPurposeWorklistInformationModelFIND: UID = UID::new(
    "GeneralPurposeWorklistInformationModelFIND",
    "1.2.840.10008.5.1.4.32.1",
    "General Purpose Worklist Information Model - FIND (Retired)",
);

/// General Purpose Scheduled Procedure Step SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32.2
/// - **UID Type:** SOP Class
pub static GeneralPurposeScheduledProcedureStepSOPClass: UID = UID::new(
    "GeneralPurposeScheduledProcedureStepSOPClass",
    "1.2.840.10008.5.1.4.32.2",
    "General Purpose Scheduled Procedure Step SOP Class (Retired)",
);

/// General Purpose Performed Procedure Step SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32.3
/// - **UID Type:** SOP Class
pub static GeneralPurposePerformedProcedureStepSOPClass: UID = UID::new(
    "GeneralPurposePerformedProcedureStepSOPClass",
    "1.2.840.10008.5.1.4.32.3",
    "General Purpose Performed Procedure Step SOP Class (Retired)",
);

/// Instance Availability Notification SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.33
/// - **UID Type:** SOP Class
pub static InstanceAvailabilityNotificationSOPClass: UID = UID::new(
    "InstanceAvailabilityNotificationSOPClass",
    "1.2.840.10008.5.1.4.33",
    "Instance Availability Notification SOP Class",
);

/// RT Beams Delivery Instruction Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.1
/// - **UID Type:** SOP Class
pub static RTBeamsDeliveryInstructionStorageTrial: UID = UID::new(
    "RTBeamsDeliveryInstructionStorageTrial",
    "1.2.840.10008.5.1.4.34.1",
    "RT Beams Delivery Instruction Storage - Trial (Retired)",
);

/// RT Conventional Machine Verification - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.2
/// - **UID Type:** SOP Class
pub static RTConventionalMachineVerificationTrial: UID = UID::new(
    "RTConventionalMachineVerificationTrial",
    "1.2.840.10008.5.1.4.34.2",
    "RT Conventional Machine Verification - Trial (Retired)",
);

/// RT Ion Machine Verification - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.3
/// - **UID Type:** SOP Class
pub static RTIonMachineVerificationTrial: UID = UID::new(
    "RTIonMachineVerificationTrial",
    "1.2.840.10008.5.1.4.34.3",
    "RT Ion Machine Verification - Trial (Retired)",
);

/// Unified Worklist and Procedure Step Service Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4
/// - **UID Type:** Service Class
pub static UnifiedWorklistandProcedureStepServiceClassTrial: UID = UID::new(
    "UnifiedWorklistandProcedureStepServiceClassTrial",
    "1.2.840.10008.5.1.4.34.4",
    "Unified Worklist and Procedure Step Service Class - Trial (Retired)",
);

/// Unified Procedure Step - Push SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.1
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPushSOPClassTrial: UID = UID::new(
    "UnifiedProcedureStepPushSOPClassTrial",
    "1.2.840.10008.5.1.4.34.4.1",
    "Unified Procedure Step - Push SOP Class - Trial (Retired)",
);

/// Unified Procedure Step - Watch SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.2
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepWatchSOPClassTrial: UID = UID::new(
    "UnifiedProcedureStepWatchSOPClassTrial",
    "1.2.840.10008.5.1.4.34.4.2",
    "Unified Procedure Step - Watch SOP Class - Trial (Retired)",
);

/// Unified Procedure Step - Pull SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.3
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPullSOPClassTrial: UID = UID::new(
    "UnifiedProcedureStepPullSOPClassTrial",
    "1.2.840.10008.5.1.4.34.4.3",
    "Unified Procedure Step - Pull SOP Class - Trial (Retired)",
);

/// Unified Procedure Step - Event SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.4
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepEventSOPClassTrial: UID = UID::new(
    "UnifiedProcedureStepEventSOPClassTrial",
    "1.2.840.10008.5.1.4.34.4.4",
    "Unified Procedure Step - Event SOP Class - Trial (Retired)",
);

/// UPS Global Subscription SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.4.34.5
/// - **UID Type:** Well-known SOP Instance
pub static UPSGlobalSubscriptionSOPInstance: UID = UID::new(
    "UPSGlobalSubscriptionSOPInstance",
    "1.2.840.10008.5.1.4.34.5",
    "UPS Global Subscription SOP Instance",
);

/// UPS Filtered Global Subscription SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.4.34.5.1
/// - **UID Type:** Well-known SOP Instance
pub static UPSFilteredGlobalSubscriptionSOPInstance: UID = UID::new(
    "UPSFilteredGlobalSubscriptionSOPInstance",
    "1.2.840.10008.5.1.4.34.5.1",
    "UPS Filtered Global Subscription SOP Instance",
);

/// Unified Worklist and Procedure Step Service Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6
/// - **UID Type:** Service Class
pub static UnifiedWorklistandProcedureStepServiceClass: UID = UID::new(
    "UnifiedWorklistandProcedureStepServiceClass",
    "1.2.840.10008.5.1.4.34.6",
    "Unified Worklist and Procedure Step Service Class",
);

/// Unified Procedure Step - Push SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.1
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPushSOPClass: UID = UID::new(
    "UnifiedProcedureStepPushSOPClass",
    "1.2.840.10008.5.1.4.34.6.1",
    "Unified Procedure Step - Push SOP Class",
);

/// Unified Procedure Step - Watch SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.2
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepWatchSOPClass: UID = UID::new(
    "UnifiedProcedureStepWatchSOPClass",
    "1.2.840.10008.5.1.4.34.6.2",
    "Unified Procedure Step - Watch SOP Class",
);

/// Unified Procedure Step - Pull SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.3
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPullSOPClass: UID = UID::new(
    "UnifiedProcedureStepPullSOPClass",
    "1.2.840.10008.5.1.4.34.6.3",
    "Unified Procedure Step - Pull SOP Class",
);

/// Unified Procedure Step - Event SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.4
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepEventSOPClass: UID = UID::new(
    "UnifiedProcedureStepEventSOPClass",
    "1.2.840.10008.5.1.4.34.6.4",
    "Unified Procedure Step - Event SOP Class",
);

/// Unified Procedure Step - Query SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.5
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepQuerySOPClass: UID = UID::new(
    "UnifiedProcedureStepQuerySOPClass",
    "1.2.840.10008.5.1.4.34.6.5",
    "Unified Procedure Step - Query SOP Class",
);

/// RT Beams Delivery Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.34.7
/// - **UID Type:** SOP Class
pub static RTBeamsDeliveryInstructionStorage: UID = UID::new(
    "RTBeamsDeliveryInstructionStorage",
    "1.2.840.10008.5.1.4.34.7",
    "RT Beams Delivery Instruction Storage",
);

/// RT Conventional Machine Verification
///
/// - **UID:** 1.2.840.10008.5.1.4.34.8
/// - **UID Type:** SOP Class
pub static RTConventionalMachineVerification: UID = UID::new(
    "RTConventionalMachineVerification",
    "1.2.840.10008.5.1.4.34.8",
    "RT Conventional Machine Verification",
);

/// RT Ion Machine Verification
///
/// - **UID:** 1.2.840.10008.5.1.4.34.9
/// - **UID Type:** SOP Class
pub static RTIonMachineVerification: UID = UID::new(
    "RTIonMachineVerification",
    "1.2.840.10008.5.1.4.34.9",
    "RT Ion Machine Verification",
);

/// RT Brachy Application Setup Delivery Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.34.10
/// - **UID Type:** SOP Class
pub static RTBrachyApplicationSetupDeliveryInstructionStorage: UID = UID::new(
    "RTBrachyApplicationSetupDeliveryInstructionStorage",
    "1.2.840.10008.5.1.4.34.10",
    "RT Brachy Application Setup Delivery Instruction Storage",
);

/// General Relevant Patient Information Query
///
/// - **UID:** 1.2.840.10008.5.1.4.37.1
/// - **UID Type:** SOP Class
pub static GeneralRelevantPatientInformationQuery: UID = UID::new(
    "GeneralRelevantPatientInformationQuery",
    "1.2.840.10008.5.1.4.37.1",
    "General Relevant Patient Information Query",
);

/// Breast Imaging Relevant Patient Information Query
///
/// - **UID:** 1.2.840.10008.5.1.4.37.2
/// - **UID Type:** SOP Class
pub static BreastImagingRelevantPatientInformationQuery: UID = UID::new(
    "BreastImagingRelevantPatientInformationQuery",
    "1.2.840.10008.5.1.4.37.2",
    "Breast Imaging Relevant Patient Information Query",
);

/// Cardiac Relevant Patient Information Query
///
/// - **UID:** 1.2.840.10008.5.1.4.37.3
/// - **UID Type:** SOP Class
pub static CardiacRelevantPatientInformationQuery: UID = UID::new(
    "CardiacRelevantPatientInformationQuery",
    "1.2.840.10008.5.1.4.37.3",
    "Cardiac Relevant Patient Information Query",
);

/// Hanging Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.38.1
/// - **UID Type:** SOP Class
pub static HangingProtocolStorage: UID = UID::new(
    "HangingProtocolStorage",
    "1.2.840.10008.5.1.4.38.1",
    "Hanging Protocol Storage",
);

/// Hanging Protocol Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.38.2
/// - **UID Type:** SOP Class
pub static HangingProtocolInformationModelFIND: UID = UID::new(
    "HangingProtocolInformationModelFIND",
    "1.2.840.10008.5.1.4.38.2",
    "Hanging Protocol Information Model - FIND",
);

/// Hanging Protocol Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.38.3
/// - **UID Type:** SOP Class
pub static HangingProtocolInformationModelMOVE: UID = UID::new(
    "HangingProtocolInformationModelMOVE",
    "1.2.840.10008.5.1.4.38.3",
    "Hanging Protocol Information Model - MOVE",
);

/// Hanging Protocol Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.38.4
/// - **UID Type:** SOP Class
pub static HangingProtocolInformationModelGET: UID = UID::new(
    "HangingProtocolInformationModelGET",
    "1.2.840.10008.5.1.4.38.4",
    "Hanging Protocol Information Model - GET",
);

/// Color Palette Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.39.1
/// - **UID Type:** SOP Class
pub static ColorPaletteStorage: UID = UID::new(
    "ColorPaletteStorage",
    "1.2.840.10008.5.1.4.39.1",
    "Color Palette Storage",
);

/// Color Palette Query/Retrieve Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.39.2
/// - **UID Type:** SOP Class
pub static ColorPaletteQueryRetrieveInformationModelFIND: UID = UID::new(
    "ColorPaletteQueryRetrieveInformationModelFIND",
    "1.2.840.10008.5.1.4.39.2",
    "Color Palette Query/Retrieve Information Model - FIND",
);

/// Color Palette Query/Retrieve Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.39.3
/// - **UID Type:** SOP Class
pub static ColorPaletteQueryRetrieveInformationModelMOVE: UID = UID::new(
    "ColorPaletteQueryRetrieveInformationModelMOVE",
    "1.2.840.10008.5.1.4.39.3",
    "Color Palette Query/Retrieve Information Model - MOVE",
);

/// Color Palette Query/Retrieve Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.39.4
/// - **UID Type:** SOP Class
pub static ColorPaletteQueryRetrieveInformationModelGET: UID = UID::new(
    "ColorPaletteQueryRetrieveInformationModelGET",
    "1.2.840.10008.5.1.4.39.4",
    "Color Palette Query/Retrieve Information Model - GET",
);

/// Product Characteristics Query SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.41
/// - **UID Type:** SOP Class
pub static ProductCharacteristicsQuerySOPClass: UID = UID::new(
    "ProductCharacteristicsQuerySOPClass",
    "1.2.840.10008.5.1.4.41",
    "Product Characteristics Query SOP Class",
);

/// Substance Approval Query SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.42
/// - **UID Type:** SOP Class
pub static SubstanceApprovalQuerySOPClass: UID = UID::new(
    "SubstanceApprovalQuerySOPClass",
    "1.2.840.10008.5.1.4.42",
    "Substance Approval Query SOP Class",
);

/// Generic Implant Template Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.43.1
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateStorage: UID = UID::new(
    "GenericImplantTemplateStorage",
    "1.2.840.10008.5.1.4.43.1",
    "Generic Implant Template Storage",
);

/// Generic Implant Template Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.43.2
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateInformationModelFIND: UID = UID::new(
    "GenericImplantTemplateInformationModelFIND",
    "1.2.840.10008.5.1.4.43.2",
    "Generic Implant Template Information Model - FIND",
);

/// Generic Implant Template Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.43.3
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateInformationModelMOVE: UID = UID::new(
    "GenericImplantTemplateInformationModelMOVE",
    "1.2.840.10008.5.1.4.43.3",
    "Generic Implant Template Information Model - MOVE",
);

/// Generic Implant Template Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.43.4
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateInformationModelGET: UID = UID::new(
    "GenericImplantTemplateInformationModelGET",
    "1.2.840.10008.5.1.4.43.4",
    "Generic Implant Template Information Model - GET",
);

/// Implant Assembly Template Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.44.1
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateStorage: UID = UID::new(
    "ImplantAssemblyTemplateStorage",
    "1.2.840.10008.5.1.4.44.1",
    "Implant Assembly Template Storage",
);

/// Implant Assembly Template Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.44.2
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateInformationModelFIND: UID = UID::new(
    "ImplantAssemblyTemplateInformationModelFIND",
    "1.2.840.10008.5.1.4.44.2",
    "Implant Assembly Template Information Model - FIND",
);

/// Implant Assembly Template Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.44.3
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateInformationModelMOVE: UID = UID::new(
    "ImplantAssemblyTemplateInformationModelMOVE",
    "1.2.840.10008.5.1.4.44.3",
    "Implant Assembly Template Information Model - MOVE",
);

/// Implant Assembly Template Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.44.4
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateInformationModelGET: UID = UID::new(
    "ImplantAssemblyTemplateInformationModelGET",
    "1.2.840.10008.5.1.4.44.4",
    "Implant Assembly Template Information Model - GET",
);

/// Implant Template Group Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.45.1
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupStorage: UID = UID::new(
    "ImplantTemplateGroupStorage",
    "1.2.840.10008.5.1.4.45.1",
    "Implant Template Group Storage",
);

/// Implant Template Group Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.45.2
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupInformationModelFIND: UID = UID::new(
    "ImplantTemplateGroupInformationModelFIND",
    "1.2.840.10008.5.1.4.45.2",
    "Implant Template Group Information Model - FIND",
);

/// Implant Template Group Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.45.3
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupInformationModelMOVE: UID = UID::new(
    "ImplantTemplateGroupInformationModelMOVE",
    "1.2.840.10008.5.1.4.45.3",
    "Implant Template Group Information Model - MOVE",
);

/// Implant Template Group Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.45.4
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupInformationModelGET: UID = UID::new(
    "ImplantTemplateGroupInformationModelGET",
    "1.2.840.10008.5.1.4.45.4",
    "Implant Template Group Information Model - GET",
);

/// Native DICOM Model
///
/// - **UID:** 1.2.840.10008.7.1.1
/// - **UID Type:** Application Hosting Model
pub static NativeDICOMModel: UID = UID::new(
    "NativeDICOMModel",
    "1.2.840.10008.7.1.1",
    "Native DICOM Model",
);

/// Abstract Multi-Dimensional Image Model
///
/// - **UID:** 1.2.840.10008.7.1.2
/// - **UID Type:** Application Hosting Model
pub static AbstractMultiDimensionalImageModel: UID = UID::new(
    "AbstractMultiDimensionalImageModel",
    "1.2.840.10008.7.1.2",
    "Abstract Multi-Dimensional Image Model",
);

/// DICOM Content Mapping Resource
///
/// - **UID:** 1.2.840.10008.8.1.1
/// - **UID Type:** Mapping Resource
pub static DICOMContentMappingResource: UID = UID::new(
    "DICOMContentMappingResource",
    "1.2.840.10008.8.1.1",
    "DICOM Content Mapping Resource",
);

/// Video Endoscopic Image Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.1
/// - **UID Type:** SOP Class
pub static VideoEndoscopicImageRealTimeCommunication: UID = UID::new(
    "VideoEndoscopicImageRealTimeCommunication",
    "1.2.840.10008.10.1",
    "Video Endoscopic Image Real-Time Communication",
);

/// Video Photographic Image Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.2
/// - **UID Type:** SOP Class
pub static VideoPhotographicImageRealTimeCommunication: UID = UID::new(
    "VideoPhotographicImageRealTimeCommunication",
    "1.2.840.10008.10.2",
    "Video Photographic Image Real-Time Communication",
);

/// Audio Waveform Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.3
/// - **UID Type:** SOP Class
pub static AudioWaveformRealTimeCommunication: UID = UID::new(
    "AudioWaveformRealTimeCommunication",
    "1.2.840.10008.10.3",
    "Audio Waveform Real-Time Communication",
);

/// Rendition Selection Document Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.4
/// - **UID Type:** SOP Class
pub static RenditionSelectionDocumentRealTimeCommunication: UID = UID::new(
    "RenditionSelectionDocumentRealTimeCommunication",
    "1.2.840.10008.10.4",
    "Rendition Selection Document Real-Time Communication",
);

/// dicomDeviceName
///
/// - **UID:** 1.2.840.10008.15.0.3.1
/// - **UID Type:** LDAP OID
pub static DicomDeviceName: UID = UID::new(
    "DicomDeviceName",
    "1.2.840.10008.15.0.3.1",
    "dicomDeviceName",
);

/// dicomDescription
///
/// - **UID:** 1.2.840.10008.15.0.3.2
/// - **UID Type:** LDAP OID
pub static DicomDescription: UID = UID::new(
    "DicomDescription",
    "1.2.840.10008.15.0.3.2",
    "dicomDescription",
);

/// dicomManufacturer
///
/// - **UID:** 1.2.840.10008.15.0.3.3
/// - **UID Type:** LDAP OID
pub static DicomManufacturer: UID = UID::new(
    "DicomManufacturer",
    "1.2.840.10008.15.0.3.3",
    "dicomManufacturer",
);

/// dicomManufacturerModelName
///
/// - **UID:** 1.2.840.10008.15.0.3.4
/// - **UID Type:** LDAP OID
pub static DicomManufacturerModelName: UID = UID::new(
    "DicomManufacturerModelName",
    "1.2.840.10008.15.0.3.4",
    "dicomManufacturerModelName",
);

/// dicomSoftwareVersion
///
/// - **UID:** 1.2.840.10008.15.0.3.5
/// - **UID Type:** LDAP OID
pub static DicomSoftwareVersion: UID = UID::new(
    "DicomSoftwareVersion",
    "1.2.840.10008.15.0.3.5",
    "dicomSoftwareVersion",
);

/// dicomVendorData
///
/// - **UID:** 1.2.840.10008.15.0.3.6
/// - **UID Type:** LDAP OID
pub static DicomVendorData: UID = UID::new(
    "DicomVendorData",
    "1.2.840.10008.15.0.3.6",
    "dicomVendorData",
);

/// dicomAETitle
///
/// - **UID:** 1.2.840.10008.15.0.3.7
/// - **UID Type:** LDAP OID
pub static DicomAETitle: UID = UID::new(
    "DicomAETitle",
    "1.2.840.10008.15.0.3.7",
    "dicomAETitle",
);

/// dicomNetworkConnectionReference
///
/// - **UID:** 1.2.840.10008.15.0.3.8
/// - **UID Type:** LDAP OID
pub static DicomNetworkConnectionReference: UID = UID::new(
    "DicomNetworkConnectionReference",
    "1.2.840.10008.15.0.3.8",
    "dicomNetworkConnectionReference",
);

/// dicomApplicationCluster
///
/// - **UID:** 1.2.840.10008.15.0.3.9
/// - **UID Type:** LDAP OID
pub static DicomApplicationCluster: UID = UID::new(
    "DicomApplicationCluster",
    "1.2.840.10008.15.0.3.9",
    "dicomApplicationCluster",
);

/// dicomAssociationInitiator
///
/// - **UID:** 1.2.840.10008.15.0.3.10
/// - **UID Type:** LDAP OID
pub static DicomAssociationInitiator: UID = UID::new(
    "DicomAssociationInitiator",
    "1.2.840.10008.15.0.3.10",
    "dicomAssociationInitiator",
);

/// dicomAssociationAcceptor
///
/// - **UID:** 1.2.840.10008.15.0.3.11
/// - **UID Type:** LDAP OID
pub static DicomAssociationAcceptor: UID = UID::new(
    "DicomAssociationAcceptor",
    "1.2.840.10008.15.0.3.11",
    "dicomAssociationAcceptor",
);

/// dicomHostname
///
/// - **UID:** 1.2.840.10008.15.0.3.12
/// - **UID Type:** LDAP OID
pub static DicomHostname: UID = UID::new(
    "DicomHostname",
    "1.2.840.10008.15.0.3.12",
    "dicomHostname",
);

/// dicomPort
///
/// - **UID:** 1.2.840.10008.15.0.3.13
/// - **UID Type:** LDAP OID
pub static DicomPort: UID = UID::new(
    "DicomPort",
    "1.2.840.10008.15.0.3.13",
    "dicomPort",
);

/// dicomSOPClass
///
/// - **UID:** 1.2.840.10008.15.0.3.14
/// - **UID Type:** LDAP OID
pub static DicomSOPClass: UID = UID::new(
    "DicomSOPClass",
    "1.2.840.10008.15.0.3.14",
    "dicomSOPClass",
);

/// dicomTransferRole
///
/// - **UID:** 1.2.840.10008.15.0.3.15
/// - **UID Type:** LDAP OID
pub static DicomTransferRole: UID = UID::new(
    "DicomTransferRole",
    "1.2.840.10008.15.0.3.15",
    "dicomTransferRole",
);

/// dicomTransferSyntax
///
/// - **UID:** 1.2.840.10008.15.0.3.16
/// - **UID Type:** LDAP OID
pub static DicomTransferSyntax: UID = UID::new(
    "DicomTransferSyntax",
    "1.2.840.10008.15.0.3.16",
    "dicomTransferSyntax",
);

/// dicomPrimaryDeviceType
///
/// - **UID:** 1.2.840.10008.15.0.3.17
/// - **UID Type:** LDAP OID
pub static DicomPrimaryDeviceType: UID = UID::new(
    "DicomPrimaryDeviceType",
    "1.2.840.10008.15.0.3.17",
    "dicomPrimaryDeviceType",
);

/// dicomRelatedDeviceReference
///
/// - **UID:** 1.2.840.10008.15.0.3.18
/// - **UID Type:** LDAP OID
pub static DicomRelatedDeviceReference: UID = UID::new(
    "DicomRelatedDeviceReference",
    "1.2.840.10008.15.0.3.18",
    "dicomRelatedDeviceReference",
);

/// dicomPreferredCalledAETitle
///
/// - **UID:** 1.2.840.10008.15.0.3.19
/// - **UID Type:** LDAP OID
pub static DicomPreferredCalledAETitle: UID = UID::new(
    "DicomPreferredCalledAETitle",
    "1.2.840.10008.15.0.3.19",
    "dicomPreferredCalledAETitle",
);

/// dicomTLSCyphersuite
///
/// - **UID:** 1.2.840.10008.15.0.3.20
/// - **UID Type:** LDAP OID
pub static DicomTLSCyphersuite: UID = UID::new(
    "DicomTLSCyphersuite",
    "1.2.840.10008.15.0.3.20",
    "dicomTLSCyphersuite",
);

/// dicomAuthorizedNodeCertificateReference
///
/// - **UID:** 1.2.840.10008.15.0.3.21
/// - **UID Type:** LDAP OID
pub static DicomAuthorizedNodeCertificateReference: UID = UID::new(
    "DicomAuthorizedNodeCertificateReference",
    "1.2.840.10008.15.0.3.21",
    "dicomAuthorizedNodeCertificateReference",
);

/// dicomThisNodeCertificateReference
///
/// - **UID:** 1.2.840.10008.15.0.3.22
/// - **UID Type:** LDAP OID
pub static DicomThisNodeCertificateReference: UID = UID::new(
    "DicomThisNodeCertificateReference",
    "1.2.840.10008.15.0.3.22",
    "dicomThisNodeCertificateReference",
);

/// dicomInstalled
///
/// - **UID:** 1.2.840.10008.15.0.3.23
/// - **UID Type:** LDAP OID
pub static DicomInstalled: UID = UID::new(
    "DicomInstalled",
    "1.2.840.10008.15.0.3.23",
    "dicomInstalled",
);

/// dicomStationName
///
/// - **UID:** 1.2.840.10008.15.0.3.24
/// - **UID Type:** LDAP OID
pub static DicomStationName: UID = UID::new(
    "DicomStationName",
    "1.2.840.10008.15.0.3.24",
    "dicomStationName",
);

/// dicomDeviceSerialNumber
///
/// - **UID:** 1.2.840.10008.15.0.3.25
/// - **UID Type:** LDAP OID
pub static DicomDeviceSerialNumber: UID = UID::new(
    "DicomDeviceSerialNumber",
    "1.2.840.10008.15.0.3.25",
    "dicomDeviceSerialNumber",
);

/// dicomInstitutionName
///
/// - **UID:** 1.2.840.10008.15.0.3.26
/// - **UID Type:** LDAP OID
pub static DicomInstitutionName: UID = UID::new(
    "DicomInstitutionName",
    "1.2.840.10008.15.0.3.26",
    "dicomInstitutionName",
);

/// dicomInstitutionAddress
///
/// - **UID:** 1.2.840.10008.15.0.3.27
/// - **UID Type:** LDAP OID
pub static DicomInstitutionAddress: UID = UID::new(
    "DicomInstitutionAddress",
    "1.2.840.10008.15.0.3.27",
    "dicomInstitutionAddress",
);

/// dicomInstitutionDepartmentName
///
/// - **UID:** 1.2.840.10008.15.0.3.28
/// - **UID Type:** LDAP OID
pub static DicomInstitutionDepartmentName: UID = UID::new(
    "DicomInstitutionDepartmentName",
    "1.2.840.10008.15.0.3.28",
    "dicomInstitutionDepartmentName",
);

/// dicomIssuerOfPatientID
///
/// - **UID:** 1.2.840.10008.15.0.3.29
/// - **UID Type:** LDAP OID
pub static DicomIssuerOfPatientID: UID = UID::new(
    "DicomIssuerOfPatientID",
    "1.2.840.10008.15.0.3.29",
    "dicomIssuerOfPatientID",
);

/// dicomPreferredCallingAETitle
///
/// - **UID:** 1.2.840.10008.15.0.3.30
/// - **UID Type:** LDAP OID
pub static DicomPreferredCallingAETitle: UID = UID::new(
    "DicomPreferredCallingAETitle",
    "1.2.840.10008.15.0.3.30",
    "dicomPreferredCallingAETitle",
);

/// dicomSupportedCharacterSet
///
/// - **UID:** 1.2.840.10008.15.0.3.31
/// - **UID Type:** LDAP OID
pub static DicomSupportedCharacterSet: UID = UID::new(
    "DicomSupportedCharacterSet",
    "1.2.840.10008.15.0.3.31",
    "dicomSupportedCharacterSet",
);

/// dicomConfigurationRoot
///
/// - **UID:** 1.2.840.10008.15.0.4.1
/// - **UID Type:** LDAP OID
pub static DicomConfigurationRoot: UID = UID::new(
    "DicomConfigurationRoot",
    "1.2.840.10008.15.0.4.1",
    "dicomConfigurationRoot",
);

/// dicomDevicesRoot
///
/// - **UID:** 1.2.840.10008.15.0.4.2
/// - **UID Type:** LDAP OID
pub static DicomDevicesRoot: UID = UID::new(
    "DicomDevicesRoot",
    "1.2.840.10008.15.0.4.2",
    "dicomDevicesRoot",
);

/// dicomUniqueAETitlesRegistryRoot
///
/// - **UID:** 1.2.840.10008.15.0.4.3
/// - **UID Type:** LDAP OID
pub static DicomUniqueAETitlesRegistryRoot: UID = UID::new(
    "DicomUniqueAETitlesRegistryRoot",
    "1.2.840.10008.15.0.4.3",
    "dicomUniqueAETitlesRegistryRoot",
);

/// dicomDevice
///
/// - **UID:** 1.2.840.10008.15.0.4.4
/// - **UID Type:** LDAP OID
pub static DicomDevice: UID = UID::new(
    "DicomDevice",
    "1.2.840.10008.15.0.4.4",
    "dicomDevice",
);

/// dicomNetworkAE
///
/// - **UID:** 1.2.840.10008.15.0.4.5
/// - **UID Type:** LDAP OID
pub static DicomNetworkAE: UID = UID::new(
    "DicomNetworkAE",
    "1.2.840.10008.15.0.4.5",
    "dicomNetworkAE",
);

/// dicomNetworkConnection
///
/// - **UID:** 1.2.840.10008.15.0.4.6
/// - **UID Type:** LDAP OID
pub static DicomNetworkConnection: UID = UID::new(
    "DicomNetworkConnection",
    "1.2.840.10008.15.0.4.6",
    "dicomNetworkConnection",
);

/// dicomUniqueAETitle
///
/// - **UID:** 1.2.840.10008.15.0.4.7
/// - **UID Type:** LDAP OID
pub static DicomUniqueAETitle: UID = UID::new(
    "DicomUniqueAETitle",
    "1.2.840.10008.15.0.4.7",
    "dicomUniqueAETitle",
);

/// dicomTransferCapability
///
/// - **UID:** 1.2.840.10008.15.0.4.8
/// - **UID Type:** LDAP OID
pub static DicomTransferCapability: UID = UID::new(
    "DicomTransferCapability",
    "1.2.840.10008.15.0.4.8",
    "dicomTransferCapability",
);

/// Universal Coordinated Time
///
/// - **UID:** 1.2.840.10008.15.1.1
/// - **UID Type:** Synchronization Frame of Reference
pub static UniversalCoordinatedTime: UID = UID::new(
    "UniversalCoordinatedTime",
    "1.2.840.10008.15.1.1",
    "Universal Coordinated Time",
);

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
//! This contains definitions of Transfer Syntaxes.

#![allow(non_upper_case_globals, clippy::doc_markdown)]

use crate::core::defn::ts::TransferSyntax;
use crate::dict::uids;

/// Implicit VR Little Endian: Default Transfer Syntax for DICOM
///
/// - **UID:** 1.2.840.10008.1.2
pub static ImplicitVRLittleEndian: TransferSyntax = TransferSyntax::new(
    &uids::ImplicitVRLittleEndian,
    false,
    false,
    false,
    false,
);

/// Explicit VR Little Endian
///
/// - **UID:** 1.2.840.10008.1.2.1
pub static ExplicitVRLittleEndian: TransferSyntax = TransferSyntax::new(
    &uids::ExplicitVRLittleEndian,
    true,
    false,
    false,
    false,
);

/// Encapsulated Uncompressed Explicit VR Little Endian
///
/// - **UID:** 1.2.840.10008.1.2.1.98
pub static EncapsulatedUncompressedExplicitVRLittleEndian: TransferSyntax = TransferSyntax::new(
    &uids::EncapsulatedUncompressedExplicitVRLittleEndian,
    true,
    false,
    false,
    true,
);

/// Deflated Explicit VR Little Endian
///
/// - **UID:** 1.2.840.10008.1.2.1.99
pub static DeflatedExplicitVRLittleEndian: TransferSyntax = TransferSyntax::new(
    &uids::DeflatedExplicitVRLittleEndian,
    true,
    false,
    true,
    false,
);

/// Explicit VR Big Endian (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.2
pub static ExplicitVRBigEndian: TransferSyntax = TransferSyntax::new(
    &uids::ExplicitVRBigEndian,
    true,
    true,
    false,
    false,
);

/// JPEG Baseline (Process 1): Default Transfer Syntax for Lossy JPEG 8 Bit Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.50
pub static JPEGBaselineProcess1: TransferSyntax = TransferSyntax::new(
    &uids::JPEGBaselineProcess1,
    true,
    false,
    false,
    true,
);

/// JPEG Extended (Process 2&4): Default Transfer Syntax for Lossy JPEG 12 Bit Image Compression (Process 4 only)
///
/// - **UID:** 1.2.840.10008.1.2.4.51
pub static JPEGExtendedProcess2_and_4: TransferSyntax = TransferSyntax::new(
    &uids::JPEGExtendedProcess2_and_4,
    true,
    false,
    false,
    true,
);

/// JPEG Extended (Process 3&5) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.52
pub static JPEGExtendedProcess3_and_5: TransferSyntax = TransferSyntax::new(
    &uids::JPEGExtendedProcess3_and_5,
    true,
    false,
    false,
    true,
);

/// JPEG Spectral Selection, Non-Hierarchical (Process 6&8) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.53
pub static JPEGSpectralSelectionNonHierarchicalProcess6_and_8: TransferSyntax = TransferSyntax::new(
    &uids::JPEGSpectralSelectionNonHierarchicalProcess6_and_8,
    true,
    false,
    false,
    true,
);

/// JPEG Spectral Selection, Non-Hierarchical (Process 7&9) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.54
pub static JPEGSpectralSelectionNonHierarchicalProcess7_and_9: TransferSyntax = TransferSyntax::new(
    &uids::JPEGSpectralSelectionNonHierarchicalProcess7_and_9,
    true,
    false,
    false,
    true,
);

/// JPEG Full Progression, Non-Hierarchical (Process 10&12) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.55
pub static JPEGFullProgressionNonHierarchicalProcess10_and_12: TransferSyntax = TransferSyntax::new(
    &uids::JPEGFullProgressionNonHierarchicalProcess10_and_12,
    true,
    false,
    false,
    true,
);

/// JPEG Full Progression, Non-Hierarchical (Process 11&13) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.56
pub static JPEGFullProgressionNonHierarchicalProcess11_and_13: TransferSyntax = TransferSyntax::new(
    &uids::JPEGFullProgressionNonHierarchicalProcess11_and_13,
    true,
    false,
    false,
    true,
);

/// JPEG Lossless, Non-Hierarchical (Process 14)
///
/// - **UID:** 1.2.840.10008.1.2.4.57
pub static JPEGLosslessNonHierarchicalProcess14: TransferSyntax = TransferSyntax::new(
    &uids::JPEGLosslessNonHierarchicalProcess14,
    true,
    false,
    false,
    true,
);

/// JPEG Lossless, Non-Hierarchical (Process 15) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.58
pub static JPEGLosslessNonHierarchicalProcess15: TransferSyntax = TransferSyntax::new(
    &uids::JPEGLosslessNonHierarchicalProcess15,
    true,
    false,
    false,
    true,
);

/// JPEG Extended, Hierarchical (Process 16&18) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.59
pub static JPEGExtendedHierarchicalProcess16_and_18: TransferSyntax = TransferSyntax::new(
    &uids::JPEGExtendedHierarchicalProcess16_and_18,
    true,
    false,
    false,
    true,
);

/// JPEG Extended, Hierarchical (Process 17&19) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.60
pub static JPEGExtendedHierarchicalProcess17_and_19: TransferSyntax = TransferSyntax::new(
    &uids::JPEGExtendedHierarchicalProcess17_and_19,
    true,
    false,
    false,
    true,
);

/// JPEG Spectral Selection, Hierarchical (Process 20&22) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.61
pub static JPEGSpectralSelectionHierarchicalProcess20_and_22: TransferSyntax = TransferSyntax::new(
    &uids::JPEGSpectralSelectionHierarchicalProcess20_and_22,
    true,
    false,
    false,
    true,
);

/// JPEG Spectral Selection, Hierarchical (Process 21&23) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.62
pub static JPEGSpectralSelectionHierarchicalProcess21_and_23: TransferSyntax = TransferSyntax::new(
    &uids::JPEGSpectralSelectionHierarchicalProcess21_and_23,
    true,
    false,
    false,
    true,
);

/// JPEG Full Progression, Hierarchical (Process 24&26) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.63
pub static JPEGFullProgressionHierarchicalProcess24_and_26: TransferSyntax = TransferSyntax::new(
    &uids::JPEGFullProgressionHierarchicalProcess24_and_26,
    true,
    false,
    false,
    true,
);

/// JPEG Full Progression, Hierarchical (Process 25&27) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.64
pub static JPEGFullProgressionHierarchicalProcess25_and_27: TransferSyntax = TransferSyntax::new(
    &uids::JPEGFullProgressionHierarchicalProcess25_and_27,
    true,
    false,
    false,
    true,
);

/// JPEG Lossless, Hierarchical (Process 28) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.65
pub static JPEGLosslessHierarchicalProcess28: TransferSyntax = TransferSyntax::new(
    &uids::JPEGLosslessHierarchicalProcess28,
    true,
    false,
    false,
    true,
);

/// JPEG Lossless, Hierarchical (Process 29) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.66
pub static JPEGLosslessHierarchicalProcess29: TransferSyntax = TransferSyntax::new(
    &uids::JPEGLosslessHierarchicalProcess29,
    true,
    false,
    false,
    true,
);

/// JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1]): Default Transfer Syntax for Lossless JPEG Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.70
pub static JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1: TransferSyntax = TransferSyntax::new(
    &uids::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1,
    true,
    false,
    false,
    true,
);

/// JPEG-LS Lossless Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.80
pub static JPEGLSLosslessImageCompression: TransferSyntax = TransferSyntax::new(
    &uids::JPEGLSLosslessImageCompression,
    true,
    false,
    false,
    true,
);

/// JPEG-LS Lossy (Near-Lossless) Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.81
pub static JPEGLSLossyNearLosslessImageCompression: TransferSyntax = TransferSyntax::new(
    &uids::JPEGLSLossyNearLosslessImageCompression,
    true,
    false,
    false,
    true,
);

/// JPEG 2000 Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.90
pub static JPEG2000ImageCompressionLosslessOnly: TransferSyntax = TransferSyntax::new(
    &uids::JPEG2000ImageCompressionLosslessOnly,
    true,
    false,
    false,
    true,
);

/// JPEG 2000 Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.91
pub static JPEG2000ImageCompression: TransferSyntax = TransferSyntax::new(
    &uids::JPEG2000ImageCompression,
    true,
    false,
    false,
    true,
);

/// JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.92
pub static JPEG2000Part2MulticomponentImageCompressionLosslessOnly: TransferSyntax = TransferSyntax::new(
    &uids::JPEG2000Part2MulticomponentImageCompressionLosslessOnly,
    true,
    false,
    false,
    true,
);

/// JPEG 2000 Part 2 Multi-component Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.93
pub static JPEG2000Part2MulticomponentImageCompression: TransferSyntax = TransferSyntax::new(
    &uids::JPEG2000Part2MulticomponentImageCompression,
    true,
    false,
    false,
    true,
);

/// JPIP Referenced
///
/// - **UID:** 1.2.840.10008.1.2.4.94
pub static JPIPReferenced: TransferSyntax = TransferSyntax::new(
    &uids::JPIPReferenced,
    true,
    false,
    false,
    false,
);

/// JPIP Referenced Deflate
///
/// - **UID:** 1.2.840.10008.1.2.4.95
pub static JPIPReferencedDeflate: TransferSyntax = TransferSyntax::new(
    &uids::JPIPReferencedDeflate,
    true,
    false,
    true,
    false,
);

/// MPEG2 Main Profile / Main Level
///
/// - **UID:** 1.2.840.10008.1.2.4.100
pub static MPEG2MainProfileMainLevel: TransferSyntax = TransferSyntax::new(
    &uids::MPEG2MainProfileMainLevel,
    true,
    false,
    false,
    true,
);

/// Fragmentable MPEG2 Main Profile / Main Level
///
/// - **UID:** 1.2.840.10008.1.2.4.100.1
pub static FragmentableMPEG2MainProfileMainLevel: TransferSyntax = TransferSyntax::new(
    &uids::FragmentableMPEG2MainProfileMainLevel,
    true,
    false,
    false,
    true,
);

/// MPEG2 Main Profile / High Level
///
/// - **UID:** 1.2.840.10008.1.2.4.101
pub static MPEG2MainProfileHighLevel: TransferSyntax = TransferSyntax::new(
    &uids::MPEG2MainProfileHighLevel,
    true,
    false,
    false,
    true,
);

/// Fragmentable MPEG2 Main Profile / High Level
///
/// - **UID:** 1.2.840.10008.1.2.4.101.1
pub static FragmentableMPEG2MainProfileHighLevel: TransferSyntax = TransferSyntax::new(
    &uids::FragmentableMPEG2MainProfileHighLevel,
    true,
    false,
    false,
    true,
);

/// MPEG-4 AVC/H.264 High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.102
pub static MPEG4AVCH264HighProfileLevel41: TransferSyntax = TransferSyntax::new(
    &uids::MPEG4AVCH264HighProfileLevel41,
    true,
    false,
    false,
    true,
);

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.102.1
pub static FragmentableMPEG4AVCH264HighProfileLevel41: TransferSyntax = TransferSyntax::new(
    &uids::FragmentableMPEG4AVCH264HighProfileLevel41,
    true,
    false,
    false,
    true,
);

/// MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.103
pub static MPEG4AVCH264BDcompatibleHighProfileLevel41: TransferSyntax = TransferSyntax::new(
    &uids::MPEG4AVCH264BDcompatibleHighProfileLevel41,
    true,
    false,
    false,
    true,
);

/// Fragmentable MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.103.1
pub static FragmentableMPEG4AVCH264BDcompatibleHighProfileLevel41: TransferSyntax = TransferSyntax::new(
    &uids::FragmentableMPEG4AVCH264BDcompatibleHighProfileLevel41,
    true,
    false,
    false,
    true,
);

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.104
pub static MPEG4AVCH264HighProfileLevel42For2DVideo: TransferSyntax = TransferSyntax::new(
    &uids::MPEG4AVCH264HighProfileLevel42For2DVideo,
    true,
    false,
    false,
    true,
);

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.104.1
pub static FragmentableMPEG4AVCH264HighProfileLevel42For2DVideo: TransferSyntax = TransferSyntax::new(
    &uids::FragmentableMPEG4AVCH264HighProfileLevel42For2DVideo,
    true,
    false,
    false,
    true,
);

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.105
pub static MPEG4AVCH264HighProfileLevel42For3DVideo: TransferSyntax = TransferSyntax::new(
    &uids::MPEG4AVCH264HighProfileLevel42For3DVideo,
    true,
    false,
    false,
    true,
);

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.105.1
pub static FragmentableMPEG4AVCH264HighProfileLevel42For3DVideo: TransferSyntax = TransferSyntax::new(
    &uids::FragmentableMPEG4AVCH264HighProfileLevel42For3DVideo,
    true,
    false,
    false,
    true,
);

/// MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2
///
/// - **UID:** 1.2.840.10008.1.2.4.106
pub static MPEG4AVCH264StereoHighProfileLevel42: TransferSyntax = TransferSyntax::new(
    &uids::MPEG4AVCH264StereoHighProfileLevel42,
    true,
    false,
    false,
    true,
);

/// Fragmentable MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2
///
/// - **UID:** 1.2.840.10008.1.2.4.106.1
pub static FragmentableMPEG4AVCH264StereoHighProfileLevel42: TransferSyntax = TransferSyntax::new(
    &uids::FragmentableMPEG4AVCH264StereoHighProfileLevel42,
    true,
    false,
    false,
    true,
);

/// HEVC/H.265 Main Profile / Level 5.1
///
/// - **UID:** 1.2.840.10008.1.2.4.107
pub static HEVCH265MainProfileLevel51: TransferSyntax = TransferSyntax::new(
    &uids::HEVCH265MainProfileLevel51,
    true,
    false,
    false,
    true,
);

/// HEVC/H.265 Main 10 Profile / Level 5.1
///
/// - **UID:** 1.2.840.10008.1.2.4.108
pub static HEVCH265Main10ProfileLevel51: TransferSyntax = TransferSyntax::new(
    &uids::HEVCH265Main10ProfileLevel51,
    true,
    false,
    false,
    true,
);

/// High-Throughput JPEG 2000 Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.201
pub static HighThroughputJPEG2000ImageCompressionLosslessOnly: TransferSyntax = TransferSyntax::new(
    &uids::HighThroughputJPEG2000ImageCompressionLosslessOnly,
    true,
    false,
    false,
    true,
);

/// High-Throughput JPEG 2000 with RPCL Options Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.202
pub static HighThroughputJPEG2000withRPCLOptionsImageCompressionLosslessOnly: TransferSyntax = TransferSyntax::new(
    &uids::HighThroughputJPEG2000withRPCLOptionsImageCompressionLosslessOnly,
    true,
    false,
    false,
    true,
);

/// High-Throughput JPEG 2000 Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.203
pub static HighThroughputJPEG2000ImageCompression: TransferSyntax = TransferSyntax::new(
    &uids::HighThroughputJPEG2000ImageCompression,
    true,
    false,
    false,
    true,
);

/// JPIP HTJ2K Referenced
///
/// - **UID:** 1.2.840.10008.1.2.4.204
pub static JPIPHTJ2KReferenced: TransferSyntax = TransferSyntax::new(
    &uids::JPIPHTJ2KReferenced,
    true,
    false,
    false,
    false,
);

/// JPIP HTJ2K Referenced Deflate
///
/// - **UID:** 1.2.840.10008.1.2.4.205
pub static JPIPHTJ2KReferencedDeflate: TransferSyntax = TransferSyntax::new(
    &uids::JPIPHTJ2KReferencedDeflate,
    true,
    false,
    true,
    false,
);

/// RLE Lossless
///
/// - **UID:** 1.2.840.10008.1.2.5
pub static RLELossless: TransferSyntax = TransferSyntax::new(
    &uids::RLELossless,
    true,
    false,
    false,
    true,
);

/// RFC 2557 MIME encapsulation (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.6.1
pub static RFC2557MIMEencapsulation: TransferSyntax = TransferSyntax::new(
    &uids::RFC2557MIMEencapsulation,
    true,
    false,
    false,
    false,
);

/// XML Encoding (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.6.2
pub static XMLEncoding: TransferSyntax = TransferSyntax::new(
    &uids::XMLEncoding,
    true,
    false,
    false,
    false,
);

/// SMPTE ST 2110-20 Uncompressed Progressive Active Video
///
/// - **UID:** 1.2.840.10008.1.2.7.1
pub static SMPTEST211020UncompressedProgressiveActiveVideo: TransferSyntax = TransferSyntax::new(
    &uids::SMPTEST211020UncompressedProgressiveActiveVideo,
    true,
    false,
    false,
    false,
);

/// SMPTE ST 2110-20 Uncompressed Interlaced Active Video
///
/// - **UID:** 1.2.840.10008.1.2.7.2
pub static SMPTEST211020UncompressedInterlacedActiveVideo: TransferSyntax = TransferSyntax::new(
    &uids::SMPTEST211020UncompressedInterlacedActiveVideo,
    true,
    false,
    false,
    false,
);

/// SMPTE ST 2110-30 PCM Digital Audio
///
/// - **UID:** 1.2.840.10008.1.2.7.3
pub static SMPTEST211030PCMDigitalAudio: TransferSyntax = TransferSyntax::new(
    &uids::SMPTEST211030PCMDigitalAudio,
    true,
    false,
    false,
    false,
);

/// Papyrus 3 Implicit VR Little Endian (Retired)
///
/// - **UID:** 1.2.840.10008.1.20
pub static Papyrus3ImplicitVRLittleEndian: TransferSyntax = TransferSyntax::new(
    &uids::Papyrus3ImplicitVRLittleEndian,
    false,
    false,
    false,
    false,
);

/* automatically generated by rust-bindgen 0.69.2 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zxing_ImageView {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zxing_ReaderOptions {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zxing_Result {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zxing_Results {
	_unused: [u8; 0],
}
pub const zxing_ImageFormat_None: zxing_ImageFormat = 0;
pub const zxing_ImageFormat_Lum: zxing_ImageFormat = 16777216;
pub const zxing_ImageFormat_RGB: zxing_ImageFormat = 50331906;
pub const zxing_ImageFormat_BGR: zxing_ImageFormat = 50462976;
pub const zxing_ImageFormat_RGBX: zxing_ImageFormat = 67109122;
pub const zxing_ImageFormat_XRGB: zxing_ImageFormat = 67174915;
pub const zxing_ImageFormat_BGRX: zxing_ImageFormat = 67240192;
pub const zxing_ImageFormat_XBGR: zxing_ImageFormat = 67305985;
pub type zxing_ImageFormat = ::core::ffi::c_uint;
pub const zxing_BarcodeFormat_None: zxing_BarcodeFormat = 0;
pub const zxing_BarcodeFormat_Aztec: zxing_BarcodeFormat = 1;
pub const zxing_BarcodeFormat_Codabar: zxing_BarcodeFormat = 2;
pub const zxing_BarcodeFormat_Code39: zxing_BarcodeFormat = 4;
pub const zxing_BarcodeFormat_Code93: zxing_BarcodeFormat = 8;
pub const zxing_BarcodeFormat_Code128: zxing_BarcodeFormat = 16;
pub const zxing_BarcodeFormat_DataBar: zxing_BarcodeFormat = 32;
pub const zxing_BarcodeFormat_DataBarExpanded: zxing_BarcodeFormat = 64;
pub const zxing_BarcodeFormat_DataMatrix: zxing_BarcodeFormat = 128;
pub const zxing_BarcodeFormat_EAN8: zxing_BarcodeFormat = 256;
pub const zxing_BarcodeFormat_EAN13: zxing_BarcodeFormat = 512;
pub const zxing_BarcodeFormat_ITF: zxing_BarcodeFormat = 1024;
pub const zxing_BarcodeFormat_MaxiCode: zxing_BarcodeFormat = 2048;
pub const zxing_BarcodeFormat_PDF417: zxing_BarcodeFormat = 4096;
pub const zxing_BarcodeFormat_QRCode: zxing_BarcodeFormat = 8192;
pub const zxing_BarcodeFormat_UPCA: zxing_BarcodeFormat = 16384;
pub const zxing_BarcodeFormat_UPCE: zxing_BarcodeFormat = 32768;
pub const zxing_BarcodeFormat_MicroQRCode: zxing_BarcodeFormat = 65536;
pub const zxing_BarcodeFormat_RMQRCode: zxing_BarcodeFormat = 131072;
pub const zxing_BarcodeFormat_DXFilmEdge: zxing_BarcodeFormat = 262144;
pub const zxing_BarcodeFormat_LinearCodes: zxing_BarcodeFormat = 313214;
pub const zxing_BarcodeFormat_MatrixCodes: zxing_BarcodeFormat = 211073;
pub const zxing_BarcodeFormat_Any: zxing_BarcodeFormat = 524287;
pub const zxing_BarcodeFormat_Invalid: zxing_BarcodeFormat = 4294967295;
pub type zxing_BarcodeFormat = ::core::ffi::c_uint;
pub use self::zxing_BarcodeFormat as zxing_BarcodeFormats;
pub const zxing_Binarizer_LocalAverage: zxing_Binarizer = 0;
pub const zxing_Binarizer_GlobalHistogram: zxing_Binarizer = 1;
pub const zxing_Binarizer_FixedThreshold: zxing_Binarizer = 2;
pub const zxing_Binarizer_BoolCast: zxing_Binarizer = 3;
pub type zxing_Binarizer = ::core::ffi::c_uint;
pub const zxing_EanAddOnSymbol_Ignore: zxing_EanAddOnSymbol = 0;
pub const zxing_EanAddOnSymbol_Read: zxing_EanAddOnSymbol = 1;
pub const zxing_EanAddOnSymbol_Require: zxing_EanAddOnSymbol = 2;
pub type zxing_EanAddOnSymbol = ::core::ffi::c_uint;
pub const zxing_TextMode_Plain: zxing_TextMode = 0;
pub const zxing_TextMode_ECI: zxing_TextMode = 1;
pub const zxing_TextMode_HRI: zxing_TextMode = 2;
pub const zxing_TextMode_Hex: zxing_TextMode = 3;
pub const zxing_TextMode_Escaped: zxing_TextMode = 4;
pub type zxing_TextMode = ::core::ffi::c_uint;
pub const zxing_ContentType_Text: zxing_ContentType = 0;
pub const zxing_ContentType_Binary: zxing_ContentType = 1;
pub const zxing_ContentType_Mixed: zxing_ContentType = 2;
pub const zxing_ContentType_GS1: zxing_ContentType = 3;
pub const zxing_ContentType_ISO15434: zxing_ContentType = 4;
pub const zxing_ContentType_UnknownECI: zxing_ContentType = 5;
pub type zxing_ContentType = ::core::ffi::c_uint;
extern "C" {
	pub fn zxing_ImageView_new(
		data: *const u8,
		width: ::core::ffi::c_int,
		height: ::core::ffi::c_int,
		format: zxing_ImageFormat,
		rowStride: ::core::ffi::c_int,
		pixStride: ::core::ffi::c_int,
	) -> *mut zxing_ImageView;
	pub fn zxing_ImageView_delete(iv: *mut zxing_ImageView);
	pub fn zxing_ImageView_crop(
		iv: *mut zxing_ImageView,
		left: ::core::ffi::c_int,
		top: ::core::ffi::c_int,
		width: ::core::ffi::c_int,
		height: ::core::ffi::c_int,
	);
	pub fn zxing_ImageView_rotate(iv: *mut zxing_ImageView, degree: ::core::ffi::c_int);
	pub fn zxing_BarcodeFormatsFromString(str_: *const ::core::ffi::c_char) -> zxing_BarcodeFormats;
	pub fn zxing_BarcodeFormatFromString(str_: *const ::core::ffi::c_char) -> zxing_BarcodeFormat;
	pub fn zxing_BarcodeFormatToString(format: zxing_BarcodeFormat) -> *mut ::core::ffi::c_char;
	pub fn zxing_ReaderOptions_new() -> *mut zxing_ReaderOptions;
	pub fn zxing_ReaderOptions_delete(opts: *mut zxing_ReaderOptions);
	pub fn zxing_ReaderOptions_setTryHarder(opts: *mut zxing_ReaderOptions, tryHarder: bool);
	pub fn zxing_ReaderOptions_setTryRotate(opts: *mut zxing_ReaderOptions, tryRotate: bool);
	pub fn zxing_ReaderOptions_setTryInvert(opts: *mut zxing_ReaderOptions, tryInvert: bool);
	pub fn zxing_ReaderOptions_setTryDownscale(opts: *mut zxing_ReaderOptions, tryDownscale: bool);
	pub fn zxing_ReaderOptions_setIsPure(opts: *mut zxing_ReaderOptions, isPure: bool);
	pub fn zxing_ReaderOptions_setReturnErrors(opts: *mut zxing_ReaderOptions, returnErrors: bool);
	pub fn zxing_ReaderOptions_setFormats(opts: *mut zxing_ReaderOptions, formats: zxing_BarcodeFormats);
	pub fn zxing_ReaderOptions_setBinarizer(opts: *mut zxing_ReaderOptions, binarizer: zxing_Binarizer);
	pub fn zxing_ReaderOptions_setEanAddOnSymbol(opts: *mut zxing_ReaderOptions, eanAddOnSymbol: zxing_EanAddOnSymbol);
	pub fn zxing_ReaderOptions_setTextMode(opts: *mut zxing_ReaderOptions, textMode: zxing_TextMode);
	pub fn zxing_ReaderOptions_setMaxNumberOfSymbols(opts: *mut zxing_ReaderOptions, n: ::core::ffi::c_int);
	pub fn zxing_ReaderOptions_getTryHarder(opts: *const zxing_ReaderOptions) -> bool;
	pub fn zxing_ReaderOptions_getTryRotate(opts: *const zxing_ReaderOptions) -> bool;
	pub fn zxing_ReaderOptions_getTryInvert(opts: *const zxing_ReaderOptions) -> bool;
	pub fn zxing_ReaderOptions_getTryDownscale(opts: *const zxing_ReaderOptions) -> bool;
	pub fn zxing_ReaderOptions_getIsPure(opts: *const zxing_ReaderOptions) -> bool;
	pub fn zxing_ReaderOptions_getReturnErrors(opts: *const zxing_ReaderOptions) -> bool;
	pub fn zxing_ReaderOptions_getFormats(opts: *const zxing_ReaderOptions) -> zxing_BarcodeFormats;
	pub fn zxing_ReaderOptions_getBinarizer(opts: *const zxing_ReaderOptions) -> zxing_Binarizer;
	pub fn zxing_ReaderOptions_getEanAddOnSymbol(opts: *const zxing_ReaderOptions) -> zxing_EanAddOnSymbol;
	pub fn zxing_ReaderOptions_getTextMode(opts: *const zxing_ReaderOptions) -> zxing_TextMode;
	pub fn zxing_ReaderOptions_getMaxNumberOfSymbols(opts: *const zxing_ReaderOptions) -> ::core::ffi::c_int;
	pub fn zxing_ContentTypeToString(type_: zxing_ContentType) -> *mut ::core::ffi::c_char;
	pub fn zxing_Result_isValid(result: *const zxing_Result) -> bool;
	pub fn zxing_Result_errorMsg(result: *const zxing_Result) -> *mut ::core::ffi::c_char;
	pub fn zxing_Result_format(result: *const zxing_Result) -> zxing_BarcodeFormat;
	pub fn zxing_Result_contentType(result: *const zxing_Result) -> zxing_ContentType;
	pub fn zxing_Result_bytes(result: *const zxing_Result, len: *mut ::core::ffi::c_int) -> *mut u8;
	pub fn zxing_Result_text(result: *const zxing_Result) -> *mut ::core::ffi::c_char;
	pub fn zxing_Result_ecLevel(result: *const zxing_Result) -> *mut ::core::ffi::c_char;
	pub fn zxing_Result_symbologyIdentifier(result: *const zxing_Result) -> *mut ::core::ffi::c_char;
	pub fn zxing_Result_orientation(result: *const zxing_Result) -> ::core::ffi::c_int;
	pub fn zxing_Result_isInverted(result: *const zxing_Result) -> bool;
	pub fn zxing_Result_isMirrored(result: *const zxing_Result) -> bool;
	#[doc = " Note: opts is optional, i.e. it can be NULL, which will imply default settings."]
	pub fn zxing_ReadBarcode(iv: *const zxing_ImageView, opts: *const zxing_ReaderOptions) -> *mut zxing_Result;
	pub fn zxing_ReadBarcodes(iv: *const zxing_ImageView, opts: *const zxing_ReaderOptions) -> *mut zxing_Results;
	pub fn zxing_Result_delete(result: *mut zxing_Result);
	pub fn zxing_Results_delete(results: *mut zxing_Results);
	pub fn zxing_Results_size(results: *const zxing_Results) -> ::core::ffi::c_int;
	pub fn zxing_Results_at(results: *const zxing_Results, i: ::core::ffi::c_int) -> *const zxing_Result;
	pub fn zxing_Results_move(results: *mut zxing_Results, i: ::core::ffi::c_int) -> *mut zxing_Result;
	pub fn zxing_LastErrorMsg() -> *mut ::core::ffi::c_char;
	pub fn zxing_free(ptr: *mut ::core::ffi::c_void);
}

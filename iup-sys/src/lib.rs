//! Rust low level bindings for [IUP][1] -- A Portable User Interface Toolkit
//!
//! [1]: http://www.tecgraf.puc-rio.br/iup/
//!
//! These bindings follow what is found in iup.h as closely as possibly with
//! the following exceptions:
//!
//! 1. C function-like macros have been converted into functions.
//! 2. Items that were marked as old or deprecated have been removed.
#![feature(libc)]

// This file is based on iup.h. If you update this file, please follow the same
// formatting and ordering as found in iup.h to make comparison easy.

// #include "iupkey.h"
// #include "iupdef.h"

extern crate libc;

use libc::{c_char, c_uchar, c_int, c_float, c_double, c_void};

pub const IUP_NAME: &'static str         = "IUP - Portable User Interface";
pub const IUP_COPYRIGHT: &'static str    = "Copyright (C) 1994-2014 Tecgraf, PUC-Rio.";
pub const IUP_DESCRIPTION: &'static str  = "Multi-platform toolkit for building graphical user interfaces.";
pub const IUP_VERSION: &'static str      = "3.13"; /* bug fixes are reported only by IupVersion functions */
pub const IUP_VERSION_NUMBER: c_int      = 313000;
pub const IUP_VERSION_DATE: &'static str = "2014/11/19"; /* does not include bug fix releases */

pub enum Ihandle {}
pub type Icallback = extern fn(ih: *mut Ihandle) -> CallbackReturn;
pub type Iparamcb = extern fn (dialog: *mut Ihandle, param_index: c_int, user_data: *mut c_void) -> c_int;

extern {
    /************************************************************************/
    /*                        Main API                                      */
    /************************************************************************/
    pub fn IupOpen(argc: *const c_int, argv: *const *const *const c_char) -> c_int;
    pub fn IupClose();
    pub fn IupImageLibOpen();

    pub fn IupMainLoop() -> c_int;
    pub fn IupLoopStep() -> c_int;
    pub fn IupLoopStepWait() -> c_int;
    pub fn IupMainLoopLevel() -> c_int;
    pub fn IupFlush();
    pub fn IupExitLoop();

    pub fn IupRecordInput(filename: *const c_char, mode: c_int) -> c_int;
    pub fn IupPlayInput(filename: *const c_char) -> c_int;

    pub fn IupUpdate(ih: *mut Ihandle);
    pub fn IupUpdateChildren(ih: *mut Ihandle);
    pub fn IupRedraw(ih: *mut Ihandle, children: c_int);
    pub fn IupRefresh(ih: *mut Ihandle);
    pub fn IupRefreshChildren(ih: *mut Ihandle);

    pub fn IupHelp(url: *const c_char) -> c_int;
    pub fn IupLoad(filename: *const c_char) -> *mut c_char;
    pub fn IupLoadBuffer(buffer: *const c_char) -> *mut c_char;

    pub fn IupVersion() -> *mut c_char;
    pub fn IupVersionDate() -> *mut c_char;
    pub fn IupVersionNumber() -> c_int;

    pub fn IupSetLanguage(lng: *const c_char);
    pub fn IupGetLanguage() -> *mut c_char;
    pub fn IupSetLanguageString(name: *const c_char, str: *const c_char);
    pub fn IupStoreLanguageString(name: *const c_char, str: *const c_char);
    pub fn IupGetLanguageString(name: *const c_char) -> *mut c_char;
    pub fn IupSetLanguagePack(ih: *mut Ihandle);

    pub fn IupDestroy(ih: *mut Ihandle);
    pub fn IupDetach(child: *mut Ihandle);
    pub fn IupAppend(ih: *mut Ihandle, child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupInsert(ih: *mut Ihandle, ref_child: *mut Ihandle, child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGetChild(ih: *mut Ihandle, pos: c_int) -> *mut Ihandle;
    pub fn IupGetChildPos(ih: *mut Ihandle, child: *mut Ihandle) -> c_int;
    pub fn IupGetChildCount(ih: *mut Ihandle) -> c_int;
    pub fn IupGetNextChild(ih: *mut Ihandle, child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGetBrother(ih: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGetParent(ih: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGetDialog(ih: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGetDialogChild(ih: *mut Ihandle, name: *const c_char) -> *mut Ihandle;
    pub fn IupReparent(ih: *mut Ihandle, new_parent: *mut Ihandle, ref_child: *mut Ihandle) -> c_int;

    pub fn IupPopup(ih: *mut Ihandle, x: c_int, y: c_int) -> c_int;
    pub fn IupShow(ih: *mut Ihandle) -> c_int;
    pub fn IupShowXY(ih: *mut Ihandle, x: c_int, y: c_int) -> c_int;
    pub fn IupHide(ih: *mut Ihandle) -> c_int;
    pub fn IupMap(ih: *mut Ihandle) -> c_int;
    pub fn IupUnmap(ih: *mut Ihandle);

    pub fn IupResetAttribute(ih: *mut Ihandle, name: *const c_char);
    pub fn IupGetAllAttributes(ih: *mut Ihandle, names: *mut *mut c_char, n: c_int) -> c_int;
    pub fn IupSetAtt(handle_name: *const c_char, ih: *mut Ihandle, name: *const c_char, ...) -> *mut Ihandle;
    pub fn IupSetAttributes(ih: *mut Ihandle, str: *const c_char) -> *mut Ihandle;
    pub fn IupGetAttributes(ih: *mut Ihandle) -> *mut c_char;

    pub fn IupSetAttribute(ih: *mut Ihandle, name: *const c_char, value: *const c_char);
    pub fn IupSetStrAttribute(ih: *mut Ihandle, name: *const c_char, value: *const c_char);
    pub fn IupSetStrf(ih: *mut Ihandle, name: *const c_char, format: *const c_char, ...);
    pub fn IupSetInt(ih: *mut Ihandle, name: *const c_char, value: c_int);
    pub fn IupSetFloat(ih: *mut Ihandle, name: *const c_char, value: c_float);
    pub fn IupSetDouble(ih: *mut Ihandle, name: *const c_char, value: c_double);
    pub fn IupSetRGB(ih: *mut Ihandle, name: *const c_char, r: c_uchar, g: c_uchar, b: c_uchar);

    pub fn IupGetAttribute(ih: *mut Ihandle, name: *const c_char) -> *mut c_char;
    pub fn IupGetInt(ih: *mut Ihandle, name: *const c_char) -> c_int;
    pub fn IupGetInt2(ih: *mut Ihandle, name: *const c_char) -> c_int;
    pub fn IupGetIntInt(ih: *mut Ihandle, name: *const c_char, i1: *mut c_int, i2: *mut c_int) -> c_int;
    pub fn IupGetFloat(ih: *mut Ihandle, name: *const c_char) -> c_float;
    pub fn IupGetDouble(ih: *mut Ihandle, name: *const c_char) -> c_double;
    pub fn IupGetRGB(ih: *mut Ihandle, name: *const c_char, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar);
    pub fn IupSetAttributeId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: *const c_char);
    pub fn IupSetStrAttributeId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: *const c_char);
    pub fn IupSetStrfId(ih: *mut Ihandle, name: *const c_char, id: c_int, format: *const c_char, ...);
    pub fn IupSetIntId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: c_int);
    pub fn IupSetFloatId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: c_float);
    pub fn IupSetDoubleId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: c_double);
    pub fn IupSetRGBId(ih: *mut Ihandle, name: *const c_char, id: c_int, r: c_uchar, g: c_uchar, b: c_uchar);

    pub fn IupGetAttributeId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> *mut c_char;
    pub fn IupGetIntId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> c_int;
    pub fn IupGetFloatId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> c_float;
    pub fn IupGetDoubleId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> c_double;
    pub fn IupGetRGBId(ih: *mut Ihandle, name: *const c_char, id: c_int, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar);

    pub fn IupSetAttributeId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: *const c_char);
    pub fn IupSetStrAttributeId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: *const c_char);
    pub fn IupSetStrfId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, format: *const c_char, ...);
    pub fn IupSetIntId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: c_int);
    pub fn IupSetFloatId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: c_float);
    pub fn IupSetDoubleId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: c_double);
    pub fn IupSetRGBId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, r: c_uchar, g: c_uchar, b: c_uchar);

    pub fn IupGetAttributeId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> *mut c_char;
    pub fn IupGetIntId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> c_int;
    pub fn IupGetFloatId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> c_float;
    pub fn IupGetDoubleId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> c_double;
    pub fn IupGetRGBId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar);

    pub fn IupSetGlobal(name: *const c_char, value: *const c_char);
    pub fn IupSetStrGlobal(name: *const c_char, value: *const c_char);
    pub fn IupGetGlobal(name: *const c_char) -> *mut c_char;

    pub fn IupSetFocus(ih: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGetFocus() -> *mut Ihandle;
    pub fn IupPreviousField(ih: *mut Ihandle) -> *mut Ihandle;
    pub fn IupNextField(ih: *mut Ihandle) -> *mut Ihandle;

    pub fn IupGetCallback(ih: *mut Ihandle, name: *const c_char) -> Icallback;
    pub fn IupSetCallback(ih: *mut Ihandle, name: *const c_char, func: Icallback) -> Icallback;
    pub fn IupSetCallbacks(ih: *mut Ihandle, name: *const c_char, func: Icallback, ...) -> *mut Ihandle;

    pub fn IupGetFunction(name: *const c_char) -> Icallback;
    pub fn IupSetFunction(name: *const c_char, func: Icallback) -> Icallback;

    pub fn IupGetHandle(name: *const c_char) -> *mut Ihandle;
    pub fn IupSetHandle(name: *const c_char, ih: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGetAllNames(names: *mut *mut c_char, n: c_int) -> c_int;
    pub fn IupGetAllDialogs(names: *mut *mut c_char, n: c_int) -> c_int;
    pub fn IupGetName(ih: *mut Ihandle) -> *mut c_char;

    pub fn IupSetAttributeHandle(ih: *mut Ihandle, name: *const c_char, ih_named: *mut Ihandle);
    pub fn IupGetAttributeHandle(ih: *mut Ihandle, name: *const c_char) -> *mut Ihandle;

    pub fn IupGetClassName(ih: *mut Ihandle) -> *mut c_char;
    pub fn IupGetClassType(ih: *mut Ihandle) -> *mut c_char;
    pub fn IupGetAllClasses(names: *mut *mut c_char, n: c_int) -> c_int;
    pub fn IupGetClassAttributes(classname: *const c_char, names: *mut *mut c_char, n: c_int) -> c_int;
    pub fn IupGetClassCallbacks(classname: *const c_char, names: *mut *mut c_char, n: c_int) -> c_int;
    pub fn IupSaveClassAttributes(ih: *mut Ihandle);
    pub fn IupCopyClassAttributes(src_ih: *mut Ihandle, dst_ih: *mut Ihandle);
    pub fn IupSetClassDefaultAttribute(classname: *const c_char, name: *const c_char, value: *const c_char);
    pub fn IupClassMatch(ih: *mut Ihandle, classname: *const c_char) -> c_int;

    pub fn IupCreate(classname: *const c_char) -> *mut Ihandle;
    pub fn IupCreatev(classname: *const c_char, params: *mut *mut c_void) -> *mut Ihandle;
    pub fn IupCreatep(classname: *const c_char, first: *mut c_void, ...) -> *mut Ihandle;

    /************************************************************************/
    /*                        Elements                                      */
    /************************************************************************/
    pub fn IupFill() -> *mut Ihandle;
    pub fn IupRadio(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupVbox(child: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupVboxv(children: *mut *mut Ihandle) -> *mut Ihandle;
    pub fn IupZbox(child: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupZboxv(children: *mut *mut Ihandle) -> *mut Ihandle;
    pub fn IupHbox(child: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupHboxv(children: *mut *mut Ihandle) -> *mut Ihandle;

    pub fn IupNormalizer(ih_first: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupNormalizerv(ih_list: *mut *mut Ihandle) -> *mut Ihandle;

    pub fn IupCbox(child: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupCboxv(children: *mut *mut Ihandle) -> *mut Ihandle;
    pub fn IupSbox(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupSplit(child1: *mut Ihandle, child2: *mut Ihandle) -> *mut Ihandle;
    pub fn IupScrollBox(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGridBox(child: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupGridBoxv(children: *mut *mut Ihandle) -> *mut Ihandle;
    pub fn IupExpander(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupDetachBox(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupBackgroundBox(child: *mut Ihandle) -> *mut Ihandle;

    pub fn IupFrame(child: *mut Ihandle) -> *mut Ihandle;

    pub fn IupImage(width: c_int, height: c_int, pixmap: *const c_uchar) -> *mut Ihandle;
    pub fn IupImageRGB(width: c_int, height: c_int, pixmap: *const c_uchar) -> *mut Ihandle;
    pub fn IupImageRGBA(width: c_int, height: c_int, pixmap: *const c_uchar) -> *mut Ihandle;

    pub fn IupItem(title: *const c_char, action: *const c_char) -> *mut Ihandle;
    pub fn IupSubmenu(title: *const c_char, child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupSeparator() -> *mut Ihandle;
    pub fn IupMenu(child: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupMenuv(children: *mut *mut Ihandle) -> *mut Ihandle;

    pub fn IupButton(title: *const c_char, action: *const c_char) -> *mut Ihandle;
    pub fn IupCanvas(action: *const c_char) -> *mut Ihandle;
    pub fn IupDialog(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupUser() -> *mut Ihandle;
    pub fn IupLabel(title: *const c_char) -> *mut Ihandle;
    pub fn IupList(action: *const c_char) -> *mut Ihandle;
    pub fn IupText(action: *const c_char) -> *mut Ihandle;
    pub fn IupMultiLine(action: *const c_char) -> *mut Ihandle;
    pub fn IupToggle(title: *const c_char, action: *const c_char) -> *mut Ihandle;
    pub fn IupTimer() -> *mut Ihandle;
    pub fn IupClipboard() -> *mut Ihandle;
    pub fn IupProgressBar() -> *mut Ihandle;
    pub fn IupVal(_type: *const c_char) -> *mut Ihandle;
    pub fn IupTabs(child: *mut Ihandle, ...) -> *mut Ihandle;
    pub fn IupTabsv(children: *mut *mut Ihandle) -> *mut Ihandle;
    pub fn IupTree() -> *mut Ihandle;
    pub fn IupLink(url: *const c_char, title: *const c_char) -> *mut Ihandle;

    /************************************************************************/
    /*                      Utilities                                       */
    /************************************************************************/

    /* IupImage utility */
    pub fn IupSaveImageAsText(ih: *mut Ihandle, file_name: *const c_char, format: *const c_char, name: *const c_char) -> c_int;

    /* IupText and IupScintilla utilities */
    pub fn IupTextConvertLinColToPos(ih: *mut Ihandle, lin: c_int, col: c_int, pos: *mut c_int);
    pub fn IupTextConvertPosToLinCol(ih: *mut Ihandle, pos: c_int, lin: *mut c_int, col: *mut c_int);

    /* IupTree utilities */
    pub fn IupTreeSetUserId(ih: *mut Ihandle, id: c_int, userid: *mut c_void) -> c_int;
    pub fn IupTreeGetUserId(ih: *mut Ihandle, id: c_int) -> *mut c_void;
    pub fn IupTreeGetId(ih: *mut Ihandle, userid: *mut c_void) -> c_int;
    pub fn IupTreeSetAttributeHandle(ih: *mut Ihandle, name: *const c_char, id: c_int, ih_named: *mut Ihandle);

    /************************************************************************/
    /*                      Pre-definided dialogs                           */
    /************************************************************************/
    pub fn IupFileDlg() -> *mut Ihandle;
    pub fn IupMessageDlg() -> *mut Ihandle;
    pub fn IupColorDlg() -> *mut Ihandle;
    pub fn IupFontDlg() -> *mut Ihandle;
    pub fn IupProgressDlg() -> *mut Ihandle;

    pub fn IupGetFile(arq: *mut c_char) -> c_int;
    pub fn IupMessage(title: *const c_char, msg: *const c_char);
    pub fn IupMessagef(title: *const c_char, format: *const c_char, ...);
    pub fn IupAlarm(title: *const c_char, msg: *const c_char, b1: *const c_char, b2: *const c_char, b3: *const c_char) -> c_int;
    pub fn IupScanf(format: *const c_char, ...) -> c_int;
    pub fn IupListDialog(_type: c_int, title: *const c_char, size: c_int, list: *mut *const c_char, op: c_int, max_col: c_int, max_lin: c_int, marks: *mut c_int) -> c_int;
    pub fn IupGetText(title: *const c_char, text: *mut c_char) -> c_int;
    pub fn IupGetColor(x: c_int, y: c_int, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar) -> c_int;

    pub fn IupGetParam(title: *const c_char, action: Iparamcb, user_data: *mut c_void, format: *const c_char, ...) -> c_int;
    pub fn IupGetParamv(title: *const c_char, action: Iparamcb, user_data: *mut c_void, format: *const c_char, param_count: c_int, param_extra: c_int, param_data: *mut *mut c_void) -> c_int;

    pub fn IupLayoutDialog(dialog: *mut Ihandle) -> *mut Ihandle;
    pub fn IupElementPropertiesDialog(elem: *mut Ihandle) -> *mut Ihandle;
}

/************************************************************************/
/*                   Common Flags and Return Values                     */
/************************************************************************/
pub const IUP_ERROR: c_int      = 1;
pub const IUP_NOERROR: c_int    = 0;
pub const IUP_OPENED: c_int     = -1;
pub const IUP_INVALID: c_int    = -1;
pub const IUP_INVALID_ID: c_int = -10;

/************************************************************************/
/*                   Callback Return Values                             */
/************************************************************************/
#[repr(C)]
pub enum CallbackReturn {
    Ignore   = -1,
    Default  = -2,
    Close    = -3,
    Continue = -4,
}

/************************************************************************/
/*           IupPopup and IupShowXY Parameter Values                    */
/************************************************************************/
pub const IUP_CENTER: c_int       = 0xFFFF;  /* 65535 */
pub const IUP_LEFT: c_int         = 0xFFFE;  /* 65534 */
pub const IUP_RIGHT: c_int        = 0xFFFD;  /* 65533 */
pub const IUP_MOUSEPOS: c_int     = 0xFFFC;  /* 65532 */
pub const IUP_CURRENT: c_int      = 0xFFFB;  /* 65531 */
pub const IUP_CENTERPARENT: c_int = 0xFFFA;  /* 65530 */
pub const IUP_TOP: c_int          = IUP_LEFT;
pub const IUP_BOTTOM: c_int       = IUP_RIGHT;

/************************************************************************/
/*               SHOW_CB Callback Values                                */
/************************************************************************/
pub const IUP_SHOW: c_int     = 0;
pub const IUP_RESTORE: c_int  = 1;
pub const IUP_MINIMIZE: c_int = 2;
pub const IUP_MAXIMIZE: c_int = 3;
pub const IUP_HIDE: c_int     = 4;

/************************************************************************/
/*               SCROLL_CB Callback Values                              */
/************************************************************************/
pub const IUP_SBUP: c_int      =  0;
pub const IUP_SBDN: c_int      =  1;
pub const IUP_SBPGUP: c_int    =  2;
pub const IUP_SBPGDN: c_int    =  3;
pub const IUP_SBPOSV: c_int    =  4;
pub const IUP_SBDRAGV: c_int   =  5;
pub const IUP_SBLEFT: c_int    =  6;
pub const IUP_SBRIGHT: c_int   =  7;
pub const IUP_SBPGLEFT: c_int  =  8;
pub const IUP_SBPGRIGHT: c_int =  9;
pub const IUP_SBPOSH: c_int    = 10;
pub const IUP_SBDRAGH: c_int   = 11;

/************************************************************************/
/*               Mouse Button Values and Functions                      */
/************************************************************************/
pub const IUP_BUTTON1: c_char = '1' as c_char;
pub const IUP_BUTTON2: c_char = '2' as c_char;
pub const IUP_BUTTON3: c_char = '3' as c_char;
pub const IUP_BUTTON4: c_char = '4' as c_char;
pub const IUP_BUTTON5: c_char = '5' as c_char;

/*
pub fn iup_isshift(s: CString) -> bool   { s[0] == 'S' as c_char }
pub fn iup_iscontrol(s: CString) -> bool { s[1] == 'C' as c_char }
pub fn iup_isbutton1(s: CString) -> bool { s[2] == '1' as c_char }
pub fn iup_isbutton2(s: CString) -> bool { s[3] == '2' as c_char }
pub fn iup_isbutton3(s: CString) -> bool { s[4] == '3' as c_char }
pub fn iup_isdouble(s: CString) -> bool  { s[5] == 'D' as c_char }
pub fn iup_isalt(s: CString) -> bool     { s[6] == 'A' as c_char }
pub fn iup_issys(s: CString) -> bool     { s[7] == 'Y' as c_char }
pub fn iup_isbutton4(s: CString) -> bool { s[8] == '4' as c_char }
pub fn iup_isbutton5(s: CString) -> bool { s[9] == '5' as c_char }
*/

/************************************************************************/
/*                      Pre-Defined Masks                               */
/************************************************************************/
pub const IUP_MASK_FLOAT: &'static str  = "[+/-]?(/d+/.?/d*|/./d+)";
pub const IUP_MASK_UFLOAT: &'static str = "(/d+/.?/d*|/./d+)";
pub const IUP_MASK_EFLOAT: &'static str = "[+/-]?(/d+/.?/d*|/./d+)([eE][+/-]?/d+)?";
pub const IUP_MASK_INT: &'static str    = "[+/-]?/d+";
pub const IUP_MASK_UINT: &'static str   = "/d+";

/************************************************************************/
/*                   IupGetParam Callback situations                    */
/************************************************************************/
pub const IUP_GETPARAM_OK: c_int     = -1;
pub const IUP_GETPARAM_INIT: c_int   = -2;
pub const IUP_GETPARAM_CANCEL: c_int = -3;
pub const IUP_GETPARAM_HELP: c_int   = -4;

/************************************************************************/
/*                   Record Input Modes                                 */
/************************************************************************/
pub const IUP_RECBINARY: c_int = 0;
pub const IUP_RECTEXT: c_int = 1;

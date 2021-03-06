use com::{com_interface, interfaces::IUnknown};
use libc::{c_char, c_void};
use winapi::shared::guiddef::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypesbase::*;
use winapi::um::oaidl::{DISPID, DISPPARAMS, EXCEPINFO, VARIANT};
use winapi::um::objidl::SNB;
use winapi::um::objidlbase::STATSTG;
use winapi::um::wingdi::LOGPALETTE;
use winapi::um::winuser::*;

#[com_interface("00000118-0000-0000-C000-000000000046")]
pub trait IOleClientSite: IUnknown {
    unsafe fn save_object(&self) -> HRESULT;
    unsafe fn get_moniker(
        &self,
        dw_assign: DWORD,
        dw_which_moniker: DWORD,
        ppmk: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn get_container(&self, pp_container: *mut *mut c_void) -> HRESULT;
    unsafe fn show_object(&self) -> HRESULT;
    unsafe fn on_show_window(&self, show: BOOL) -> HRESULT;
    unsafe fn request_new_object_layout(&self) -> HRESULT;
}

#[com_interface("00000112-0000-0000-C000-000000000046")]
pub trait IOleObject: IUnknown {
    unsafe fn set_client_site(&self, p_client_site: *mut c_void) -> HRESULT;
    unsafe fn get_client_site(&self, p_client_site: *mut *mut c_void) -> HRESULT;
    unsafe fn set_host_names(
        &self,
        sz_container_app: *const c_char,
        sz_container_obj: *const c_char,
    ) -> HRESULT;
    unsafe fn close(&self, dw_save_option: DWORD) -> HRESULT;
    unsafe fn set_moniker(&self, dw_which_moniker: DWORD, pmk: *mut c_void);
    unsafe fn get_moniker(
        &self,
        dw_assign: DWORD,
        dw_which_moniker: DWORD,
        ppmk: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn init_from_data(
        &self,
        p_data_object: *mut c_void,
        f_creation: BOOL,
        dw_reserved: DWORD,
    ) -> HRESULT;
    unsafe fn get_clipboard_data(
        &self,
        dw_reserved: DWORD,
        pp_data_object: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn do_verb(
        &self,
        i_verb: LONG,
        lpmsg: LPMSG,
        p_active_site: *mut c_void,
        lindex: LONG,
        hwnd_parent: HWND,
        lprc_pos_rect: LPCRECT,
    ) -> HRESULT;
    unsafe fn enum_verbs(&self, pp_enum_ole_verb: *mut *mut c_void) -> HRESULT;
    unsafe fn update(&self) -> HRESULT;
    unsafe fn is_up_to_date(&self) -> HRESULT;
    unsafe fn get_user_class_id(&self, p_clsid: *mut CLSID) -> HRESULT;
    unsafe fn get_user_type(&self, dw_form_of_type: DWORD, psz_user_type: *mut LPOLESTR)
        -> HRESULT;
    unsafe fn set_extent(&self, dw_draw_aspect: DWORD, psizel: *mut SIZEL) -> HRESULT;
    unsafe fn get_extent(&self, dw_draw_aspect: DWORD, psizel: *mut SIZEL) -> HRESULT;
    unsafe fn advise(&self, p_advise_sink: *mut c_void, pdw_connection: *mut DWORD) -> HRESULT;
    unsafe fn unadvise(&self, dw_connection: DWORD) -> HRESULT;
    unsafe fn enum_advise(&self, ppenum_advise: *mut *mut c_void) -> HRESULT;
    unsafe fn get_misc_status(&self, dw_aspect: DWORD, pdw_status: *mut DWORD) -> HRESULT;
    unsafe fn set_color_scheme(&self, p_logpal: *mut LOGPALETTE) -> HRESULT;
}

#[com_interface("00000114-0000-0000-C000-000000000046")]
pub trait IOleWindow: IUnknown {
    unsafe fn get_window(&self, phwnd: *mut HWND) -> HRESULT;
    unsafe fn context_sensitive_help(&self, f_enter_mode: BOOL) -> HRESULT;
}

#[com_interface("00000113-0000-0000-C000-000000000046")]
pub trait IOleInPlaceObject: IOleWindow {
    unsafe fn in_place_deactivate(&self) -> HRESULT;
    unsafe fn ui_deactivate(&self) -> HRESULT;
    unsafe fn set_object_rects(&self, lprc_pos_rect: LPCRECT, lprc_clip_rect: LPCRECT) -> HRESULT;
    unsafe fn reactivate_and_undo(&self) -> HRESULT;
}

#[com_interface("00000119-0000-0000-C000-000000000046")]
pub trait IOleInPlaceSite: IOleWindow {
    unsafe fn can_in_place_activate(&self) -> HRESULT;
    unsafe fn on_in_place_activate(&self) -> HRESULT;
    unsafe fn on_ui_activate(&self) -> HRESULT;
    unsafe fn get_window_context(
        &self,
        pp_frame: *mut *mut c_void,
        pp_doc: *mut *mut c_void,
        lprc_pos_rect: LPRECT,
        lprc_clip_rect: LPRECT,
        lp_frame_info: *mut OLEINPLACEFRAMEINFO,
    ) -> HRESULT;
    unsafe fn scroll(&self, scroll_extant: SIZE) -> HRESULT;
    unsafe fn on_ui_deactivate(&self, f_undoable: BOOL) -> HRESULT;
    unsafe fn on_in_place_deactivate(&self) -> HRESULT;
    unsafe fn discard_undo_state(&self) -> HRESULT;
    unsafe fn deactivate_and_undo(&self) -> HRESULT;
    unsafe fn on_pos_rect_change(&self, lprc_post_rect: LPRECT) -> HRESULT;
}

#[com_interface("0000000b-0000-0000-C000-000000000046")]
pub trait IStorage: IUnknown {
    unsafe fn create_stream(
        &self,
        pwcs_name: *const WCHAR,
        grf_mode: DWORD,
        reserved1: DWORD,
        reserved2: DWORD,
        ppstm: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn open_stream(
        &self,
        pwcs_name: *const WCHAR,
        reserved1: *mut c_void,
        grf_mode: DWORD,
        reserved2: DWORD,
        ppstm: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn create_storage(
        &self,
        pwcs_name: *const WCHAR,
        grf_mode: DWORD,
        reserved1: DWORD,
        reserved2: DWORD,
        ppstg: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn open_storage(
        &self,
        pwcs_name: *const WCHAR,
        pstg_priority: *mut c_void,
        grf_mode: DWORD,
        snb_exclude: SNB,
        reserved: DWORD,
        ppstg: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn copy_to(
        &self,
        ciid_exclude: DWORD,
        rgiid_exclude: *const IID,
        snb_exclude: SNB,
        pstg_dest: *mut c_void,
    ) -> HRESULT;
    unsafe fn move_element_to(
        &self,
        pwcs_name: *const WCHAR,
        pstg_dest: *mut c_void,
        pwcs_new_name: *const WCHAR,
        grf_flags: DWORD,
    ) -> HRESULT;
    unsafe fn commit(&self, grf_commit_flags: DWORD) -> HRESULT;
    unsafe fn revert(&self) -> HRESULT;
    unsafe fn enum_elements(
        &self,
        reserved1: DWORD,
        reserved2: *mut c_void,
        reserved3: DWORD,
        ppenum: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn destroy_element(&self, pwcs_name: *const WCHAR) -> HRESULT;
    unsafe fn rename_element(
        &self,
        pwcs_old_name: *const WCHAR,
        pwcs_new_name: *const WCHAR,
    ) -> HRESULT;
    unsafe fn set_element_times(
        &self,
        pwcs_name: *const WCHAR,
        pctime: *const FILETIME,
        patime: *const FILETIME,
        pmtime: *const FILETIME,
    ) -> HRESULT;
    unsafe fn set_class(&self, clsid: REFCLSID) -> HRESULT;
    unsafe fn set_state_bits(&self, grf_state_bits: DWORD, grf_mask: DWORD) -> HRESULT;
    unsafe fn stat(&self, pstatstg: *mut STATSTG, grf_stat_flag: DWORD) -> HRESULT;
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct OLEINPLACEFRAMEINFO {
    // OIFI
    pub cb: UINT,
    pub fMDIApp: BOOL,
    pub hwndFrame: HWND,
    pub haccel: HACCEL,
    pub cAccelEntries: UINT,
}

impl Default for OLEINPLACEFRAMEINFO {
    #[inline]
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// #[com_interface("8856F961-340A-11D0-A96B-00C04FD705A2")] // CLSID
// pub trait WebBrowserCLS : IUnknown {}

#[com_interface("00020400-0000-0000-C000-000000000046")]
pub trait IDispatch: IUnknown {
    unsafe fn get_type_info_count(&self, pctinfo: *mut UINT) -> HRESULT;
    unsafe fn get_type_info(
        &self,
        i_ti_info: UINT,
        icid: LCID,
        pp_ti_info: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn get_ids_of_names(
        &self,
        riid: *const IID,
        rgsz_names: *mut LPOLESTR,
        c_names: UINT,
        lcid: LCID,
        rg_disp_id: *mut DISPID,
    ) -> HRESULT;
    unsafe fn invoke(
        &self,
        disp_id_member: DISPID,
        riid: *const IID,
        lcid: LCID,
        w_flags: WORD,
        p_disp_params: *mut DISPPARAMS,
        p_var_result: *mut VARIANT,
        p_excep_info: *mut EXCEPINFO,
        pu_arg_err: *mut UINT,
    ) -> HRESULT;
}

#[com_interface("EAB22AC1-30C1-11CF-A7EB-0000C05BAE0B")]
pub trait IWebBrowser: IDispatch {
    unsafe fn go_back(&self) -> HRESULT;
    unsafe fn go_forward(&self) -> HRESULT;
    unsafe fn go_home(&self) -> HRESULT;
    unsafe fn go_search(&self) -> HRESULT;
    unsafe fn navigate(
        &self,
        url: BSTR,
        flags: *mut VARIANT,
        target_frame_name: *mut VARIANT,
        post_data: *mut VARIANT,
        headers: *mut VARIANT,
    ) -> HRESULT;
    unsafe fn refresh(&self) -> HRESULT;
    unsafe fn refresh2(&self) -> HRESULT;
    unsafe fn stop(&self) -> HRESULT;
    unsafe fn get_application(&self) -> HRESULT;
    unsafe fn get_parent(&self) -> HRESULT;
    unsafe fn get_container(&self) -> HRESULT;
    unsafe fn get_document(&self) -> HRESULT;
    unsafe fn get_top_level_container(&self) -> HRESULT;
    unsafe fn get_type(&self) -> HRESULT;
    unsafe fn get_left(&self) -> HRESULT;
    unsafe fn put_left(&self) -> HRESULT;
    unsafe fn get_top(&self) -> HRESULT;
    unsafe fn put_top(&self) -> HRESULT;
    unsafe fn get_width(&self) -> HRESULT;
    unsafe fn put_width(&self) -> HRESULT;
    unsafe fn get_height(&self) -> HRESULT;
    unsafe fn put_height(&self) -> HRESULT;
    unsafe fn get_location_name(&self) -> HRESULT;
    unsafe fn get_location_url(&self) -> HRESULT;
    unsafe fn get_busy(&self) -> HRESULT;
}

// #[com_interface("0002DF05-0000-0000-C000-000000000046")]
// pub trait IWebBrowserApp : IWebBrowser {
//     unsafe fn quit(&self) -> HRESULT;
//     unsafe fn client_to_window(&self) -> HRESULT;
//     unsafe fn put_property(&self) -> HRESULT;
//     unsafe fn get_property(&self) -> HRESULT;
//     unsafe fn get_name(&self) -> HRESULT;
//     unsafe fn get_hwnd(&self) -> HRESULT;
//     unsafe fn get_full_name(&self) -> HRESULT;
//     unsafe fn get_path(&self) -> HRESULT;
//     unsafe fn get_visible(&self) -> HRESULT;
//     unsafe fn put_visible(&self) -> HRESULT;
//     unsafe fn get_status_bar(&self) -> HRESULT;
//     unsafe fn put_status_bar(&self) -> HRESULT;
//     unsafe fn get_status_text(&self) -> HRESULT;
//     unsafe fn put_status_text(&self) -> HRESULT;
//     unsafe fn get_tool_bar(&self) -> HRESULT;
//     unsafe fn put_tool_bar(&self) -> HRESULT;
//     unsafe fn get_menu_bar(&self) -> HRESULT;
//     unsafe fn put_menu_bar(&self) -> HRESULT;
//     unsafe fn get_full_screen(&self) -> HRESULT;
//     unsafe fn put_full_screen(&self) -> HRESULT;
// }

// #[com_interface("D30C1661-CDAF-11d0-8A3E-00C04FC9E26E")]
// pub trait IWebBrowser2: IWebBrowserApp {
//     unsafe fn navigate2(&self) -> HRESULT;
//     unsafe fn query_status_wb(&self) -> HRESULT;
//     unsafe fn exec_wb(&self) -> HRESULT;
//     unsafe fn show_browser_bar(&self) -> HRESULT;
//     unsafe fn get_ready_state(&self) -> HRESULT;
//     unsafe fn get_offline(&self) -> HRESULT;
//     unsafe fn put_offline(&self) -> HRESULT;
//     unsafe fn get_silent(&self) -> HRESULT;
//     unsafe fn put_silent(&self) -> HRESULT;
//     unsafe fn get_register_as_browser(&self) -> HRESULT;
//     unsafe fn put_register_as_browser(&self) -> HRESULT;
//     unsafe fn get_register_as_drop_target(&self) -> HRESULT;
//     unsafe fn put_register_as_drop_target(&self) -> HRESULT;
//     unsafe fn get_theater_mode(&self) -> HRESULT;
//     unsafe fn put_theater_mode(&self) -> HRESULT;
//     unsafe fn get_address_bar(&self) -> HRESULT;
//     unsafe fn put_address_bar(&self) -> HRESULT;
//     unsafe fn get_resizable(&self) -> HRESULT;
//     unsafe fn put_resizable(&self) -> HRESULT;
// }

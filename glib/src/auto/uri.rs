// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib_sys;
use std::fmt;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use std::ptr;
use translate::*;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use Bytes;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use Error;
use GString;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use UriFlags;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use UriHideFlags;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Uri(Shared<glib_sys::GUri>);

    match fn {
        ref => |ptr| glib_sys::g_uri_ref(ptr),
        unref => |ptr| glib_sys::g_uri_unref(ptr),
        get_type => || glib_sys::g_uri_get_type(),
    }
}

impl Uri {
    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_auth_params(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_auth_params(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_flags(&self) -> UriFlags {
        unsafe { from_glib(glib_sys::g_uri_get_flags(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_fragment(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_fragment(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_host(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_host(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_password(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_password(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_path(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_path(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_port(&self) -> i32 {
        unsafe { glib_sys::g_uri_get_port(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_query(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_query(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_scheme(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_scheme(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_user(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_user(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn get_userinfo(&self) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_get_userinfo(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn parse_relative(&self, uri_ref: &str, flags: UriFlags) -> Result<Uri, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_uri_parse_relative(
                self.to_glib_none().0,
                uri_ref.to_glib_none().0,
                flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    fn to_string(&self) -> GString {
        unsafe { from_glib_full(glib_sys::g_uri_to_string(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn to_string_partial(&self, flags: UriHideFlags) -> GString {
        unsafe {
            from_glib_full(glib_sys::g_uri_to_string_partial(
                self.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn build(
        flags: UriFlags,
        scheme: &str,
        userinfo: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> Uri {
        unsafe {
            from_glib_full(glib_sys::g_uri_build(
                flags.to_glib(),
                scheme.to_glib_none().0,
                userinfo.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn build_with_user(
        flags: UriFlags,
        scheme: &str,
        user: Option<&str>,
        password: Option<&str>,
        auth_params: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> Uri {
        unsafe {
            from_glib_full(glib_sys::g_uri_build_with_user(
                flags.to_glib(),
                scheme.to_glib_none().0,
                user.to_glib_none().0,
                password.to_glib_none().0,
                auth_params.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn escape_bytes(unescaped: &[u8], reserved_chars_allowed: Option<&str>) -> GString {
        let length = unescaped.len() as usize;
        unsafe {
            from_glib_full(glib_sys::g_uri_escape_bytes(
                unescaped.to_glib_none().0,
                length,
                reserved_chars_allowed.to_glib_none().0,
            ))
        }
    }

    pub fn escape_string(
        unescaped: &str,
        reserved_chars_allowed: Option<&str>,
        allow_utf8: bool,
    ) -> GString {
        unsafe {
            from_glib_full(glib_sys::g_uri_escape_string(
                unescaped.to_glib_none().0,
                reserved_chars_allowed.to_glib_none().0,
                allow_utf8.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn is_valid(uri_string: &str, flags: UriFlags) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ =
                glib_sys::g_uri_is_valid(uri_string.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn join(
        flags: UriFlags,
        scheme: Option<&str>,
        userinfo: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> GString {
        unsafe {
            from_glib_full(glib_sys::g_uri_join(
                flags.to_glib(),
                scheme.to_glib_none().0,
                userinfo.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn join_with_user(
        flags: UriFlags,
        scheme: Option<&str>,
        user: Option<&str>,
        password: Option<&str>,
        auth_params: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> GString {
        unsafe {
            from_glib_full(glib_sys::g_uri_join_with_user(
                flags.to_glib(),
                scheme.to_glib_none().0,
                user.to_glib_none().0,
                password.to_glib_none().0,
                auth_params.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    pub fn list_extract_uris(uri_list: &str) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(glib_sys::g_uri_list_extract_uris(
                uri_list.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn parse(uri_string: &str, flags: UriFlags) -> Result<Uri, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                glib_sys::g_uri_parse(uri_string.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[cfg(any(feature = "v2_66", feature = "dox"))]
    //pub fn parse_params(params: &str, separators: &str, flags: UriParamsFlags) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, Error> {
    //    unsafe { TODO: call glib_sys:g_uri_parse_params() }
    //}

    pub fn parse_scheme(uri: &str) -> Option<GString> {
        unsafe { from_glib_full(glib_sys::g_uri_parse_scheme(uri.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn peek_scheme(uri: &str) -> Option<GString> {
        unsafe { from_glib_none(glib_sys::g_uri_peek_scheme(uri.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn resolve_relative(
        base_uri_string: Option<&str>,
        uri_ref: &str,
        flags: UriFlags,
    ) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_uri_resolve_relative(
                base_uri_string.to_glib_none().0,
                uri_ref.to_glib_none().0,
                flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn split(
        uri_ref: &str,
        flags: UriFlags,
    ) -> Result<
        (
            Option<GString>,
            Option<GString>,
            Option<GString>,
            i32,
            GString,
            Option<GString>,
            Option<GString>,
        ),
        Error,
    > {
        unsafe {
            let mut scheme = ptr::null_mut();
            let mut userinfo = ptr::null_mut();
            let mut host = ptr::null_mut();
            let mut port = mem::MaybeUninit::uninit();
            let mut path = ptr::null_mut();
            let mut query = ptr::null_mut();
            let mut fragment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_uri_split(
                uri_ref.to_glib_none().0,
                flags.to_glib(),
                &mut scheme,
                &mut userinfo,
                &mut host,
                port.as_mut_ptr(),
                &mut path,
                &mut query,
                &mut fragment,
                &mut error,
            );
            let port = port.assume_init();
            if error.is_null() {
                Ok((
                    from_glib_full(scheme),
                    from_glib_full(userinfo),
                    from_glib_full(host),
                    port,
                    from_glib_full(path),
                    from_glib_full(query),
                    from_glib_full(fragment),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn split_network(
        uri_string: &str,
        flags: UriFlags,
    ) -> Result<(Option<GString>, Option<GString>, i32), Error> {
        unsafe {
            let mut scheme = ptr::null_mut();
            let mut host = ptr::null_mut();
            let mut port = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_uri_split_network(
                uri_string.to_glib_none().0,
                flags.to_glib(),
                &mut scheme,
                &mut host,
                port.as_mut_ptr(),
                &mut error,
            );
            let port = port.assume_init();
            if error.is_null() {
                Ok((from_glib_full(scheme), from_glib_full(host), port))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn split_with_user(
        uri_ref: &str,
        flags: UriFlags,
    ) -> Result<
        (
            Option<GString>,
            Option<GString>,
            Option<GString>,
            Option<GString>,
            Option<GString>,
            i32,
            GString,
            Option<GString>,
            Option<GString>,
        ),
        Error,
    > {
        unsafe {
            let mut scheme = ptr::null_mut();
            let mut user = ptr::null_mut();
            let mut password = ptr::null_mut();
            let mut auth_params = ptr::null_mut();
            let mut host = ptr::null_mut();
            let mut port = mem::MaybeUninit::uninit();
            let mut path = ptr::null_mut();
            let mut query = ptr::null_mut();
            let mut fragment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_uri_split_with_user(
                uri_ref.to_glib_none().0,
                flags.to_glib(),
                &mut scheme,
                &mut user,
                &mut password,
                &mut auth_params,
                &mut host,
                port.as_mut_ptr(),
                &mut path,
                &mut query,
                &mut fragment,
                &mut error,
            );
            let port = port.assume_init();
            if error.is_null() {
                Ok((
                    from_glib_full(scheme),
                    from_glib_full(user),
                    from_glib_full(password),
                    from_glib_full(auth_params),
                    from_glib_full(host),
                    port,
                    from_glib_full(path),
                    from_glib_full(query),
                    from_glib_full(fragment),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    pub fn unescape_bytes(
        escaped_string: &str,
        illegal_characters: Option<&str>,
    ) -> Result<Bytes, Error> {
        let length = escaped_string.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_uri_unescape_bytes(
                escaped_string.to_glib_none().0,
                length,
                illegal_characters.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn unescape_segment(
        escaped_string: Option<&str>,
        escaped_string_end: Option<&str>,
        illegal_characters: Option<&str>,
    ) -> Option<GString> {
        unsafe {
            from_glib_full(glib_sys::g_uri_unescape_segment(
                escaped_string.to_glib_none().0,
                escaped_string_end.to_glib_none().0,
                illegal_characters.to_glib_none().0,
            ))
        }
    }

    pub fn unescape_string(
        escaped_string: &str,
        illegal_characters: Option<&str>,
    ) -> Option<GString> {
        unsafe {
            from_glib_full(glib_sys::g_uri_unescape_string(
                escaped_string.to_glib_none().0,
                illegal_characters.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Uri {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}

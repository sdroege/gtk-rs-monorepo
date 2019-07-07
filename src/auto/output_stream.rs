// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "futures")]
use futures::future;
use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;
use Cancellable;
use Error;
use InputStream;
use OutputStreamSpliceFlags;

glib_wrapper! {
    pub struct OutputStream(Object<gio_sys::GOutputStream, gio_sys::GOutputStreamClass, OutputStreamClass>);

    match fn {
        get_type => || gio_sys::g_output_stream_get_type(),
    }
}

pub const NONE_OUTPUT_STREAM: Option<&OutputStream> = None;

pub trait OutputStreamExt: 'static {
    fn clear_pending(&self);

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin>;

    fn flush<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn flush_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn flush_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin>;

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    fn is_closing(&self) -> bool;

    //fn printf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<usize>;

    fn set_pending(&self) -> Result<(), Error>;

    fn splice<P: IsA<InputStream>, Q: IsA<Cancellable>>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Q>,
    ) -> Result<isize, Error>;

    fn splice_async<
        P: IsA<InputStream>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<isize, Error>) + Send + 'static,
    >(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    );

    #[cfg(feature = "futures")]
    fn splice_async_future<P: IsA<InputStream> + Clone + 'static>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<isize, Error>> + std::marker::Unpin>;

    //fn vprintf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize>;

    fn write<P: IsA<Cancellable>>(
        &self,
        buffer: &[u8],
        cancellable: Option<&P>,
    ) -> Result<isize, Error>;

    fn write_bytes<P: IsA<Cancellable>>(
        &self,
        bytes: &glib::Bytes,
        cancellable: Option<&P>,
    ) -> Result<isize, Error>;

    fn write_bytes_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn write_bytes_async_future(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<isize, Error>> + std::marker::Unpin>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, Error>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_all<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, Error>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_all_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_all_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Box_<dyn future::Future<Output = Result<usize, Error>> + std::marker::Unpin>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Box_<dyn future::Future<Output = Result<usize, Error>> + std::marker::Unpin>;
}

impl<O: IsA<OutputStream>> OutputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            gio_sys::g_output_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_output_stream_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn close_async_trampoline<
            Q: FnOnce(Result<(), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                gio_sys::g_output_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_output_stream_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.close_async(io_priority, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn flush<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_output_stream_flush(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn flush_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn flush_async_trampoline<
            Q: FnOnce(Result<(), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                gio_sys::g_output_stream_flush_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = flush_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_output_stream_flush_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn flush_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.flush_async(io_priority, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_output_stream_has_pending(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_output_stream_is_closed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closing(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_output_stream_is_closing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn printf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<usize> {
    //    unsafe { TODO: call gio_sys:g_output_stream_printf() }
    //}

    fn set_pending(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ =
                gio_sys::g_output_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn splice<P: IsA<InputStream>, Q: IsA<Cancellable>>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Q>,
    ) -> Result<isize, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_output_stream_splice(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn splice_async<
        P: IsA<InputStream>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<isize, Error>) + Send + 'static,
    >(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn splice_async_trampoline<
            R: FnOnce(Result<isize, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_output_stream_splice_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = splice_async_trampoline::<R>;
        unsafe {
            gio_sys::g_output_stream_splice_async(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                flags.to_glib(),
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn splice_async_future<P: IsA<InputStream> + Clone + 'static>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<isize, Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        let source = source.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.splice_async(
                &source,
                flags,
                io_priority,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    //fn vprintf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize> {
    //    unsafe { TODO: call gio_sys:g_output_stream_vprintf() }
    //}

    fn write<P: IsA<Cancellable>>(
        &self,
        buffer: &[u8],
        cancellable: Option<&P>,
    ) -> Result<isize, Error> {
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_output_stream_write(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn write_bytes<P: IsA<Cancellable>>(
        &self,
        bytes: &glib::Bytes,
        cancellable: Option<&P>,
    ) -> Result<isize, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_output_stream_write_bytes(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn write_bytes_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn write_bytes_async_trampoline<
            Q: FnOnce(Result<isize, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_output_stream_write_bytes_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = write_bytes_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_output_stream_write_bytes_async(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn write_bytes_async_future(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<isize, Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        let bytes = bytes.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.write_bytes_async(&bytes, io_priority, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, Error> {
    //    unsafe { TODO: call gio_sys:g_output_stream_writev() }
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_all<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, Error> {
    //    unsafe { TODO: call gio_sys:g_output_stream_writev_all() }
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_all_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call gio_sys:g_output_stream_writev_all_async() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_all_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Box_<dyn future::Future<Output = Result<usize, Error>> + std::marker::Unpin> {
    //use GioFuture;
    //use fragile::Fragile;

    //let vectors = vectors.clone();
    //GioFuture::new(self, move |obj, send| {
    //    let cancellable = Cancellable::new();
    //    let send = Fragile::new(send);
    //    obj.writev_all_async(
    //        &vectors,
    //        io_priority,
    //        Some(&cancellable),
    //        move |res| {
    //            let _ = send.into_inner().send(res);
    //        },
    //    );

    //    cancellable
    //})
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call gio_sys:g_output_stream_writev_async() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //fn writev_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Box_<dyn future::Future<Output = Result<usize, Error>> + std::marker::Unpin> {
    //use GioFuture;
    //use fragile::Fragile;

    //let vectors = vectors.clone();
    //GioFuture::new(self, move |obj, send| {
    //    let cancellable = Cancellable::new();
    //    let send = Fragile::new(send);
    //    obj.writev_async(
    //        &vectors,
    //        io_priority,
    //        Some(&cancellable),
    //        move |res| {
    //            let _ = send.into_inner().send(res);
    //        },
    //    );

    //    cancellable
    //})
    //}
}

impl fmt::Display for OutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OutputStream")
    }
}

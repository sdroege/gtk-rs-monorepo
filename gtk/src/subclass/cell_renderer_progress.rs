use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use CellRenderer;
use CellRendererProgress;

pub trait CellRendererProgressImpl: CellRendererImpl {}

unsafe impl<T: CellRendererProgressImpl> IsSubclassable<T> for CellRendererProgress {
    fn override_vfuncs(class: &mut ::glib::object::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::override_vfuncs(class);
    }
}

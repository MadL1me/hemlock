use crate::vendors::FileVendor;
use crate::VendorSource;

pub struct LocalVendor;

impl FileVendor for LocalVendor {
    fn vendor(&self, source: VendorSource) -> Result<&str, crate::AnyError> {
        todo!()
    }
}
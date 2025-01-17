pub use fluvio_controlplane_metadata::tableformat::*;

mod convert {

    use crate::{CreatableAdminSpec, DeletableAdminSpec};
    use crate::objects::{
        CreateFrom, DeleteRequest, ListResponse, ObjectFrom, ObjectTryFrom, WatchRequest,
    };
    use crate::{
        AdminSpec, NameFilter,
        objects::{ListRequest, Metadata, WatchResponse},
    };
    use super::TableFormatSpec;

    impl AdminSpec for TableFormatSpec {
        type ListFilter = NameFilter;
        type ListType = Metadata<Self>;
        type WatchResponseType = Self;
    }

    impl CreatableAdminSpec for TableFormatSpec {
        const CREATE_TYPE: u8 = 5;
    }

    impl DeletableAdminSpec for TableFormatSpec {
        type DeleteKey = String;
    }

    CreateFrom!(TableFormatSpec, TableFormat);
    ObjectFrom!(WatchRequest, TableFormat);
    ObjectFrom!(WatchResponse, TableFormat);
    ObjectFrom!(ListRequest, TableFormat);
    ObjectFrom!(ListResponse, TableFormat);
    ObjectFrom!(DeleteRequest, TableFormat);

    ObjectTryFrom!(WatchResponse, TableFormat);
    ObjectTryFrom!(ListResponse, TableFormat);
}

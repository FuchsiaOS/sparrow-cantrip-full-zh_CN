// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Cantrip OS process management support

#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use cantrip_memory_interface::ObjDescBundle;
use cantrip_os_common::camkes::Camkes;
use cantrip_os_common::sel4_sys::seL4_CPtr;
use cantrip_security_interface::SecurityRequestError;
use core::str;
use log::trace;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod bundle_image;
pub use bundle_image::*;

const REQUEST_DATA_SIZE: usize = 128;

pub type BundleIdArray = Vec<String>;

// Size of the data buffer used to pass a serialized BundleIdArray between Rust <> C.
// The data structure size is bounded by the camkes ipc buffer (120 bytes!)
// and also by it being allocated on the stack of the rpc glue code.
// So we need to balance these against being able to return all values.
pub const RAW_BUNDLE_ID_DATA_SIZE: usize = 100;
pub type RawBundleIdData = [u8; RAW_BUNDLE_ID_DATA_SIZE];

// BundleId capacity before spillover to the heap.
// TODO(sleffler): hide this; it's part of the implementation
pub const DEFAULT_BUNDLE_ID_CAPACITY: usize = 64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bundle {
    // NB: application & ML binaries use well-known paths relative to bundle_id
    // NB: ProcessManager owns loaded application's memory

    // Bundle id extracted from manifest
    pub app_id: String,

    // Size (bytes) of loaded application
    pub app_memory_size: u32,
}
impl Bundle {
    pub fn new(bundle_id: &str) -> Self {
        Bundle {
            app_id: String::from(bundle_id),
            app_memory_size: 0u32,
        }
    }
}

// Interface to underlying Bundle implementations. Mainly
// used to inject fakes for unit tests.
pub trait BundleImplInterface {
    fn start(&mut self) -> Result<(), ProcessManagerError>;
    fn stop(&mut self) -> Result<(), ProcessManagerError>;
    fn suspend(&self) -> Result<(), ProcessManagerError>;
    fn resume(&self) -> Result<(), ProcessManagerError>;
    fn capscan(&self) -> Result<(), ProcessManagerError>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProcessManagerError {
    Success = 0,
    BundleIdInvalid,
    PackageBufferLenInvalid,
    BundleNotFound,
    BundleFound,
    BundleRunning,
    BundleNotRunning,
    UnknownError,
    DeserializeError,
    SerializeError,
    ObjCapInvalid,
    // Generic errors, mostly for unit tests.
    InstallFailed,
    UninstallFailed,
    StartFailed,
    StopFailed,
    // TODO(sleffler): for use if/when ProcessManagerInterface grows
    SuspendFailed,
    ResumeFailed,
    CapScanFailed,
}

// Interface to underlying facilities (StorageManager, seL4); also
// used to inject fakes for unit tests.
pub trait ProcessManagerInterface {
    type BundleImpl: BundleImplInterface;

    fn install(&mut self, pkg_contents: &ObjDescBundle) -> Result<String, ProcessManagerError>;
    fn install_app(
        &mut self,
        app_id: &str,
        pkg_contents: &ObjDescBundle,
    ) -> Result<(), ProcessManagerError>;
    fn uninstall(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError>;
    fn start(&mut self, bundle: &Bundle) -> Result<Self::BundleImpl, ProcessManagerError>;
    fn stop(&mut self, bundle_impl: &mut Self::BundleImpl) -> Result<(), ProcessManagerError>;
    fn capscan(&self, bundle_impl: &Self::BundleImpl) -> Result<(), ProcessManagerError>;
}

pub trait PackageManagementInterface {
    fn install(&mut self, pkg_contents: &ObjDescBundle) -> Result<String, ProcessManagerError>;
    fn install_app(
        &mut self,
        app_id: &str,
        pkg_contents: &ObjDescBundle,
    ) -> Result<(), ProcessManagerError>;
    fn uninstall(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError>;
}

pub trait ProcessControlInterface {
    fn start(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError>;
    fn stop(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError>;
    fn get_running_bundles(&self) -> Result<BundleIdArray, ProcessManagerError>;
    fn capscan(&self, bundle_id: &str) -> Result<(), ProcessManagerError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageManagementRequest<'a> {
    Install(Cow<'a, ObjDescBundle>), // Install package (returns bundle_id)
    InstallApp {
        // Install application
        app_id: &'a str,
        pkg_contents: Cow<'a, ObjDescBundle>,
    },
    Uninstall(&'a str), // Uninstall package
}
impl<'a> PackageManagementRequest<'a> {
    fn get_container_cap(&self) -> Option<seL4_CPtr> {
        match self {
            PackageManagementRequest::Install(pkg_contents)
            | PackageManagementRequest::InstallApp {
                app_id: _,
                pkg_contents,
            } => Some(pkg_contents.cnode),
            PackageManagementRequest::Uninstall(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallResponse {
    pub bundle_id: String,
}

#[inline]
fn cantrip_pkg_mgmt_request_aux(
    cap: Option<seL4_CPtr>,
    request_buffer: &[u8],
    reply_buffer: &mut RawBundleIdData,
) -> Result<(), ProcessManagerError> {
    extern "C" {
        pub fn pkg_mgmt_request(
            c_request_buffer_len: u32,
            c_request_buffer: *const u8,
            c_reply_buffer: *mut RawBundleIdData,
        ) -> ProcessManagerError;
    }
    if let Some(cap) = cap {
        let _cleanup = Camkes::set_request_cap(cap);
        unsafe {
            pkg_mgmt_request(
                request_buffer.len() as u32,
                request_buffer.as_ptr(),
                reply_buffer as *mut _,
            )
            .into()
        }
    } else {
        // NB: guard against a received badge being treated as an
        // outbound capability. This is needed because the code CAmkES
        // generates for pkg_mgmt_request always enables possible xmit
        // of 1 capability.
        Camkes::clear_request_cap();
        unsafe {
            pkg_mgmt_request(
                request_buffer.len() as u32,
                request_buffer.as_ptr(),
                reply_buffer as *mut _,
            )
            .into()
        }
    }
}

#[inline]
fn cantrip_pkg_mgmt_request<T: DeserializeOwned>(
    request: &PackageManagementRequest,
) -> Result<T, ProcessManagerError> {
    trace!(
        "cantrip_pkg_mgmt_request {:?} cap {:?}",
        &request,
        request.get_container_cap()
    );
    let mut request_buffer = [0u8; REQUEST_DATA_SIZE];
    let request_slice = postcard::to_slice(request, &mut request_buffer)
        .or(Err(ProcessManagerError::SerializeError))?;
    let mut reply_buffer = [0u8; RAW_BUNDLE_ID_DATA_SIZE];
    cantrip_pkg_mgmt_request_aux(request.get_container_cap(), request_slice, &mut reply_buffer)?;
    postcard::from_bytes(&reply_buffer).or(Err(ProcessManagerError::DeserializeError))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessControlRequest<'a> {
    Start(&'a str),
    Stop(&'a str),
    GetRunningBundles, // -> bundle_ids

    CapScan,
    CapScanBundle(&'a str),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRunningBundlesResponse {
    pub bundle_ids: BundleIdArray,
}

#[inline]
fn cantrip_proc_ctrl_request<T: DeserializeOwned>(
    request: &ProcessControlRequest,
) -> Result<T, ProcessManagerError> {
    extern "C" {
        pub fn proc_ctrl_request(
            c_request_buffer_len: u32,
            c_request_buffer: *const u8,
            c_reply_buffer: *mut RawBundleIdData,
        ) -> ProcessManagerError;
    }
    trace!("cantrip_proc_ctrl_request {:?}", &request,);
    let mut request_buffer = [0u8; REQUEST_DATA_SIZE];
    let request_slice = postcard::to_slice(request, &mut request_buffer)
        .or(Err(ProcessManagerError::SerializeError))?;
    let mut reply_buffer = [0u8; RAW_BUNDLE_ID_DATA_SIZE];
    // NB: guard against a received badge being treated as an
    // outbound capability. This is needed because the code CAmkES
    // generates for pkg_mgmt_request always enables possible xmit
    // of 1 capability.
    Camkes::clear_request_cap();
    match unsafe {
        proc_ctrl_request(
            request_slice.len() as u32,
            request_slice.as_ptr(),
            &mut reply_buffer as *mut _,
        )
    } {
        ProcessManagerError::Success => {
            postcard::from_bytes(&reply_buffer).or(Err(ProcessManagerError::DeserializeError))
        }
        err => Err(err),
    }
}

impl From<SecurityRequestError> for ProcessManagerError {
    fn from(err: SecurityRequestError) -> ProcessManagerError {
        match err {
            SecurityRequestError::SreSuccess => ProcessManagerError::Success,
            SecurityRequestError::SreBundleIdInvalid => ProcessManagerError::BundleIdInvalid,
            SecurityRequestError::SreBundleNotFound => ProcessManagerError::BundleNotFound,
            SecurityRequestError::SrePackageBufferLenInvalid => {
                ProcessManagerError::PackageBufferLenInvalid
            }
            SecurityRequestError::SreInstallFailed => ProcessManagerError::InstallFailed,
            SecurityRequestError::SreUninstallFailed => ProcessManagerError::UninstallFailed,
            // NB: other errors "cannot happen" so just return something unique
            _ => ProcessManagerError::UnknownError,
        }
    }
}

impl From<cstr_core::NulError> for ProcessManagerError {
    fn from(_err: cstr_core::NulError) -> ProcessManagerError {
        ProcessManagerError::BundleIdInvalid
    }
}

impl From<ProcessManagerError> for Result<(), ProcessManagerError> {
    fn from(err: ProcessManagerError) -> Result<(), ProcessManagerError> {
        if err == ProcessManagerError::Success {
            Ok(())
        } else {
            Err(err)
        }
    }
}

#[inline]
pub fn cantrip_proc_ctrl_get_running_bundles() -> Result<BundleIdArray, ProcessManagerError> {
    cantrip_proc_ctrl_request(&ProcessControlRequest::GetRunningBundles)
        .map(|reply: GetRunningBundlesResponse| reply.bundle_ids)
}

#[inline]
pub fn cantrip_pkg_mgmt_install(
    pkg_contents: &ObjDescBundle,
) -> Result<String, ProcessManagerError> {
    Camkes::debug_assert_slot_cnode(
        "cantrip_pkg_mgmt_install",
        &Camkes::top_level_path(pkg_contents.cnode),
    );
    cantrip_pkg_mgmt_request(&PackageManagementRequest::Install(Cow::Borrowed(pkg_contents)))
        .map(|reply: InstallResponse| reply.bundle_id)
}

#[inline]
pub fn cantrip_pkg_mgmt_install_app(
    app_id: &str,
    pkg_contents: &ObjDescBundle,
) -> Result<(), ProcessManagerError> {
    Camkes::debug_assert_slot_cnode(
        "cantrip_pkg_mgmt_install_app",
        &Camkes::top_level_path(pkg_contents.cnode),
    );
    cantrip_pkg_mgmt_request(&PackageManagementRequest::InstallApp {
        app_id,
        pkg_contents: Cow::Borrowed(pkg_contents),
    })
}

#[inline]
pub fn cantrip_pkg_mgmt_uninstall(bundle_id: &str) -> Result<(), ProcessManagerError> {
    cantrip_pkg_mgmt_request(&PackageManagementRequest::Uninstall(bundle_id))
}

#[inline]
pub fn cantrip_proc_ctrl_start(bundle_id: &str) -> Result<(), ProcessManagerError> {
    cantrip_proc_ctrl_request(&ProcessControlRequest::Start(bundle_id))
}

#[inline]
pub fn cantrip_proc_ctrl_stop(bundle_id: &str) -> Result<(), ProcessManagerError> {
    cantrip_proc_ctrl_request(&ProcessControlRequest::Stop(bundle_id))
}

#[inline]
pub fn cantrip_proc_ctrl_capscan() -> Result<(), ProcessManagerError> {
    cantrip_proc_ctrl_request(&ProcessControlRequest::CapScan)
}

#[inline]
pub fn cantrip_proc_ctrl_capscan_bundle(bundle_id: &str) -> Result<(), ProcessManagerError> {
    cantrip_proc_ctrl_request(&ProcessControlRequest::CapScanBundle(bundle_id))
}

// TODO(sleffler): move out of interface?
#[cfg(test)]
mod tests {
    use super::*;
    use postcard;

    #[test]
    fn test_raw_bundle_id_data_empty() {
        let bid_array = BundleIdArray::new();
        // Marhshall/unmarshall empty bid_array.
        let mut raw_data = [0u8; RAW_BUNDLE_ID_DATA_SIZE];
        assert!(postcard::to_slice(&bid_array, &mut raw_data).is_ok());
        assert_eq!(
            postcard::from_bytes::<BundleIdArray>(raw_data.as_ref()).unwrap(),
            bid_array
        );
    }

    #[test]
    fn test_raw_bundle_id_data_simple() {
        let mut bid_array = BundleIdArray::new();
        bid_array.push(String::from("zero"));
        bid_array.push(String::from("one"));
        bid_array.push(String::from("two"));

        // Marhshall/unmarshall bid_array.
        let mut raw_data = [0u8; RAW_BUNDLE_ID_DATA_SIZE];
        assert!(postcard::to_slice(&bid_array, &mut raw_data).is_ok());
        assert_eq!(
            postcard::from_bytes::<BundleIdArray>(raw_data.as_ref()).unwrap(),
            bid_array
        );
    }

    #[test]
    fn test_raw_bundle_id_data_out_of_space() {
        // Marshall an array with >255 id's; serialize will fail because
        // there's not enough space.
        let mut bid_array = BundleIdArray::new();
        for bid in 0..256 {
            bid_array.push(bid.to_string());
        }
        let mut raw_data = [0u8; RAW_BUNDLE_ID_DATA_SIZE];
        assert!(postcard::to_slice(&bid_array, &mut raw_data).is_err());
    }

    #[test]
    fn test_raw_bundle_id_data_too_long() {
        // Marshall an id with length >255; serialize will fail because
        // there's not enough space.
        let mut bid_array = BundleIdArray::new();
        bid_array.push("0123456789".repeat(26));
        let mut raw_data = [0u8; RAW_BUNDLE_ID_DATA_SIZE];
        assert!(postcard::to_slice(&bid_array, &mut raw_data).is_err());
    }
}

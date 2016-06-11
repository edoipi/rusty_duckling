// Copyright © 2016, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to runtimeobject.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn GetRestrictedErrorInfo();
    // pub fn HSTRING_UserFree();
    // pub fn HSTRING_UserMarshal();
    // pub fn HSTRING_UserSize();
    // pub fn HSTRING_UserUnmarshal();
    // pub fn IsErrorPropagationEnabled();
    // pub fn RoActivateInstance();
    // pub fn RoCaptureErrorContext();
    // pub fn RoClearError();
    // pub fn RoFailFastWithErrorContext();
    // pub fn RoFreeParameterizedTypeExtra();
    // pub fn RoGetActivatableClassRegistration();
    // pub fn RoGetActivationFactory();
    // pub fn RoGetApartmentIdentifier();
    // pub fn RoGetBufferMarshaler();
    // pub fn RoGetErrorReportingFlags();
    // pub fn RoGetMatchingRestrictedErrorInfo();
    // pub fn RoGetMetaDataFile();
    // pub fn RoGetParameterizedTypeInstanceIID();
    // pub fn RoGetServerActivatableClasses();
    // pub fn RoInitialize();
    // pub fn RoInspectCapturedStackBackTrace();
    // pub fn RoInspectThreadErrorInfo();
    // pub fn RoOriginateError();
    // pub fn RoOriginateErrorW();
    // pub fn RoOriginateLanguageException();
    // pub fn RoParameterizedTypeExtraGetTypeSignature();
    // pub fn RoParseTypeName();
    // pub fn RoRegisterActivationFactories();
    // pub fn RoRegisterForApartmentShutdown();
    // pub fn RoReportFailedDelegate();
    // pub fn RoReportUnhandledError();
    // pub fn RoResolveNamespace();
    // pub fn RoResolveRestrictedErrorInfoReference();
    // pub fn RoRevokeActivationFactories();
    // pub fn RoSetErrorReportingFlags();
    // pub fn RoTransformError();
    // pub fn RoTransformErrorW();
    // pub fn RoUninitialize();
    // pub fn RoUnregisterForApartmentShutdown();
    // pub fn SetRestrictedErrorInfo();
    // pub fn WindowsCompareStringOrdinal();
    // pub fn WindowsConcatString();
    // pub fn WindowsCreateString();
    // pub fn WindowsCreateStringReference();
    // pub fn WindowsDeleteString();
    // pub fn WindowsDeleteStringBuffer();
    // pub fn WindowsDuplicateString();
    // pub fn WindowsGetStringLen();
    // pub fn WindowsGetStringRawBuffer();
    // pub fn WindowsInspectString();
    // pub fn WindowsIsStringEmpty();
    // pub fn WindowsPreallocateStringBuffer();
    // pub fn WindowsPromoteStringBuffer();
    // pub fn WindowsReplaceString();
    // pub fn WindowsStringHasEmbeddedNull();
    // pub fn WindowsSubstring();
    // pub fn WindowsSubstringWithSpecifiedLength();
    // pub fn WindowsTrimStringEnd();
    // pub fn WindowsTrimStringStart();
}
#[cfg(target_arch = "x86_64")]
extern "system" {
    // pub fn HSTRING_UserFree64();
    // pub fn HSTRING_UserMarshal64();
    // pub fn HSTRING_UserSize64();
    // pub fn HSTRING_UserUnmarshal64();
}

use crate::{
    error::Error,
    functions::IL2CPP_SO,
    types::{Il2CppClass, MethodInfo},
};
use std::{ffi::CString, slice};

impl Il2CppClass {
    pub fn find<'a>(namespace: &str, name: &str) -> Result<&'a mut Self, Error> {
        let domain = unsafe { IL2CPP_SO.il2cpp_domain_get() };
        if domain.is_null() {
            return Err(Error::NullPointer("could not get domain".to_owned()));
        }

        let mut assembly_count = 0;
        let all_assemblies =
            unsafe { IL2CPP_SO.il2cpp_domain_get_assemblies(domain, &mut assembly_count) };
        let all_assemblies = unsafe { slice::from_raw_parts(all_assemblies, assembly_count) };

        for assembly in all_assemblies {
            if assembly.is_null() {
                return Err(Error::NullPointer(
                    "could not get all assemblies".to_owned(),
                ));
            }

            let image = unsafe { IL2CPP_SO.il2cpp_assembly_get_image(*assembly) };
            if image.is_null() {
                return Err(Error::NullPointer("assembly has a null image".to_owned()));
            }

            let class = unsafe {
                IL2CPP_SO.il2cpp_class_from_name(
                    image,
                    CString::new(namespace)?.as_ptr(),
                    CString::new(name)?.as_ptr(),
                )
            };
            if !class.is_null() {
                return Ok(unsafe { &mut *class });
            }
        }
        Err(Error::NullPointer(format!(
            "couldn't find class {}.{}",
            namespace, name,
        )))
    }

    pub fn get_method<'a>(
        &mut self,
        name: &str,
        argument_count: i32,
    ) -> Result<&'a MethodInfo, Error> {
        let method_info = unsafe {
            IL2CPP_SO.il2cpp_class_get_method_from_name(
                self,
                CString::new(name)?.as_ptr(),
                argument_count,
            )
        };
        if method_info.is_null() {
            return Err(Error::NullPointer(format!(
                "couldn't find method {:?} with {} parameters",
                name, argument_count,
            )));
        }
        Ok(unsafe { &*method_info })
    }
}


extern crate winapi;
extern crate psapi;
extern crate kernel32;

use winapi::minwindef::{DWORD, HMODULE, LPDWORD, BOOL};
use winapi::winnt::{HANDLE, LPWSTR, WCHAR, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use kernel32::{K32EnumProcesses, K32EnumProcessModules, CloseHandle, K32GetModuleBaseNameW, OpenProcess};

struct Process {
    a_procs: [DWORD; 1024],
    len: usize
}

impl Process {

    pub fn new() -> Result<Process, ()> {

        let mut pe = Process {
            a_procs: unsafe { std::mem::uninitialized() },
            len: 0
        };

        let mut cb_needed: DWORD = 0;

        let a_procs_ref: [usize; 2] = unsafe { std::mem::transmute(pe.a_procs.as_mut()) };
        let a_procs_ptr = a_procs_ref[0] as *mut DWORD;
        let cb_needed_ptr = &mut cb_needed as *mut u32;

        unsafe {
            if K32EnumProcesses(a_procs_ptr, pe.a_procs.len() as u32, cb_needed_ptr) != -1 {
                pe.len = (cb_needed / std::mem::size_of::<DWORD>() as DWORD) as usize;
                Ok(pe)
            } else {
                Err(())
            }
        }

    }

    fn get_process_name(&self, procId: DWORD) -> Result<String, ()> {

        let mut ret_val = Err(());
        let h_proc: HANDLE = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0 as BOOL, procId) };

        if !h_proc.is_null() {

            let mut h_mod: HMODULE = unsafe { std::mem::uninitialized() };
            let mut cb_needed: DWORD = 0;
            let mut name: [WCHAR; 1024] = unsafe { std::mem::uninitialized() };

            unsafe {
                if K32EnumProcessModules(h_proc, &mut h_mod as *mut HMODULE, std::mem::size_of::<HMODULE>() as DWORD, &mut cb_needed as LPDWORD) != -1 {
                    let mut name_ref: [usize; 2] = unsafe { std::mem::transmute(name.as_mut()) };
                    let name_ptr = name_ref[0] as LPWSTR;
                    K32GetModuleBaseNameW(h_proc, h_mod, name_ptr, 1024);

                    //unsafe { ret_val = Ok(std::ffi::CString::from_raw(std::mem::transmute(&mut name)).into_string().unwrap()) }
                    ret_val = Ok("Name".to_string());
                }

                CloseHandle(h_proc);
            }
        } 
        ret_val
    }
}

struct ProcessIter<'a> {
    inner: &'a mut Process,
    ind: usize
}

impl<'a> IntoIterator for &'a mut Process {
    type Item = String;
    type IntoIter = ProcessIter<'a>;

    fn into_iter(self) -> ProcessIter<'a> {
        ProcessIter {
            inner: self,
            ind: 0
        }
    }
}

impl<'a> Iterator for ProcessIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ind < self.inner.len {
            self.ind += 1;
            let process_id = self.inner.a_procs[self.ind];
            let process_name = self.inner.get_process_name(process_id).unwrap();
            Some(format!("Name: {} PID: {}", process_name, process_id))
        } else {
            None
        }
    }
}

fn main() {

    let mut pe = match Process::new() {
        Ok(ref mut pe) => {
            for p in pe.into_iter() {
                println!("{}", p);
            }
        }
        Err(_) => panic!("Error")
    };

}

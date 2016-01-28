
extern crate winapi;
extern crate psapi;
extern crate kernel32;

use kernel32::{CreateToolhelp32Snapshot, Process32First, Process32Next, CloseHandle};
use winapi::minwindef::{DWORD, MAX_PATH};
use winapi::tlhelp32::{TH32CS_SNAPPROCESS, PROCESSENTRY32, LPPROCESSENTRY32};
use winapi::shlobj::{INVALID_HANDLE_VALUE};
use winapi::winnt::{HANDLE};

struct ProcessesSnapshot {
    inner: Option<HANDLE>
}

impl ProcessesSnapshot {

    pub fn new() -> Result<ProcessesSnapshot, ()> {
        let h_processes_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if h_processes_snap == INVALID_HANDLE_VALUE {
            return Err(());
        }

        Ok(ProcessesSnapshot {
            inner: Some(h_processes_snap)
        })
    }

    //fn get_process_name(&self, procId: DWORD) -> Result<String, ()> {

        //let mut ret_val = Err(());
        //let h_proc: HANDLE = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0 as BOOL, procId) };

        //if !h_proc.is_null() {

            //let mut h_mod: HMODULE = unsafe { std::mem::uninitialized() };
            //let mut cb_needed: DWORD = 0;
            //let mut name: [WCHAR; 1024] = unsafe { std::mem::uninitialized() };

            //unsafe {
                //if K32EnumProcessModules(h_proc, &mut h_mod as *mut HMODULE, std::mem::size_of::<HMODULE>() as DWORD, &mut cb_needed as LPDWORD) != -1 {
                    //let mut name_ref: [usize; 2] = unsafe { std::mem::transmute(name.as_mut()) };
                    //let name_ptr = name_ref[0] as LPWSTR;
                    //K32GetModuleBaseNameW(h_proc, h_mod, name_ptr, 1024);

                    ////unsafe { ret_val = Ok(std::ffi::CString::from_raw(std::mem::transmute(&mut name)).into_string().unwrap()) }
                    //ret_val = Ok("Name".to_string());
                //}

                //CloseHandle(h_proc);
            //}
        //}
        //ret_val
    //}
}

fn new_process_entry() -> PROCESSENTRY32 {
    PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as DWORD,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [' ' as i8; MAX_PATH],
    }
}

struct ProcessIter {
    snapshot: ProcessesSnapshot,
    pe32: PROCESSENTRY32
}

impl IntoIterator for ProcessesSnapshot {
    type Item = String;
    type IntoIter = ProcessIter;

    fn into_iter(self) -> ProcessIter {

        let mut pe32 = new_process_entry();

        unsafe {
            if let Some(s) = self.inner {
                if Process32First(s, &mut pe32 as LPPROCESSENTRY32) == 0 {
                    CloseHandle(s);
                }
            }
        }

        ProcessIter {
            snapshot: self,
            pe32: pe32
        }

    }
}

impl Iterator for ProcessIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {

        use std::ffi::*;
        let string = unsafe { CString::from_raw(self.pe32.szExeFile[..].as_mut_ptr()) };
        let ret = String::from_utf8_lossy(string.as_bytes());

        if let Some(snap) = self.snapshot.inner {
            unsafe {
                if Process32Next(snap, &mut self.pe32 as LPPROCESSENTRY32) != 0 {
                    Some(ret.into_owned())
                } else {
                    None
                }
            }
        } else { None }
    }
}

fn main() {

    match ProcessesSnapshot::new() {
        Ok(pe) => {
            for p in pe.into_iter() {
                println!("{}", p);
            }
        }
        Err(_) => panic!("Error")
    };

}

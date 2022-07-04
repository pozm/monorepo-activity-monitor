#![feature(cstr_from_bytes_until_nul)]
use std::{ffi::{CString,CStr}, collections::HashMap};
use chrono::NaiveDateTime;
use windows::Win32::{System::{Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, MODULEENTRY32, Module32First, PROCESSENTRY32, Process32First, Module32Next, Process32Next}, Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, GetProcessTimes}, ProcessStatus::{K32GetProcessImageFileNameA, K32GetModuleFileNameExA}, LibraryLoader::GetModuleFileNameA, Time::FileTimeToSystemTime}, Foundation::{HANDLE, CHAR, HINSTANCE, CloseHandle, FILETIME, SYSTEMTIME}, Storage::FileSystem::{GetFinalPathNameByHandleA, FILE_NAME_NORMALIZED}, self};
use windows::{core::*};
pub fn is_any_running(exe_path:&Vec<String>,seen_paths:&mut Vec<(String,NaiveDateTime)>) -> Option<()> {
    let mut handles: Vec<(String,NaiveDateTime)> = vec![];
    let mut check_alive = HashMap::new();
    for s in seen_paths.clone() {
        check_alive.insert(s.0, ());
    }
    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0); // all procceses
        if handle.is_err() {
            return None;
        }
        let handle = handle.unwrap();
        let mut mod32 = PROCESSENTRY32::default();

        mod32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if !Process32First(handle, &mut mod32).as_bool() {
            return None;
        }

        let get_as_string = |x : &[CHAR]| CStr::from_bytes_until_nul(&x.iter().map(|c| c.0).collect::<Vec<u8>>()).unwrap().to_str().unwrap().to_string();
        let get_as_string_u8 = |x : &[u8]| CStr::from_bytes_until_nul(&x).unwrap().to_str().unwrap().to_string();
        let mut are_we_chillin = |x:&str,mod32:&mut PROCESSENTRY32,handles:&mut Vec<(String,NaiveDateTime)>| {
            if !exe_path.iter().any(|b|b.ends_with(x)) {
                return None;
            }
            else {
                let h = OpenProcess(PROCESS_QUERY_INFORMATION, false, mod32.th32ProcessID);
                if h.is_err() {
                    return None
                }
                let h = h.unwrap();
                let mut img_name = vec!['\0' as u8;260];
                K32GetModuleFileNameExA(h,HINSTANCE::default(), &mut img_name);
                // println!("{}", get_as_string_u8(&img_name));
                let actual_name = get_as_string_u8(&img_name);
                if handles.iter().any(|x|(*x).0 == actual_name) || seen_paths.iter().any(|x|(*x).0 == actual_name)  {
                    CloseHandle(h);
                    check_alive.remove(&actual_name);
                    return None;
                }
                if !exe_path.contains(&actual_name)  {
                    CloseHandle(h);
                    return None;
                }
                else {
                    let sys_time = get_system_time_from_process(&h);
                    CloseHandle(h);
                    return Some((actual_name,sys_time));  
                } 
            }
        };

        if let Some(pogging) = are_we_chillin(&get_as_string(&mod32.szExeFile),&mut mod32,&mut handles) {
            let h = OpenProcess(PROCESS_QUERY_INFORMATION, false, mod32.th32ProcessID).unwrap();
            handles.push(pogging);
        }

        while Process32Next(handle, &mut mod32).as_bool() {
            let pogs = &get_as_string(&mod32.szExeFile);
            // println!("! {:?}", pogs);
            if let Some(pogging) = are_we_chillin(pogs,&mut mod32,&mut handles) {
                handles.push(pogging);
            }
        }



        // if exe_path.iter().any(|x| compare_handle_str(&handle, x)) {
        //     return true;
        // }
        
        
    }
    seen_paths.append(&mut handles);
    for s in check_alive {
        let p = seen_paths.iter().position(|x| x.0 == s.0).unwrap();
        // unsafe {CloseHandle(seen_paths[p].0);}
        seen_paths.swap_remove(p);
    }
    Some(())
}

fn sys_time_to_rust(sys_time:&SYSTEMTIME) -> NaiveDateTime {
    let t = format!("{}-{}-{:0>2} {}:{}:{}",
        sys_time.wYear,
        sys_time.wMonth,
        sys_time.wDay,
        sys_time.wHour,
        sys_time.wMinute,
        sys_time.wSecond,
    );
    // println!("{}", t);
    chrono::NaiveDateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S").unwrap()
  }

unsafe fn get_system_time_from_process(h:&HANDLE) -> NaiveDateTime {
    let mut file_time = FILETIME::default();
    GetProcessTimes(h,&mut file_time,&mut FILETIME::default(),&mut FILETIME::default(),&mut FILETIME::default());
    let mut sys_time = SYSTEMTIME::default();
    FileTimeToSystemTime(&mut file_time, &mut sys_time);
    sys_time_to_rust(&sys_time)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SysTimeWrapper(SYSTEMTIME);

// unsafe fn compare_handle_str(h :&, str:&str) -> bool {
//     let mut houtput : Vec<u8> = vec![];
//     GetFinalPathNameByHandleA(h,&mut houtput, FILE_NAME_NORMALIZED);
//     str.as_bytes().eq(&houtput)
// }
use windows::{
    core::*,
    Win32::{Foundation::GetLastError, System::Threading::*, UI::WindowsAndMessaging::*},
};
fn main() {
    unsafe {
        let si: STARTUPINFOW = Default::default();
        let mut pi: PROCESS_INFORMATION = Default::default();
        let result = MessageBoxW(
            None,
            w!("This is just some random text"),
            w!("Message Box"),
            MB_YESNOCANCEL | MB_ICONEXCLAMATION,
        );
        if result == IDYES {
            if (!CreateProcessW(
                w!("C:\\Windows\\System32\\notepad.exe"),
                PWSTR::null(),
                None,
                None,
                false,
                BELOW_NORMAL_PRIORITY_CLASS,
                None,
                None,
                &si,
                &mut pi,
            ))
            .as_bool()
            {
                println!("(-) Failed to create process: {:?}", GetLastError());
            } else {
                println!("(+) process started! pid: {}", pi.dwProcessId)
            }
        } else {
            println!("(-) Wrong id! Id: {:?}", result)
        }
    }
}

use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};

fn main() {
    unsafe {
        MessageBoxA(None, "Hello World!", "My Title", MB_OK);
    }
}

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

pub unsafe extern "system" fn main_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_DESTROY {
        PostQuitMessage(0);
        return LRESULT(0);
    }
    return DefWindowProcA(hwnd, msg, wparam, lparam);
}

fn main() -> Result<()> {
    unsafe {
        let wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
            lpfnWndProc: Some(main_wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleA(None)?,
            hIcon: HICON(0),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: HBRUSH(6), // COLOR_WINDOW + 1
            lpszMenuName: PCSTR(b"\0".as_ptr()),
            lpszClassName: PCSTR(b"MainWindow\0".as_ptr()),
            hIconSm: HICON(0),
        };
        if RegisterClassExA(&wc) == 0 {
            panic!("Error: couldn't register window class");
        }

        let hwnd_main = CreateWindowExA(
            WINDOW_EX_STYLE(0),
            wc.lpszClassName,
            "Simple Win32 Window",
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            600,
            400,
            HWND(0),
            HMENU(0),
            wc.hInstance,
            std::ptr::null_mut(),
        );
        if hwnd_main == HWND(0) {
            panic!("Error: couldn't create window");
        }

        ShowWindow(hwnd_main, SW_SHOW);

        let mut msg = MSG {
            hwnd: HWND(0),
            message: 0,
            wParam: WPARAM(0),
            lParam: LPARAM(0),
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        loop {
            if GetMessageA(&mut msg, HWND(0), 0, 0) == BOOL(0) {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
    Ok(())
}

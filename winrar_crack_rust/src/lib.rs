/*
 _   __ _____ _____  _    _ _   _
| | / /|  ___|  _  || |  | | | | |
| |/ / | |__ | | | || |  | | | | |
|    \ |  __|| | | || |/\| | | | |
| |\  \| |___\ \_/ /\  /\  / |_| |
\_| \_/\____/ \___/  \/  \/ \___/
                            2022

Copyright (c) Fluxuss Cyber Tech Desenvolvimento de Software, SLU (FLUXUSS)
 */
mod winrar_crack;
use winapi::um::{ consoleapi::AllocConsole };

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain ( _module: winapi::shared::minwindef::HINSTANCE, _reason: winapi::shared::minwindef::DWORD, _ms_reserved: winapi::shared::minwindef::LPVOID ) -> winapi::shared::minwindef::BOOL {

    match _reason {

        1 => {

            //msdn AllocConsole -> Para recuperar uma instância de console do prompt
            unsafe { AllocConsole(); }

            //C++ eu te amo, mais Rust tá me conquistando <3
            std::thread::spawn(|| { //O Poder das Closures do Rust :D

                loop {

                    let mut _winrarCrack = winrar_crack::WinrarCrack::new();

                    println!("[!] ModuleBase {:#06x}", _winrarCrack.get_current_process_base());

                    _winrarCrack.preparete_patch();

                    break;

                };

            });

        }, // Process Attach
        0 => (), // Process Detach
        _ => (), // Rasão de chamada desconhecida pela DLL

    }

    winapi::shared::minwindef::TRUE
}
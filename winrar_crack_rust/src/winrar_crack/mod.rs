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
use winapi::shared::minwindef::{DWORD, LPVOID, PDWORD};
use winapi::um::{libloaderapi::GetModuleHandleA, memoryapi::VirtualProtect, winnt::RtlCopyMemory};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};


pub struct FluDisasm {
    
    context_rip: u64,
    mem_buf: [u8; 100],

}

impl FluDisasm {
    
    fn new( context_rip: u64, mem_buf: [ u8; 100 ] ) -> Self {

        FluDisasm { 

            context_rip,
            mem_buf,

        }

    }

    fn disasm_init( &self ) {

        let mut decoder = Decoder::with_ip( 64, &self.mem_buf, self.context_rip, DecoderOptions::NONE );

        let mut formatter = NasmFormatter::new( );

        formatter.options_mut().set_digit_separator( "`" );

        formatter.options_mut().set_first_operand_char_index( 10 );

        let mut output = String::new( );

        let mut r = Instruction::default( );

        while decoder.can_decode( ) {

            decoder.decode_out( &mut r );

            output.clear( );

            formatter.format( &r, &mut output );

            print!( "RIP {:016X}", r.ip( ) );

            let init = ( r.ip( ) - self.context_rip ) as usize;

            let opcodes_out = &self.mem_buf[ init .. init + r.len( ) ];

            for i in &*opcodes_out {

                print!( "{:02X}", *i );

            }

            if opcodes_out.len( ) < 10 {
                for _ in 0..10 - opcodes_out.len( ) {
                    print!( " " );
                }
            }

            println!( " {}", output );


        }

    }

}


#[derive(FromPrimitive)]
enum WinrarCrackVals {

    OpTest = 0x84,
    OpNop = 0x90,
    PageExecuteReadwrite = 0x40, //PAGE_EXECUTE_READ_WRITE

}

pub struct WinrarCrack{

    module_base: u64,
    relative_offset: u64,

}

impl WinrarCrack {

    /*
        Recuperando o module base do processo do Winrar
    */
    fn set_current_process_base( &mut self ) {

        unsafe {

            self.module_base = GetModuleHandleA( 0 as winapi::um::winnt::LPCSTR ) as u64;

        }

    }

    /*
        Get para recuperar o valor da module base
    */
    pub fn get_current_process_base( &mut self ) -> u64 {

        if self.module_base == 0 { self.set_current_process_base( ); } // Caso a imagebase seja zero, necessita-se obter a base do Winrar

        self.module_base

    }

    /*
        Preparando e efetuando o patch no Winrar
    */
    pub fn preparete_patch( &mut self ) {

        let img_base = self.get_current_process_base( ) + self.relative_offset; //Calculando o offset relativo com a imagebase

        dbg!( img_base );

        println!( "[!] Deslocamento calculado: {:#06x}", img_base );

        /*
            Referênciando a paginação de memória e mudando-a para garantir que a paginação garante o direito de escrita
        */
        let old_protect: [ DWORD; 1 ] = [ 0 ]; 

        //Vamos alterar a paginação de memória
        unsafe {

            println!( "[!] Definindo uma nova paginação de memória: {:#06x}", WinrarCrackVals::PageExecuteReadwrite as DWORD );

            VirtualProtect( img_base as LPVOID, 1,  WinrarCrackVals::PageExecuteReadwrite as DWORD, old_protect.as_ptr( ) as PDWORD );

            println!( "[!] Proteção antiga dessa paginação de memória: {:#06x}", old_protect[ 0 ] );

            dbg!( old_protect );
        

        }

        /*
            Init flu disasm
        */
        let opcodes_extracted: [ u8; 100 ] = [ 0x0; 100 ];

        /*
            Efetuando uma leitura e extraindo o opcode presente no offset em questão
        */
        let read_original : [u8; 1] = [ 0x0; 1 ];

        unsafe {

            RtlCopyMemory( opcodes_extracted.as_ptr() as *mut winapi::ctypes::c_void, (img_base - 50) as *mut winapi::ctypes::c_void, 100 );

            RtlCopyMemory( read_original.as_ptr() as *mut winapi::ctypes::c_void, img_base as *mut winapi::ctypes::c_void, 1 );

        }

        let flu = FluDisasm::new( img_base-50, opcodes_extracted );

        dbg!( opcodes_extracted );

        println!( "[!] Byte Lido: {:#06x}", read_original[0] );

        dbg!( read_original );

        flu.disasm_init( );

        /*
            Validando o opcode lido, para garantir que estou fazendo patch no local correto e que a versão do Winrar é a esperada
        */
        match FromPrimitive::from_u8( read_original[0] ) {

            Some( WinrarCrackVals::OpTest ) => {

                println!( "[!] Sucesso, o opcode responsável por verificação da regkey foi encontrado!" );

                /*
                    Movendo o opcode para ignorar o opcode original do Winrar de comparação e retornando a páginação de memória ao normal antes do patch
                */
                unsafe {

                    let op_nop = WinrarCrackVals::OpNop as u8;

                    println!("[!] Nop => {:#06X}", WinrarCrackVals::OpNop as DWORD);

                    RtlCopyMemory(img_base as *mut winapi::ctypes::c_void, op_nop as *mut winapi::ctypes::c_void, 1);

                    VirtualProtect(img_base as *mut winapi::ctypes::c_void, 1, old_protect.as_ptr() as DWORD, old_protect.as_ptr() as PDWORD);

                }

                println!("[!] Pronto, o Winrar agora foi crackeado e pode ser usado sem nenhuma restrição.");

            },
            _ => println!("[!] A Versão do Winrar não bate com a esperada !"), //Pode-se ocorrer da versão não ser compatível então necessita-se garantir.
        }

    }

    /*
        Construtor da impl WinrarCrack
    */
    pub fn new() -> Self {

        Self {

            module_base: 0,
            relative_offset: 0xECD44

        }

    }

}
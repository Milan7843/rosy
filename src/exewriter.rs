use core::num;
use std::fs::File;
use std::hint;
use std::io::prelude::*;

use crate::assembler;
use crate::codegenerator::Argument;
use crate::codegenerator::Instruction;
use crate::codegenerator::Register;
use crate::codegenerator::RegisterType;
use crate::compiler;

pub fn write_exe_file(path: &std::path::PathBuf, machine_code: &mut Vec<u8>, syscalls_to_resolve: &Vec<(String, usize)>) -> std::io::Result<()> {
	let mut file = std::fs::File::create(path)?;

	// Write headers
	write_headers(&mut file, machine_code, syscalls_to_resolve)?;

	Ok(())
}

#[derive(PartialEq)]
pub struct ImportEntry {
	pub dll_name: String,                // Name of the DLL to import from
	pub function_names: Vec<String>,     // Name of the function to import
	pub function_address_rvas: Vec<u32>, // RVA of the function (filled in later)
}

fn write_headers(file: &mut File, machine_code: &mut Vec<u8>, syscalls_to_resolve: &Vec<(String, usize)>) -> std::io::Result<()> {
	let size_of_code: u32 = 0x200;
	let size_of_initialized_data: u32 = 0x800;
	let size_of_uninitialized_data: u32 = 0x00;
	let adress_of_entry_point: u32 = 0x1000;
	let base_of_code: u32 = 0x1000;
	let image_version_major: u16 = 0x00;
	let image_version_minor: u16 = 0x00;
	let size_of_image: u32 = 0x6000;
	let size_of_headers: u32 = 0x400;
	let checksum: u32 = 0x0;

	let mut file_headers: Vec<u8> = Vec::new();

	// DOS Header (64 bytes)
	write_u16(&mut file_headers, 0x5A4D); // 0x00: e_magic = "MZ"
	write_u16(&mut file_headers, 0x0090); // 0x02: e_cblp
	write_u16(&mut file_headers, 0x0003); // 0x04: e_cp
	write_u16(&mut file_headers, 0x0000); // 0x06: e_crlc = 0 (relocations)
	write_u16(&mut file_headers, 0x0004); // 0x08: e_cparhdr = 4 (header size in paragraphs)
	write_u16(&mut file_headers, 0x0000); // 0x0A: e_minalloc = 0
	write_u16(&mut file_headers, 0xFFFF); // 0x0C: e_maxalloc = 0xFFFF
	write_u16(&mut file_headers, 0x0000); // 0x0E: e_ss = 0 (initial SS)
	write_u16(&mut file_headers, 0x00B8); // 0x10: e_sp = 0x00B8 (initial SP)
	write_u16(&mut file_headers, 0x0000); // 0x12: e_csum = 0
	write_u16(&mut file_headers, 0x0000); // 0x14: e_ip = 0
	write_u16(&mut file_headers, 0x0000); // 0x16: e_cs = 0
	write_u16(&mut file_headers, 0x0040); // 0x18: e_lfarlc = 0x40 (relocation table offset)
	write_u16(&mut file_headers, 0x0000); // 0x1A: e_ovno = 0
	// e_res (8 bytes)
	write_u16(&mut file_headers, 0x0000); // 0x1C
	write_u16(&mut file_headers, 0x0000); // 0x1E
	write_u16(&mut file_headers, 0x0000); // 0x20
	write_u16(&mut file_headers, 0x0000); // 0x22
	write_u16(&mut file_headers, 0x0000); // 0x24: e_oemid
	write_u16(&mut file_headers, 0x0000); // 0x26: e_oeminfo
	// e_res2 (20 bytes)
	for _ in 0..10 {
		write_u16(&mut file_headers, 0x0000); // 0x28..0x3B
	}
	write_u32(&mut file_headers, 0x80); // 0x3C - 0x3F: pointer to PE header (just after this but we could leave room for a DOS stub)
	// Dos Stub (64 bytes)
	const DOS_STUB: [u8; 64] = [
		0x0E, 0x1F, // push cs; pop ds
		0xBA, 0x0E, 0x00, // mov dx, 0x000E
		0xB4, 0x09, // mov ah, 0x09
		0xCD, 0x21, // int 0x21
		0xB8, 0x01, 0x4C, // mov ax, 0x4C01
		0xCD, 0x21, // int 0x21
		// "This program cannot be run in DOS mode.\r\n$"
		0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x20, 0x63, 0x61,
		0x6E, 0x6E, 0x6F, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20, 0x69, 0x6E, 0x20,
		0x44, 0x4F, 0x53, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x0D, 0x0D, 0x0A, 0x24, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x00,
	];
	write_bytes(&mut file_headers, &DOS_STUB);
	write_bytes(&mut file_headers, b"PE\0\0"); // 0x80 - 0x83: PE signature "PE\0\0"

	// Now the COFF File Header (20 bytes)
	write_u16(&mut file_headers, 0x8664); // 0x84 - 0x85: Machine type https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#machine-types
	write_u16(&mut file_headers, 0x0002); // 0x86 - 0x87: Number of sections
	write_u32(&mut file_headers, 0x00000000); // 0x88 - 0x8B: TimeDateStamp TODO
	write_u32(&mut file_headers, 0x00000000); // 0x8C - 0x8F: PointerToSymbolTable
	write_u32(&mut file_headers, 0x00000000); // 0x90 - 0x93: Number of symbols
	let size_of_optional_header_index = write_u16(&mut file_headers, 0x0000); // 0x94 - 0x95: SizeOfOptionalHeader
	write_u16(&mut file_headers, 0x022E); // 0x96 - 0x97: Characteristics (0x0002: valid file, 0x0020: large address aware)

	let optional_header_start = file_headers.len();

	// Optional Header Standard Fields
	write_u16(&mut file_headers, 0x020B); // 0x98 - 0x99: Magic number (0x10B = PE32, 0x20B = PE32+ (64 bit))
	write_u8(&mut file_headers, 0x02); // 0x9A: Linker major version
	write_u8(&mut file_headers, 0x2D); // 0x9B: Linker minor version
	write_u32(&mut file_headers, size_of_code); // 0x9C - 0x9F: Size of code block
	write_u32(&mut file_headers, size_of_initialized_data); // 0xA0 - 0xA3: Size of initialized data
	write_u32(&mut file_headers, size_of_uninitialized_data); // 0xA4 - 0xA7: Size of uninitialized data
	write_u32(&mut file_headers, adress_of_entry_point); // 0xA8 - 0xAB: Adress of the entry point of the program
	write_u32(&mut file_headers, base_of_code); // 0xAC - 0xAF: Base of code

	// Optional Header Windows-Specific Fields
	write_u64(&mut file_headers, 0x140000000); // 0xB0 - 0xB7: Image base
	write_u32(&mut file_headers, 0x1000); // 0xB8 - 0xBB: Section alignment (4096 bytes)
	write_u32(&mut file_headers, 0x200); // 0xBC - 0xBF: File alignment (512 bytes)
	write_u16(&mut file_headers, 0x04); // 0xC0 - 0xC1: Operation system version major
	write_u16(&mut file_headers, 0x00); // 0xC2 - 0xC3: Operation system version minor
	write_u16(&mut file_headers, image_version_major); // 0xC4 - 0xC5: Image version major
	write_u16(&mut file_headers, image_version_minor); // 0xC6 - 0xC7: Image version minor
	write_u16(&mut file_headers, 0x05); // 0xC8 - 0xC9: Subsystem version major
	write_u16(&mut file_headers, 0x02); // 0xCA - 0xCB: Subsystem version minor
	write_u32(&mut file_headers, 0x0000); // 0xCC - 0xCF: Win32 version value (reserved, must be 0)
	write_u32(&mut file_headers, size_of_image); // 0xD0 - 0xD3: Size of the image (in bytes)
	write_u32(&mut file_headers, size_of_headers); // 0xD4 - 0xD7: Size of headers (in bytes)
	write_u32(&mut file_headers, checksum); // 0xD8 - 0xDB: Checksum
	write_u16(&mut file_headers, 0x03); // 0xDC - 0xDD: Subsystem (3 = The Windows character subsystem)
	write_u16(&mut file_headers, 0x0100 | 0x0040 | 0x0020); // 0xDE - 0xDF: DllCharacteristics (0x0040 = Dynamic base, 0x0100 = NX compatible) https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#dll-characteristics
	write_u64(&mut file_headers, 0x200000); // 0xE0 - 0xE7: Size of stack reserve
	write_u64(&mut file_headers, 0x1000); // 0xE8 - 0xEF: Size of stack commit
	write_u64(&mut file_headers, 0x100000); // 0xF0 - 0xF7: Size of heap reserve
	write_u64(&mut file_headers, 0x1000); // 0xF8 - 0xFF: Size of heap commit
	write_u32(&mut file_headers, 0x0); // 0x100 - 0x103: Loader flags (reserved, must be 0)
	write_u32(&mut file_headers, 0x10); // 0x104 - 0x107: Number of RVA and sizes

	// Writing 16 data directories (16 * 8 = 128 bytes) https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#optional-header-data-directories-image-only

	// Export table
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Import table
	write_u32(&mut file_headers, 0x5000); // VirtualAddress
	let import_table_size_index = write_u32(&mut file_headers, 0x00000000); // Size
																			// Resource table
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Exception table
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Certificate table
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Base relocation table
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Debug
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Architecture
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Global pointer
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // TLS table
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Load config table
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Bound import
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // IAT
	let iat_rav_index = write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	let iat_size_index = write_u32(&mut file_headers, 0x00000000); // Size
																   // Delay import descriptor
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // CLR runtime header
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size
											  // Reserved
	write_u32(&mut file_headers, 0x00000000); // VirtualAddress
	write_u32(&mut file_headers, 0x00000000); // Size

	// Update size of optional header
	let optional_header_end = file_headers.len();
	let size_of_optional_header = (optional_header_end - optional_header_start) as u16;
	write_at_u16(
		&mut file_headers,
		size_of_optional_header_index,
		size_of_optional_header,
	);

	// Section Table (Section Headers)
	// Normally would be:
	/*
	.text → compiled machine code
	.rdata → read-only data (string literals, constants, import table)
	.data → global/static variables (read/write)
	.rsrc → resources (icons, manifests, version info)
	.reloc → relocation table (for when the EXE is loaded at a different base address)
	*/
	// One text section for the code https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#section-table-section-headers
	write_bytes(&mut file_headers, b".text\0\0\0"); // 0x00 - 0x07: Section name (8 bytes, null-padded)
	let text_size_index = write_u32(&mut file_headers, 0x0000000B); // 0x08 - 0x0B: Virtual size (size when loaded into memory)
	write_u32(&mut file_headers, 0x00001000); // 0x0C - 0x0F: Virtual address (RVA when loaded into memory) (4096)
	write_u32(&mut file_headers, 0x00000200); // 0x10 - 0x13: Size of raw data (size in the file, must be multiple of file alignment) (512)
	write_u32(&mut file_headers, 0x00000400); // 0x14 - 0x17: Pointer to raw data (file offset) (512)
	write_u32(&mut file_headers, 0x00000000); // 0x18 - 0x1B: Pointer to relocations (not used for executable images)
	write_u32(&mut file_headers, 0x00000000); // 0x1C - 0x1F: Pointer to line numbers (deprecated, set to 0)
	write_u16(&mut file_headers, 0x0000); // 0x20 - 0x21: Number of relocations (not used for executable images)
	write_u16(&mut file_headers, 0x0000); // 0x22 - 0x23: Number of line numbers (deprecated, set to 0)
	write_u32(&mut file_headers, 0x60000020); // 0x24 - 0x27: Characteristics (0x60000020 = code, execute/read)

	// And one rdata section for the imports https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#section-table-section-headers
	write_bytes(&mut file_headers, b".idata\0\0"); // 0x00 - 0x07: Section name (8 bytes, null-padded)
	let idata_size_index = write_u32(&mut file_headers, 0x00000000); // 0x08 - 0x0B: Virtual size (size when loaded into memory)
	write_u32(&mut file_headers, 0x00005000); // 0x0C - 0x0F: Virtual address (RVA when loaded into memory) (8192)
	write_u32(&mut file_headers, 0x00000200); // 0x10 - 0x13: Size of raw data (size in the file, must be multiple of file alignment) (512)
	write_u32(&mut file_headers, 0x00000600); // 0x14 - 0x17: Pointer to raw data (file offset) (1024)
	write_u32(&mut file_headers, 0x00000000); // 0x18 - 0x1B: Pointer to relocations (not used for executable images)
	write_u32(&mut file_headers, 0x00000000); // 0x1C - 0x1F: Pointer to line numbers (deprecated, set to 0)
	write_u16(&mut file_headers, 0x0000); // 0x20 - 0x21: Number of relocations (not used for executable images)
	write_u16(&mut file_headers, 0x0000); // 0x22 - 0x23: Number of line numbers (deprecated, set to 0)
	write_u32(&mut file_headers, 0x40000040); // 0x24 - 0x27: Characteristics (0x40000040 = initialized data, read-only)

	println!("File headers size: {}", file_headers.len());

	// OUR PROGRAM will be at 0x400 (1024) in storage
	let mut padding_needed = 1024 - (file_headers.len() % 1024);
	write_zeroes(&mut file_headers, padding_needed);
	//write_at_u32(&mut file_headers, text_size_index, 64); // Update text section virtual size
	write_at_u32(&mut file_headers, text_size_index, 0x4000); // Update text section virtual size

	// Writing our program: 31 C9 48 8B 05 F7 0F 00 00 FF D0
	//write_bytes(&mut file_headers, &[0x31, 0xC9,
	//    0x48, 0x8B, 0x05, 0xF7, 0x0F, 0x00, 0x00,
	//    0xFF, 0xD0]);

	// _start minimal example: prints 'a' and exits
	// Text is at 0x1000 (4096) in memory
	// Idata is at 0x5000 (20480) in memory

	let mut idata: Vec<u8> = Vec::new();

	// Writing the import table
	let mut imports = vec![ImportEntry {
		dll_name: "KERNEL32.dll".to_string(),
		function_names: vec![
			"ExitProcess".to_string(),
			"GetStdHandle".to_string(),
			"WriteFile".to_string(),
		],
		function_address_rvas: Vec::new(),
	}];

	let (iat_rva, iat_size) = write_idata(&mut idata, &mut imports, 0x5000);

	println!(
		"rvas: {}, {}, {}",
		imports[0].function_address_rvas[0],
		imports[0].function_address_rvas[1],
		imports[0].function_address_rvas[2]
	);

	for (syscall_name, at) in syscalls_to_resolve {
		let mut found = false;
		for import in &imports {
			for (i, func_name) in import.function_names.iter().enumerate() {
				if *func_name == *syscall_name {
					let func_rva = import.function_address_rvas[i] - 0x1000;
					write_at_u32(machine_code, *at, func_rva);

					found = true;
					break;
				}
			}
			if found {
				break;
			}
		}
		if !found {
			panic!("Could not find syscall {}", syscall_name);
		}
	}

	let get_std_handle_rel32 = imports[0].function_address_rvas[1] - (0x1000 + 11 + 6);
	let write_file_rel32 = imports[0].function_address_rvas[2] - (0x1000 + 48 + 6);
	let exit_process_rel32 = imports[0].function_address_rvas[0] - (0x1000 + 57 + 6);

	let mut program: Vec<u8> = vec![
		// prologue: reserve shadow space (Win64 ABI)
		0x48, 0x83, 0xEC, 0x28, // sub rsp, 0x28
		// GetStdHandle(-11)
		0x48, 0xC7, 0xC1, 0xF5, 0xFF, 0xFF, 0xFF, // mov rcx, -11
		0xFF, 0x15, 0xAA, 0xAA, 0xAA, 0xAA, // call [rip+rel32] ; GetStdHandle IAT
		// save stdout handle
		0x48, 0x89, 0xC1, // mov rcx, rax    ; hFile
		// WriteFile(stdout, "a", 1, NULL, NULL)
		0x48, 0x8D, 0x15, 0x24, 0x00, 0x00, 0x00, // lea rdx, [rip+0x24] ; &"a"
		0x49, 0xB8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r8, 1
		0x4D, 0x31, 0xC9, // xor r9, r9
		0x48, 0x31, 0xC0, // xor rax, rax
		0x48, 0x89, 0x44, 0x24, 0x20, // mov [rsp+0x20], rax ; lpOverlapped=NULL
		0xFF, 0x15, 0xBB, 0xBB, 0xBB, 0xBB, // call [rip+rel32] ; WriteFile IAT
		// ExitProcess(0)
		0x48, 0x31, 0xC9, // xor rcx, rcx
		0xFF, 0x15, 0xCC, 0xCC, 0xCC, 0xCC, // call [rip+rel32] ; ExitProcess IAT
		// --- data ---
		0x61, 0x00, // "a\0"
	];

	// Patch the relative calls
	write_at_u32(&mut program, 13, get_std_handle_rel32);
	//write_at_u32(&mut program, 50, write_file_rel32);
	//write_at_u32(&mut program, 59, exit_process_rel32);

	//write_bytes(&mut file_headers, &program2);
	write_bytes(&mut file_headers, machine_code);

	println!("Machine code: {:02X?}", machine_code);

	//write_bytes(&mut file_headers, &[0x55, 0x48, 0x89, 0xe5, 0x90, 0x5d, 0xc3, 0x55, 0x48, 0x89, 0xe5, 0x48, 0x83, 0xec, 0x20, 0xe8, 0xec, 0xff, 0xff, 0xff, 0xb9, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8b, 0x05, 0x18, 0x40, 0x00, 0x00, 0xff, 0xd0, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0xff, 0x25, 0x02, 0x40, 0x00, 0x00, 0x90, 0x90, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]);
	// THE DATA for our program will be at 0x600 (1536) in memory
	padding_needed = 512 - machine_code.len();
	write_zeroes(&mut file_headers, padding_needed);

	let idata_size = idata.len() as u32;
	write_at_u32(&mut file_headers, idata_size_index, idata_size); // Update idata section virtual size
	write_at_u32(&mut file_headers, import_table_size_index, idata_size); // Update import table size

	write_at_u32(&mut file_headers, iat_rav_index, iat_rva); // Update IAT RVA
	write_at_u32(&mut file_headers, iat_size_index, iat_size);

	padding_needed = 512 - idata_size as usize;
	write_zeroes(&mut idata, padding_needed);

	write_file_bytes(file, &file_headers)?;
	write_file_bytes(file, &idata)?;

	Ok(())
}

fn write_idata(idata: &mut Vec<u8>, imports: &mut [ImportEntry], rva_start: u32) -> (u32, u32) {
	let mut ilt_reference_indices = Vec::<usize>::new();
	let mut iat_reference_indices = Vec::<usize>::new();
	let mut name_reference_indices = Vec::<usize>::new();
	let mut hint_name_reference_indices_ilt = Vec::<Vec<usize>>::new();
	let mut hint_name_reference_indices_iat = Vec::<Vec<usize>>::new();

	// Import Directory Table (20 bytes each)
	for _ in imports.iter() {
		let ilt_reference_index = write_u32(idata, 0); // Placeholder for OriginalFirstThunk (RVA to Import Lookup Table)
		write_u32(idata, 0x00000000); // TimeDateStamp
		write_u32(idata, 0x00000000); // ForwarderChain
		let name_reference_index = write_u32(idata, 0x00000000); // Placeholder for Name
		let iat_reference_index = write_u32(idata, 0x00000000); // Placeholder for FirstThunk

		ilt_reference_indices.push(ilt_reference_index);
		iat_reference_indices.push(iat_reference_index);
		name_reference_indices.push(name_reference_index);
	}

	// Null entry to terminate the table
	write_u32(idata, 0x00000000); // OriginalFirstThunk
	write_u32(idata, 0x00000000); // TimeDateStamp
	write_u32(idata, 0x00000000); // ForwarderChain
	write_u32(idata, 0x00000000); // Name
	write_u32(idata, 0x00000000); // FirstThunk

	// Now for each import, write the ILT
	for (import_index, import) in imports.iter().enumerate() {
		let mut ilt_hint_name_indices = Vec::<usize>::new();

		// RVA to this import's ILT (which is about to be written)
		let ilt_rva = rva_start + idata.len() as u32;

		// Import Lookup Table (ILT) - array of RVAs (8 bytes each)
		for _ in &import.function_names {
			let hint_name_index = write_u64(idata, 0); // Placeholder for RVA to Hint/Name table
			ilt_hint_name_indices.push(hint_name_index);
		}
		write_u64(idata, 0x00000000); // Null terminator for ILT

		// Now write the ILT reference
		write_at_u32(idata, ilt_reference_indices[import_index], ilt_rva);

		hint_name_reference_indices_ilt.push(ilt_hint_name_indices);
	}

	let total_iat_start = idata.len();
	let total_iat_rva = rva_start + idata.len() as u32;

	// Now for each import, write the IAT
	for (import_index, import) in imports.into_iter().enumerate() {
		let mut iat_hint_name_indices = Vec::<usize>::new();

		// RVA to this import's IAT (which is about to be written)
		let iat_rva = rva_start + idata.len() as u32;

		// Import Address Table (IAT) - array of RVAs (8 bytes each)
		for (function_index, _) in import.function_names.iter().enumerate() {
			let hint_name_index = write_u64(idata, 0); // Placeholder for RVA to Hint/Name table
			iat_hint_name_indices.push(hint_name_index);
			let function_iat_rva = iat_rva + (function_index as u32) * 8;
			import.function_address_rvas.push(function_iat_rva);
		}
		write_u64(idata, 0x00000000); // Null terminator for IAT

		// Now write the IAT reference
		write_at_u32(idata, iat_reference_indices[import_index], iat_rva);

		hint_name_reference_indices_iat.push(iat_hint_name_indices);
	}

	let total_iat_end = idata.len();
	let total_iat_size = (total_iat_end - total_iat_start) as u32;

	// Now writing the hint/name table
	for (import_index, import) in imports.iter().enumerate() {
		for (function_index, function_name) in import.function_names.clone().iter().enumerate() {
			let hint_name_rva = rva_start + idata.len() as u32;
			write_u16(idata, 0x017C); // Hint (index to export name pointer table)
			write_bytes(idata, function_name.as_bytes()); // Name of the function to import
			write_u8(idata, 0); // Null terminator

			// Accounting for possible odd name lengths
			if (function_name.len() + 1) % 2 == 1 {
				write_u8(idata, 0);
			}

			// Updating the hint/name references in ILT and IAT
			write_at_u32(
				idata,
				hint_name_reference_indices_ilt[import_index][function_index],
				hint_name_rva,
			);
			write_at_u32(
				idata,
				hint_name_reference_indices_iat[import_index][function_index],
				hint_name_rva,
			);
		}
	}

	// Now writing the DLL names
	for (import_index, import) in imports.iter().enumerate() {
		let name_rva = rva_start + idata.len() as u32;

		write_bytes(idata, import.dll_name.as_bytes());
		write_u8(idata, 0); // Null terminator

		// Accounting for possible odd name lengths
		if (import.dll_name.len() + 1) % 2 == 1 {
			write_u8(idata, 0);
		}

		// Updating the name reference
		write_at_u32(idata, name_reference_indices[import_index], name_rva);
	}

	return (total_iat_rva as u32, total_iat_size as u32);
}

fn write_file_u8(file: &mut impl Write, value: u8) -> std::io::Result<()> {
	file.write_all(&[value])
}

fn write_file_u16(file: &mut impl Write, value: u16) -> std::io::Result<()> {
	let bytes = value.to_le_bytes();
	file.write_all(&bytes)
}

fn write_file_u32(file: &mut impl Write, value: u32) -> std::io::Result<()> {
	let bytes = value.to_le_bytes();
	file.write_all(&bytes)
}

fn write_file_u64(file: &mut impl Write, value: u64) -> std::io::Result<()> {
	let bytes = value.to_le_bytes();
	file.write_all(&bytes)
}

fn write_file_bytes(file: &mut impl Write, data: &[u8]) -> std::io::Result<()> {
	file.write_all(data)
}

fn write_file_zeroes(file: &mut impl Write, count: usize) -> std::io::Result<()> {
	const CHUNK: [u8; 256] = [0; 256];
	let mut remaining = count;
	while remaining > 0 {
		let write_now = remaining.min(256);
		file.write_all(&CHUNK[..write_now])?;
		remaining -= write_now;
	}
	Ok(())
}

fn write_u8(buf: &mut Vec<u8>, value: u8) -> usize {
	let index = buf.len();
	buf.push(value);
	index
}

fn write_u16(buf: &mut Vec<u8>, value: u16) -> usize {
	let index = buf.len();
	buf.extend_from_slice(&value.to_le_bytes());
	index
}

fn write_u32(buf: &mut Vec<u8>, value: u32) -> usize {
	let index = buf.len();
	buf.extend_from_slice(&value.to_le_bytes());
	index
}

fn write_u64(buf: &mut Vec<u8>, value: u64) -> usize {
	let index = buf.len();
	buf.extend_from_slice(&value.to_le_bytes());
	index
}

fn write_bytes(buf: &mut Vec<u8>, data: &[u8]) -> usize {
	let index = buf.len();
	buf.extend_from_slice(data);
	index
}

fn write_zeroes(buf: &mut Vec<u8>, count: usize) -> usize {
	let index = buf.len();
	buf.resize(buf.len() + count, 0);
	index
}

fn write_at_u8(buf: &mut Vec<u8>, at: usize, value: u8) {
	if buf.len() <= at {
		buf.resize(at + 1, 0);
	}
	buf[at] = value;
}

fn write_at_u16(buf: &mut Vec<u8>, at: usize, value: u16) {
	let bytes = value.to_le_bytes();
	if buf.len() < at + 2 {
		buf.resize(at + 2, 0);
	}
	buf[at..at + 2].copy_from_slice(&bytes);
}

fn write_at_u32(buf: &mut Vec<u8>, at: usize, value: u32) {
	let bytes = value.to_le_bytes();
	if buf.len() < at + 4 {
		buf.resize(at + 4, 0);
	}
	buf[at..at + 4].copy_from_slice(&bytes);
}

fn write_at_u64(buf: &mut Vec<u8>, at: usize, value: u64) {
	let bytes = value.to_le_bytes();
	if buf.len() < at + 8 {
		buf.resize(at + 8, 0);
	}
	buf[at..at + 8].copy_from_slice(&bytes);
}

pub fn write_at_bytes(buf: &mut Vec<u8>, at: usize, data: &[u8]) {
	if buf.len() < at + data.len() {
		buf.resize(at + data.len(), 0);
	}
	buf[at..at + data.len()].copy_from_slice(data);
}

fn write_at_zeroes(buf: &mut Vec<u8>, at: usize, count: usize) {
	if buf.len() < at + count {
		buf.resize(at + count, 0);
	} else {
		buf[at..at + count].fill(0);
	}
}

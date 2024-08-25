pub struct Linker;

impl Linker {
    pub fn link(objects: Vec<Vec<u8>>) -> Vec<u8> {
        let mut executable = Vec::new();

        // ELF Header (64 bytes for 64-bit ELF)
        executable.extend_from_slice(&[
            0x7f, b'E', b'L', b'F', // Magic number
            0x02, // 64-bit
            0x01, // Little endian
            0x01, // Current version
            0x00, // System V ABI
            0x00, // ABI Version
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
            0x02, 0x00, // Executable
            0x3e, 0x00, // x86-64
            0x01, 0x00, 0x00,
            0x00, // Version
                  // ... (fill in the rest of the header)
        ]);

        // Program header table

        // Section header table

        // Actual code
        for object in objects {
            executable.extend_from_slice(&object);
        }

        executable
    }
}

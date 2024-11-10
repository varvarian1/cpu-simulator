pub struct CPU {
    ram: [u8; 64],
    register: [u8; 2],
    pc: usize,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            ram: [0; 64],
            register: [0; 2],
            pc: 0,
        }
    }

    pub fn set_register(&mut self, index: usize, value: u8) {
        self.register[index] = value;
    }

    pub fn get_register(&self, index: usize) -> u8 {
        self.register[index]
    }

    pub fn load_command(&mut self, address: usize, command: Command) {
        let command_bytes = match command {
            Command::ADD { x, y } => [0x01, x as u8, y as u8],
            Command::SBC { x, y } => [0x02, x as u8, y as u8],
        };
        
        for (i, &byte) in command_bytes.iter().enumerate() {
            self.ram[address + i] = byte;
        }
    }

    pub fn execute_command(&mut self) {
        while self.pc < self.ram.len() {
            let result = self.execute_command_at(self.pc);
            println!("Command Result: {}", result);
            
            self.pc += 3;
        }
    }

    pub fn execute_command_at(&mut self, address: usize) -> u8 {
        let command = self.fetch_command_at(address);
        
        match command {
            Command::ADD { x, y } => {
                let result = self.get_register(x).wrapping_add(self.get_register(y));
                self.set_register(0, result);
                result 
            }
            Command::SBC { x, y } => {
                let result = self.get_register(x).wrapping_sub(self.get_register(y));
                self.set_register(0, result); 
                result 
            }
        }
    }

    pub fn fetch_command_at(&self, address: usize) -> Command {
        let opcode = self.ram[address];
        let x = self.ram[address + 1] as usize;
        let y = self.ram[address + 2] as usize;
        match opcode {
            0x01 => Command::ADD { x, y },
            0x02 => Command::SBC { x, y },
            _ => panic!("Unknown command"),
        }
    }
}

pub enum Command {
    ADD { x: usize, y: usize },
    SBC { x: usize, y: usize },
    // NOP
    // JMP
}
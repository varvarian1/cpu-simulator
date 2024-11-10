mod cpu;

use cpu::CPU;
use cpu::Command;

fn main() {
    let mut cpu = CPU::new(); 

    cpu.set_register(0, 5);
    cpu.set_register(1, 10);

    cpu.load_command(0, Command::ADD { x: 0, y: 1 });
    cpu.load_command(3, Command::SBC { x: 0, y: 1 });

    cpu.execute_command();
}
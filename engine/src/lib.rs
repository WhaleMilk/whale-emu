mod cpu;

pub enum Inputs {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

pub trait CallbackEvent {
    fn debug_opcode(&mut self);
}

pub trait Context {
    fn callbacks(&mut self) -> Option<&mut dyn CallbackEvent>;
}
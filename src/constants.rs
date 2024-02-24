use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Attribute: u32 {
        const Earth = 1;
        const Water = 2;
        const Fire = 4;
        const Wind = 8;
        const Light = 16;
        const Dark = 32;
        const Devine = 64;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct OT: u32 {
        const OCG = 1;
        const TCG = 2;
        const Custom = 4;
        const SC = 8;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Race: u32 {
        const Warrior = 1;
        const Spellcaster = 2;
        const Fairy = 4;
        const Fiend = 8;
        const Zombie = 16;
        const Machine = 32;
        const Aqua = 64;
        const Pyro = 128;
        const Rock = 256;
        const Windbeast = 512;
        const Plant = 1024;
        const Insect = 2048;
        const Thunder = 4096;
        const Dragon = 8192;
        const Beast = 16384;
        const Beastwarrior = 32768;
        const Dinosaur = 65536;
        const Fish = 131072;
        const Seaserpent = 262144;
        const Reptile = 524288;
        const Psycho = 1048576;
        const Devine = 2097152;
        const Creatorgod = 4194304;
        const Wyrm = 8388608;
        const Cybers = 16777216;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Type: u32 {
        const Monster = 1;
        const Spell = 2;
        const Trap = 4;
        const Normal = 16;
        const Effect = 32;
        const Fusion = 64;
        const Ritual = 128;
        const Trapmonster = 256;
        const Spirit = 512;
        const Union = 1024;
        const Dual = 2048;
        const Tuner = 4096;
        const Synchro = 8192;
        const Token = 16384;
        const Quickplay = 65536;
        const Continuous = 131072;
        const Equip = 262144;
        const Field = 524288;
        const Counter = 1048576;
        const Flip = 2097152;
        const Toon = 4194304;
        const Xyz = 8388608;
        const Pendulum = 16777216;
        const Spsummon = 33554432;
        const Link = 67108864;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Linkmarkers: i32 {
        const BottomLeft = 1;
        const Bottom = 2;
        const BottomRight = 4;
        const Left = 8;
        const Right = 32;
        const TopLeft = 64;
        const Top = 128;
        const TopRight = 256;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Category: u64 {
        const category1 = 0x1;
        const category2 = 0x2;
        const category3 = 0x4;
        const category4 = 0x8;
        const category5 = 0x10;
        const category6 = 0x20;
        const category7 = 0x40;
        const category8 = 0x80;
        const category9 = 0x100;
        const category10 = 0x200;
        const category11 = 0x400;
        const category12 = 0x800;
        const category13 = 0x1000;
        const category14 = 0x2000;
        const category15 = 0x4000;
        const category16 = 0x8000;
        const category17 = 0x10000;
        const category18 = 0x20000;
        const category19 = 0x40000;
        const category20 = 0x80000;
        const category21 = 0x100000;
        const category22 = 0x200000;
        const category23 = 0x400000;
        const category24 = 0x800000;
        const category25 = 0x1000000;
        const category26 = 0x2000000;
        const category27 = 0x4000000;
        const category28 = 0x8000000;
        const category29 = 0x10000000;
        const category30 = 0x20000000;
        const category31 = 0x40000000;
        const category32 = 0x80000000;
    }
}

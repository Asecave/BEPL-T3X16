use std::{cmp::min, fs::File, io::Write};

use quartz_nbt::{io::{write_nbt, Flavor}, NbtCompound, NbtList};

const BARREL_OFFSET_X: i16 = 4;
const BARREL_OFFSET_Y: i16 = 2;
const BARREL_OFFSET_Z: i16 = 3;

pub fn create_rom_schematic(hex_code: &Vec<i16>) {
    let lines: i16 = hex_code.len() as i16;

    let width: i16 = min(lines, 16) * BARREL_OFFSET_X - BARREL_OFFSET_X + 1;
    let length: i16 = (lines / 16 + 1) * BARREL_OFFSET_Z - BARREL_OFFSET_Z + 1;
    let height: i16 = 4 * BARREL_OFFSET_Y - BARREL_OFFSET_Y + 1;

    let mut block_entities = NbtList::with_capacity(lines as usize * 4);
    let mut data = vec![1i8; (width * height * length) as usize];

    let mut y: i16 = 0;
    for part in 0..4 {
        let mut x: i16 = 0;
        let mut z: i16 = 0;
        for instruction in hex_code {
            let signal_strength = (*instruction as u16 >> (part * 4)) % 16;
            block_entities.push(create_signal_strength_barrel(signal_strength as usize, vec![x as i32, y as i32, z as i32]));
            data[(x + z * width + y * width * length) as usize] = 0;
            x += BARREL_OFFSET_X;
            if x > width  {
                z += BARREL_OFFSET_Z;
                x = 0;
            }
        }
        y += BARREL_OFFSET_Y;
    }

    let schematic = create_schematic_nbt(width, length, height, block_entities, data);

    write_nbt_to_file(schematic, "C:/Users/Asecave/AppData/Roaming/ATLauncher/instances/SurvivalTweaked121/config/worldedit/schematics/rom.schem");
}

fn write_nbt_to_file(nbt: NbtCompound, path: &str) {

    let mut buf: Vec<u8> = Vec::new();
    write_nbt(&mut buf, None, &nbt, Flavor::GzCompressed).unwrap();

    let mut file = File::create(path).expect("Could not create file");
    file.write_all(&buf).expect("Could not write file");
}

fn create_signal_strength_barrel(signal_strength: usize, pos: Vec<i32>) -> NbtCompound {
    let mut items = vec![0, 1, 124, 247, 371, 494, 618, 741, 864, 988, 1111, 1235, 1358, 1482, 1605, 1728][signal_strength];
    let stacks = items / 64;
    let rest = items % 64;
    let slots_used = stacks + min(rest, 1);

    let mut barrel = NbtCompound::new();
    let mut data = NbtCompound::new();

    let mut item_list = NbtList::with_capacity(slots_used);
    
    let mut slot: i8 = 0;
    while items != 0 {
        let mut stack = NbtCompound::new();
        let count = min(items, 64);
        stack.insert("count", count as i32);
        stack.insert("id", "minecraft:redstone");
        stack.insert("Slot", slot);
        slot += 1;

        item_list.push(stack);

        items -= count;
    }

    data.insert("Items", item_list);
    data.insert("id", "minecraft:barrel");

    barrel.insert("Data", data);
    barrel.insert("Id", "minecraft:barrel");
    barrel.insert("Pos", pos);
    barrel
}

fn create_schematic_nbt(width: i16, length: i16, height: i16, block_entities: NbtList, data: Vec<i8>) -> NbtCompound {
    let mut base_nbt = NbtCompound::new();
    let mut schematic = NbtCompound::new();
    let mut blocks = NbtCompound::new();
    let mut palette = NbtCompound::new();

    palette.insert("minecraft:air", 1);
    palette.insert("minecraft:barrel", 0);

    blocks.insert("Palette", palette);
    blocks.insert("BlockEntities", block_entities);
    blocks.insert("Data", data);

    schematic.insert("Blocks", blocks);
    schematic.insert("DataVersion", 3953);
    schematic.insert("Width", width);
    schematic.insert("Length", length);
    schematic.insert("Height", height);
    schematic.insert("Version", 3);
    schematic.insert("Offset", vec![0, -7, 0]);

    base_nbt.insert("Schematic", schematic);
    base_nbt
}

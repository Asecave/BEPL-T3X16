import mcschematic

offset_x = 4
offset_z = 3

def create_signal_strength_barrel(signal_strength):
    items = 0
    match signal_strength:
        case 1:
            items = 1
        case 2:
            items = 124
        case 3:
            items = 247
        case 4:
            items = 371
        case 5:
            items = 494
        case 6:
            items = 618
        case 7:
            items = 741
        case 8:
            items = 864
        case 9:
            items = 988
        case 10:
            items = 1111
        case 11:
            items = 1235
        case 12:
            items = 1358
        case 13:
            items = 1482
        case 14:
            items = 1605
        case 15:
            items = 1728

    properties = "minecraft:barrel{Items:["
    last_slot = 0

    if signal_strength > 1:
        for i in range(items // 64):
            properties += "{Slot:" + str(i) + "b, Count:64, id:\"minecraft:redstone\"},"
            last_slot = i
    else:
        last_slot = -1
    
    if items % 64 != 0:
        properties += "{Slot:" + str(last_slot + 1) + "b, Count:" + str((items % 64)) + ", id:\"minecraft:redstone\"}"
    else:
        if signal_strength != 0:
            properties = properties[:-1]

    properties += "]}"

    return properties


schem = mcschematic.MCSchematic()

for x in range(3):
    for z in range(3):
        schem.setBlock((x * offset_x, 0, z * offset_z), create_signal_strength_barrel(0))
        schem.setBlock((x * offset_x, 2, z * offset_z), create_signal_strength_barrel(0))
        schem.setBlock((x * offset_x, 4, z * offset_z), create_signal_strength_barrel(0))
        schem.setBlock((x * offset_x, 6, z * offset_z), create_signal_strength_barrel(0))

schem.save(".", "rom", mcschematic.Version.JE_1_20_1)
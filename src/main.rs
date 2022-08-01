use anyhow::Result;
use nbt::Value;
use std::collections::HashMap;
use std::fs::File;

const WIDTH: usize = 110;
const HEIGHT: usize = 110;

fn main() -> Result<()> {
    let mut schem = nbt::Blob::named("Schematic");
    schem.insert("Version", Value::Int(2))?;
    schem.insert("BlockEntities", Value::List(Vec::new()))?;
    schem.insert("DataVersion", Value::Int(2975))?;

    let mut metadata = HashMap::new();
    metadata.insert("WEOffsetZ".to_string(), Value::Int(0));
    metadata.insert("WEOffsetY".to_string(), Value::Int(0));
    metadata.insert("WEOffsetX".to_string(), Value::Int(0));
    schem.insert("Metadata", Value::Compound(metadata))?;

    let mut palette = HashMap::new();
    palette.insert("minecraft:air".to_string(), Value::Int(0));
    schem.insert("Palette", Value::Compound(palette))?;
    schem.insert("PaletteMax", Value::Int(1))?;

    schem.insert("Width", Value::Short(WIDTH as i16))?;
    schem.insert("Height", Value::Short(HEIGHT as i16))?;
    schem.insert("Length", Value::Short(1))?;
    schem.insert("BlockData", Value::ByteArray(vec![0; WIDTH * HEIGHT]))?;

    schem.insert("Offset", Value::IntArray(vec![0, 0, 0]))?;
    let mut entities = Vec::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut entity = HashMap::new();
            entity.insert("Id".to_string(), Value::String("minecraft:item_frame".to_string()));
            entity.insert("Pos".to_string(), Value::List(vec![
                Value::Double(x as f64),
                Value::Double(y as f64),
                Value::Double(0.0),
            ]));
            entity.insert("Rotation".to_string(), Value::List(vec![
                Value::Float(0.0),
                Value::Float(0.0),
            ]));

            // item frame data
            entity.insert("Fixed".to_string(), Value::Byte(1));
            entity.insert("Invisible".to_string(), Value::Byte(0));
            entity.insert("ItemDropChance".to_string(), Value::Float(1.0));
            entity.insert("ItemRotation".to_string(), Value::Byte(0));

            let mut item = HashMap::new();
            item.insert("id".to_string(), Value::String("minecraft:filled_map".to_string()));
            let mut item_tag = HashMap::new();
            item_tag.insert("map".to_string(), Value::Int(entities.len() as i32));
            item.insert("tag".to_string(), Value::Compound(item_tag));
            item.insert("Count".to_string(), Value::Byte(1));
            entity.insert("Item".to_string(), Value::Compound(item));

            // hangable data
            entity.insert("Facing".to_string(), Value::Byte(3));
            entity.insert("TileX".to_string(), Value::Int(x as i32));
            entity.insert("TileY".to_string(), Value::Int(y as i32));
            entity.insert("TileZ".to_string(), Value::Int(0));

            entities.push(Value::Compound(entity));
        }
    }
    schem.insert("Entities", Value::List(entities))?;

    schem.to_gzip_writer(&mut File::create("map_wall.schem")?)?;

    Ok(())
}

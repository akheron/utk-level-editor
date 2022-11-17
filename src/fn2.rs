#[derive(Debug)]
pub struct Line {
    pub x: u8,
    pub y: u8,
    pub width: u8,
}

#[derive(Debug)]
pub struct Character {
    pub width: u32,
    pub height: u32,
    pub lines: Vec<Line>,
}

pub struct FN2 {
    pub first_visible_character: u8,
    pub characters: Vec<Character>,
}

impl FN2 {
    pub fn parse(data: &[u8]) -> FN2 {
        let mut offset: usize = 0x027D;
        let characters = (0..92)
            .map(|_| {
                let width = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
                offset += 4;
                let height = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
                offset += 4;
                let color_bytes = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
                offset += 4;
                let line_bytes = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
                offset += 4;
                offset += color_bytes as usize;

                let lines = (0..(line_bytes / 3))
                    .filter_map(|_| {
                        let line = Line {
                            x: data[offset],
                            y: data[offset + 1],
                            width: data[offset + 2],
                        };
                        offset += 3;
                        if line.width > 0 {
                            Some(line)
                        } else {
                            None
                        }
                    })
                    .collect();

                Character {
                    width,
                    height,
                    lines,
                }
            })
            .collect();
        FN2 {
            first_visible_character: 33,
            characters,
        }
    }
}

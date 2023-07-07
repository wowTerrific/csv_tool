use std::io::Write;
use std::{error::Error, io};
use std::fs::OpenOptions;
use std::fs;

pub fn write_example() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["city", "region", "country", "population"])?;
    wtr.write_record(&["Elkhart", "Indiana", "US", "45"])?;
    wtr.write_record(&["Goshen", "Indiana", "US", "75"])?;
    wtr.write_record(&["Middlebury", "Indiana", "US", "100"])?;
    wtr.flush()?;
    Ok(())
}


pub fn write_fs(name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).open(name)?;
    file.write_all(b"this is new line\n")?;
    file.flush()?;
    Ok(())
}

pub fn write_csv(name: &str, data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).open(name)?;
    let mut text_buffer = String::new();

    for v in data {
        let line = v.join(",");
        text_buffer.push_str(&line);
        text_buffer.push_str("\n");
    }

    file.write_all(text_buffer.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn write_greg(name: &str, data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).open(name)?;
    let mut text_buffer = String::new();

    for v in data {
        let line = v.join(",");
        text_buffer.push_str(&line);
        text_buffer.push_str("\n");
    }
    let formatted_bytes = format!("{:?}",text_buffer.as_bytes());
    file.write_all(formatted_bytes.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn read_greg(name: &str) -> Result<String, Box<dyn Error>> {
    let mut text_buffer = fs::read_to_string(name)?;
    text_buffer.pop();

    let bind = &text_buffer[1..].to_string();
    
    let str_byte_array: Vec<&str> = bind.split(", ").collect();
    let byte_array = str_byte_array
                                .iter()
                                .map(|s| {
                                    s.parse::<u8>().expect("Could not parse greg file")
                                })
                                .collect();
    
    let result = String::from_utf8(byte_array)?;

    Ok(result)
}

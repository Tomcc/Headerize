extern crate xml;

use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::EventWriter;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let path = Path::new("C:/Users/tomma/dev/mcpe/1.3/handheld/project/VS2015/Minecraft/Minecraft.Shared/Minecraft.Shared.vcxitems");
    let outpath = Path::new("lol.xml");

    let file = File::open(path).unwrap();
    let mut file = BufReader::new(file);

    //why the hell does this UTF8 file have a BOM? Microsoft, that's why
    file.read(&mut[0, 0, 0]).unwrap();

    let parser = EventReader::new(file);
    let mut depth = 0;

    let file = File::create(outpath).unwrap();
    let file = BufWriter::new(file);

    let mut writer = xml::writer::EmitterConfig::new().perform_indent(true).create_writer(file);

    let mut includes: Vec<String> = vec![];

    let mut ignoredepth = 9999;
        
    let mut state = 0;

    for e in parser {
        match e {
            Ok(e) => {

                match e {
                    XmlEvent::StartElement { ref name, ref attributes, .. } => {
                        if name.local_name == "ClCompile" && attributes.len() > 0 {
                            ignoredepth = depth;

                            if state == 0 {
                                use xml::writer::XmlEvent;

                                //write out every cpp
                                for include in &includes {
                                    let start = XmlEvent::start_element("ClCompile")
                                        .attr("Include", &include)
                                        .into();
                                    
                                    writer.write::<XmlEvent>(start).unwrap();

                                    writer.write::<XmlEvent>(XmlEvent::start_element("PrecompiledHeader").into()).unwrap();
                                    writer.write::<XmlEvent>(XmlEvent::characters("NotUsing").into()).unwrap();
                                    writer.write::<XmlEvent>(XmlEvent::end_element().into()).unwrap();
                                    writer.write::<XmlEvent>(XmlEvent::end_element().into()).unwrap();
                                }

                                state = 1;
                            }
                        }
                        else if name.local_name == "ClInclude" && attributes.len() > 0 {
                            assert!(state == 0, "Can't find an include now!");
                            includes.push(attributes[0].value.clone());
                        }
                        depth += 1;
                    }
                    XmlEvent::EndElement { ref name } => {
                        depth -= 1;
                        if depth < ignoredepth {
                            ignoredepth = 9999;
                        }
                    }
                    _ => {}
                }

                if depth < ignoredepth {
                    if let Some(writable) = e.as_writer_event() {
                        writer.write(writable).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

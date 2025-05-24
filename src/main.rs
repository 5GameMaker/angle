use std::{
    collections::HashMap,
    fs::{self, File},
    io::{stdin, BufRead, Cursor},
    mem::transmute,
    ops::Deref,
    process::{exit, Command, Stdio},
    slice,
};

use args::parse_args;
use base64::Engine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod args;

// Figura stuff

#[derive(Deserialize)]
pub struct FiguraFileMetadata {
    name: Box<str>,
    authors: Box<str>,
    color: Box<str>,
}

#[derive(Deserialize)]
pub struct FiguraModelBoxDataFace {
    uv: [f32; 4],
    tex: usize,
}

#[derive(Deserialize)]
pub struct FiguraModelBoxData {
    d: FiguraModelBoxDataFace,
    e: FiguraModelBoxDataFace,
    n: FiguraModelBoxDataFace,
    s: FiguraModelBoxDataFace,
    u: FiguraModelBoxDataFace,
    w: FiguraModelBoxDataFace,
}

#[derive(Deserialize)]
pub struct FiguraModelPart {
    name: Box<str>,
    #[serde(default = "Default::default")]
    chld: Vec<FiguraModelPart>,
    #[serde(default = "Default::default")]
    f: [f32; 3],
    #[serde(default = "Default::default")]
    t: [f32; 3],
    #[serde(default = "Default::default")]
    piv: [f32; 3],
    #[serde(default = "Default::default")]
    rot: Option<[f32; 3]>,
    #[serde(default = "Default::default")]
    cube_data: Option<FiguraModelBoxData>,
}

#[derive(Deserialize)]
pub struct FiguraFileTexturesData {
    d: Box<str>,
}

#[derive(Deserialize)]
pub struct FiguraFileTextures {
    src: HashMap<Box<str>, Box<[i8]>>,
    data: Vec<FiguraFileTexturesData>,
}

#[derive(Deserialize)]
pub struct FiguraFile {
    metadata: FiguraFileMetadata,
    models: FiguraModelPart,
    scripts: HashMap<Box<str>, Box<[i8]>>,
    textures: FiguraFileTextures,
}

// JSON

#[derive(Serialize)]
pub struct AvatarJson {
    name: Box<str>,
    color: Box<str>,
    authors: Vec<Box<str>>,
}

// Blockbench

#[derive(Serialize)]
pub struct BlockbenchModelMeta {
    format_version: &'static str,
    model_format: &'static str,
}
impl Default for BlockbenchModelMeta {
    fn default() -> Self {
        Self {
            format_version: "4.9",
            model_format: "free",
        }
    }
}

#[derive(Serialize)]
pub struct BlockbenchModelPartFace {
    uv: [f32; 4],
    texture: usize,
}
impl BlockbenchModelPartFace {
    pub fn new(v: &FiguraModelBoxDataFace) -> Self {
        Self {
            uv: v.uv,
            texture: v.tex,
        }
    }
}

#[derive(Serialize)]
pub struct BlockbenchModelPartFaces {
    north: BlockbenchModelPartFace,
    east: BlockbenchModelPartFace,
    south: BlockbenchModelPartFace,
    west: BlockbenchModelPartFace,
    up: BlockbenchModelPartFace,
    down: BlockbenchModelPartFace,
}

#[derive(Serialize)]
pub struct BlockbenchModelPart {
    name: Box<str>,
    uuid: Box<str>,
    origin: [f32; 3],
    from: [f32; 3],
    to: [f32; 3],
    rotation: Option<[f32; 3]>,
    faces: BlockbenchModelPartFaces,
}

#[derive(Serialize)]
#[serde(untagged)]
enum BlockbenchModelGroupChild {
    Part(Box<str>),
    Group(BlockbenchModelGroup),
}

#[derive(Serialize)]
pub struct BlockbenchModelGroup {
    name: Box<str>,
    uuid: Box<str>,
    origin: [f32; 3],
    #[serde(rename = "isOpen")]
    is_open: bool,
    #[serde(default = "Default::default")]
    children: Vec<BlockbenchModelGroupChild>,
}

#[derive(Serialize)]
pub struct BlockbenchModelTexture {
    uuid: Box<str>,
    name: Box<str>,
    source: Box<str>,
    folder: &'static str,
    id: Box<str>,
    relative_path: Box<str>,
    path: Box<str>,
    width: u32,
    uv_width: u32,
    height: u32,
    uv_height: u32,
}

#[derive(Serialize)]
pub struct BlockbenchModel {
    meta: BlockbenchModelMeta,
    elements: Vec<BlockbenchModelPart>,
    outliner: Vec<BlockbenchModelGroupChild>,
    textures: Vec<BlockbenchModelTexture>,
}

enum WalkStage<'a, Item> {
    Enter(&'a Item),
    Leave,
}

struct Walk<'a, Item, I, F>
where
    Item: 'a,
    I: Iterator<Item = &'a Item>,
    F: FnMut(&'a Item) -> I,
{
    start: Option<&'a Item>,
    layers: Vec<(&'a Item, I)>,
    fun: F,
}
impl<'a, Item, I, F> Walk<'a, Item, I, F>
where
    Item: 'a,
    I: Iterator<Item = &'a Item>,
    F: FnMut(&'a Item) -> I,
{
    pub fn new(item: &'a Item, fun: F) -> Self {
        Self {
            start: Some(item),
            fun,
            layers: vec![],
        }
    }
}
impl<'a, Item, I, F> Iterator for Walk<'a, Item, I, F>
where
    Item: 'a,
    I: Iterator<Item = &'a Item>,
    F: FnMut(&'a Item) -> I,
{
    type Item = WalkStage<'a, Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.start.take() {
            self.layers.push((y, (self.fun)(y)));
            return Some(WalkStage::Enter(y));
        }
        let mut x = self.layers.pop()?;
        match x.1.next() {
            Some(y) => {
                self.layers.push(x);
                self.layers.push((y, (self.fun)(y)));
                Some(WalkStage::Enter(y))
            }
            None => Some(WalkStage::Leave),
        }
    }
}

fn main() {
    let args = parse_args();

    match fs::create_dir_all(&args.outdir) {
        Ok(_) => match fs::read_dir(&args.outdir) {
            Ok(mut x) => match x.next() {
                None => (),
                Some(Ok(_)) => {
                    eprintln!("/!\\ Directory is not empty! /!\\");
                    eprintln!("Files will be overridden upon collision");
                    eprintln!("Continue?");
                    eprintln!();
                    eprint!("[y/n] > ");

                    let mut s = String::new();
                    let s = match stdin().lock().read_line(&mut s) {
                        Ok(l) => &s[0..l],
                        Err(why) => panic!("{why}"),
                    };

                    if !s.to_ascii_lowercase().contains('y') {
                        eprintln!("Aborted.");
                        exit(1);
                    }
                }
                Some(Err(why)) => {
                    eprintln!("/!\\ Cannot read directory: {why}");
                }
            },
            Err(why) => {
                eprintln!("/!\\ Cannot read directory: {why}");
            }
        },
        Err(why) => {
            eprintln!("Failed to create directory: {why}");
            eprintln!("Exiting.");
            exit(1);
        }
    }

    let file = match File::open(&args.file) {
        Ok(x) => x,
        Err(why) => {
            eprintln!("Failed to open file: {why}");
            exit(1);
        }
    };
    let file: FiguraFile = match nbt::from_gzip_reader(file) {
        Ok(x) => x,
        Err(why) => {
            eprintln!("Failed to parse nbt: {why}");
            exit(1);
        }
    };

    let mut detected_failure = false;

    if let Err(why) = serde_json::to_writer_pretty(
        match File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(args.outdir.join("avatar.json"))
        {
            Ok(x) => x,
            Err(why) => {
                eprintln!("Failed to save avatar.json: {why}");
                exit(1);
            }
        },
        &AvatarJson {
            name: file.metadata.name,
            color: file.metadata.color,
            authors: vec![file.metadata.authors],
        },
    ) {
        eprintln!("Failed to save avatar.json: {why}");
        detected_failure = true;
    }

    for (file, content) in &file.scripts {
        let content = unsafe {
            slice::from_raw_parts(content.as_ptr() as *const _ as *const u8, content.len())
        };
        if let Err(why) = fs::write(args.outdir.join(format!("{file}.lua")), content) {
            eprintln!("Failed to save {file}.lua: {why}");
            detected_failure = true;
        }
    }

    if !file.scripts.is_empty() {
        match Command::new("stylua")
            .args(file.scripts.iter().map(|x| format!("{}.lua", x.0)))
            .current_dir(&args.outdir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()
        {
            Ok(mut x) => match x.wait() {
                Ok(i) => {
                    if !i.success() {
                        eprintln!("/!\\ Stylua exited with error code {i}");
                    }
                }
                Err(why) => {
                    eprintln!("/!\\ Waiting on stylua failed: {why}");
                }
            },
            Err(_) => {
                eprintln!("Not formatting, make sure 'stylua' can be found in PATH");
            }
        }
    }

    for (file, content) in &file.textures.src {
        let content = unsafe {
            slice::from_raw_parts(content.as_ptr() as *const _ as *const u8, content.len())
        };
        if let Err(why) = fs::write(args.outdir.join(format!("{file}.png")), content) {
            eprintln!("Failed to save {file}.png: {why}");
            detected_failure = true;
        }
    }

    'a: for (i, model) in file.models.chld.into_iter().enumerate() {
        let w = match File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(args.outdir.join(format!("{}.bbmodel", model.name)))
        {
            Ok(x) => x,
            Err(why) => {
                eprintln!("Failed to save {}.bbmodel: {why}", model.name);
                detected_failure = true;
                continue;
            }
        };

        let mut textures = vec![];
        for (name, data) in file
            .textures
            .data
            .iter()
            .map(|x| (&x.d, file.textures.src.get(&x.d)))
        {
            let Some(data) = data else {
                eprintln!(
                    "Failed to save {}.bbmodel: Texture '{name}' is missing from registry",
                    model.name
                );
                detected_failure = true;
                continue 'a;
            };

            let mut decoder = png::Decoder::new(Cursor::new(unsafe {
                transmute::<&Box<[i8]>, &Box<[u8]>>(data)
            }));
            let header = match decoder.read_header_info() {
                Ok(x) => x,
                Err(why) => {
                    eprintln!("Failed to save {}.bbmodel: {why}", model.name);
                    detected_failure = true;
                    continue 'a;
                }
            };

            let size = header.size();

            textures.push(BlockbenchModelTexture {
                name: format!("{name}.png").into(),
                uuid: Uuid::new_v4().as_hyphenated().to_string().into(),
                relative_path: format!("{name}.png").into(),
                path: args
                    .outdir
                    .join(format!("{name}.png"))
                    .canonicalize()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .into(),
                folder: "",
                id: i.to_string().into(),
                source: format!(
                    "data:image/png;base64,{}",
                    base64::engine::general_purpose::STANDARD_NO_PAD.encode({
                        unsafe {
                            let p = data.deref();
                            std::slice::from_raw_parts(p.as_ptr() as *const u8, p.len())
                        }
                    })
                )
                .into(),
                width: size.0,
                uv_width: size.0,
                height: size.1,
                uv_height: size.1,
            });
        }

        let mut outliner = vec![];
        let mut elements = vec![];
        // Safety: It's safe enough as far as my understanding of
        //         this code goes. Borrow checker could not borrow
        //         check this at the time, if it ever would.
        //
        //         Since we are never inserting things and are only
        //         pushing, all references we obtain stay valid for
        //         their entire lifetime.
        unsafe {
            let mut did_root = false;
            let mut is_cube = false;
            let mut groups: Vec<&mut BlockbenchModelGroup> = vec![];
            let groups_ptr = &mut groups as *mut Vec<&mut BlockbenchModelGroup>;
            let gr = || groups_ptr.as_mut().unwrap();
            let outliner_ptr = &mut outliner as *mut Vec<BlockbenchModelGroupChild>;
            let ol = || outliner_ptr.as_mut().unwrap();
            fn ctog(v: &mut BlockbenchModelGroupChild) -> &mut BlockbenchModelGroup {
                if let BlockbenchModelGroupChild::Group(x) = v {
                    x
                } else {
                    unreachable!()
                }
            }
            for s in Walk::new(&model, |x| x.chld.iter()) {
                match s {
                    WalkStage::Enter(x) => {
                        if !did_root {
                            did_root = true;
                            continue;
                        }
                        let uuid: Box<str> = Uuid::new_v4().as_hyphenated().to_string().into();
                        if let Some(data) = &x.cube_data {
                            is_cube = true;
                            if let Some(x) = gr().last_mut() {
                                x.children
                                    .push(BlockbenchModelGroupChild::Part(uuid.clone()));
                            } else {
                                outliner.push(BlockbenchModelGroupChild::Part(uuid.clone()));
                            }
                            let part = BlockbenchModelPart {
                                name: x.name.clone(),
                                uuid,
                                origin: x.piv,
                                from: x.f,
                                to: x.t,
                                rotation: x.rot,
                                faces: BlockbenchModelPartFaces {
                                    up: BlockbenchModelPartFace::new(&data.u),
                                    down: BlockbenchModelPartFace::new(&data.d),
                                    east: BlockbenchModelPartFace::new(&data.e),
                                    west: BlockbenchModelPartFace::new(&data.w),
                                    north: BlockbenchModelPartFace::new(&data.n),
                                    south: BlockbenchModelPartFace::new(&data.s),
                                },
                            };
                            elements.push(part);
                        } else {
                            is_cube = false;
                            let group = BlockbenchModelGroup {
                                name: x.name.clone(),
                                uuid,
                                origin: x.piv,
                                is_open: false,
                                children: vec![],
                            };
                            if let Some(x) = gr().last_mut() {
                                x.children.push(BlockbenchModelGroupChild::Group(group));
                                gr().push(ctog(x.children.last_mut().unwrap()));
                            } else {
                                ol().push(BlockbenchModelGroupChild::Group(group));
                                gr().push(ctog(ol().last_mut().unwrap()));
                            }
                        }
                    }
                    WalkStage::Leave => {
                        if is_cube {
                            is_cube = false;
                            continue;
                        }
                        gr().pop();
                    }
                }
            }
        }
        //      // Safety: It's safe enough as far as my understanding of
        //      //         this code goes. Borrow checker could not borrow
        //      //         check this at the time, if it ever would.
        //      //
        //      //         Since we are never inserting things and are only
        //      //         pushing, all references we obtain stay valid for
        //      //         their entire lifetime.
        //      unsafe {
        //          let mut refs: Vec<&mut BlockbenchModelPart> = vec![];
        //          let refs_ptr = &mut refs as *mut Vec<&mut BlockbenchModelPart>;
        //          let outliner_ptr = &mut outliner as *mut Option<_>;
        //          for s in Walk::new(&model, |x| x.chld.iter()) {
        //              match s {
        //                  WalkStage::Enter(item) => {
        //                      let part = BlockbenchModelPart {
        //                          name: item.name.clone(),
        //                          uuid: Uuid::new_v4().as_hyphenated().to_string().into(),
        //                          origin: item.piv,
        //                          is_open: true,
        //                          children: vec![],
        //                      };
        //                      let ptr = match refs_ptr.as_mut().unwrap().last_mut() {
        //                          Some(x) => {
        //                              x.children.push(part);
        //                              x.children.last_mut().unwrap()
        //                          }
        //                          None => {
        //                              assert!(outliner_ptr.as_mut().unwrap().replace(part).is_none());
        //                              outliner_ptr.as_mut().unwrap().as_mut().unwrap()
        //                          }
        //                      };
        //                      refs_ptr.as_mut().unwrap().push(ptr);
        //                  }
        //                  WalkStage::Leave => {
        //                      assert!(refs_ptr.as_mut().unwrap().pop().is_some());
        //                  }
        //              }
        //          }
        //      }

        if let Err(why) = serde_json::to_writer(
            w,
            &BlockbenchModel {
                meta: Default::default(),
                elements,
                outliner,
                textures,
            },
        ) {
            eprintln!("Failed to save {}.bbmodel: {why}", model.name);
            detected_failure = true;
        }
    }

    if detected_failure {
        exit(1);
    }
}

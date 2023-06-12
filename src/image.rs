use std::collections::{HashSet, LinkedList};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io, path::Path};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use sha2::{self, Digest};
// use generic_array::*;
use core::fmt::LowerHex;

use crate::{common::Constellation, owner::Owner};

/*
# Explanation
Each owner and program can have an accompanying logo. 
For any image, one source image is kept which is converted to a chosen destination format e.g. (png 128x128).
The state of which source images have been converted to what destination images is serialized and kept in a file.
On compile, newly added images are detected and converted, changed images are detected by hash and reconverted.

# Adding source images

You may add a source image of any owner/program of any quality at the prescribed location. (see below)

You may replace a source image with a higher quality version, this is called "trumping", according to the following criteria
1) file format: .svg > .png > .jpg > .webp
2) higher resolution (if applicable)
3) higher compression quality (if applicable)

# Directory structure

Image state is at
    ./res/state/img_state.ron

Source images reside in ./res/img/owner_name/
    e.g. ./res/Erithax/Erithax.svg          for an owner logo
    e.g. ./res/Erithax/Erithaciousui.png    for a program logo

Archival source images can reside in ./res/img/owner_name/archive/ 

Distribution images reside in ./dist/res/img/owner_name/
    e.g. ./dist/res/img/Erithax/Erithax.svg
    e.g. ./dist/res/img/Erithax/Erithaciousrender_64.png
    e.g. ./dist/res/img/Erithax/Erithaciousrender_640.png
*/

/*
DEV NOTES

MAYBE: instead of going persistent, just generate new images on each cargo run, but maybe make it so it only generates the 64x64 images in debug mode?
TODO: replace file extension jpeg -> jpg

# TODO: MAYBE: UPDATE BINARY
A binary with build tasks is built which is a CLI program that can do several tasks:
* TODO: update_ratings: update download numbers, star ratings for all programs
* TODO: update_images: convert and compress new or trumped source images
*/



const SRC_DIR: &'static str = ".\\res\\srcimg\\";
const DST_DIR: &'static str = ".\\res\\static\\img\\";
const STORED_IMS_PATH: &'static str = "./res/state/img_state.ron";
const VALID_SRC_EXTS: [&'static str; 4] = ["svg", "png", "jpg", "webp"];

// svg's are never rasterized
// must delete all contents of ./dist/res/img/ to make apply a change to the destination format
const DST_FORMAT: ImgFormat = ImgFormat {
    ext: ImgExt::Png,
    siz: 128,
};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ImgExt {
    Svg,
    Png,
    Jpg,
    Webp,
}
impl ImgExt {
    fn str(&self) -> &'static str {
        match self {
            ImgExt::Svg => "svg",
            ImgExt::Png => "png",
            ImgExt::Jpg => "jpg",
            ImgExt::Webp => "webp",
        }
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
struct ImgFormat {
    ext: ImgExt,
    siz: u32,
}
impl ImgFormat {
    fn str(&self) -> String {
        return "_".to_owned() + self.siz.to_string().as_str() + "." + self.ext.str()
    }
}

fn containsfilestem(p: &Path, stem: &str) -> bool {
    assert!(p.is_dir());
    for i in fs::read_dir(&p).unwrap() {
        let i = i.unwrap().path();
        if i.file_stem().unwrap() == stem {return true}
    }
    return false
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    name: String, // without postfix
    format: ImgFormat,
    hash: String,
    time_created: SystemTime,
}
impl Image {
    pub fn relpath(&self) -> PathBuf {
        return Path::new(OsStr::new((self.name.clone() + "." + self.format.ext.str()).as_str())).to_path_buf()
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ImageList {
    stuffio: Vec<(String, Vec<Image>, Vec<Image>)>,
}
impl ImageList {
    pub fn storeBlank() {
        let blank_image_list = ImageList {
            stuffio: Vec::new()
        };
        let s = ron::to_string(&blank_image_list).unwrap();
        fs::write(STORED_IMS_PATH, s).expect("was unable to store blank image list");
    }

    pub fn load() -> ImageList {
        let file = fs::read_to_string(STORED_IMS_PATH).expect("img state file not found, did you remove it or change the directory structure?");
        let r = ron::from_str::<ImageList>(&file).unwrap();
        return r
    }

    pub fn store(&self) {
        fs::write(STORED_IMS_PATH, ron::to_string(self).unwrap())
            .expect("was unable to store blank image list");
    }

    pub fn assert_file_dir_valid(c: &Constellation) {
        // check that the ./res/img/ file directory structure is valid
        for imgdir_item in 
            fs::read_dir(SRC_DIR).expect(&format!("Image source directory SRC_DIR = {} could not be opened", SRC_DIR)) 
        {
            let imgdir_item = imgdir_item.unwrap().path();
            assert!(imgdir_item.is_dir(), 
                "item in ./res/img named {} is not a directory", imgdir_item.display());
            assert!(Owner::iter().any(|x| imgdir_item.file_name().unwrap().to_str().unwrap() == x.to_string()), 
                "Directory {} in ./res/img/ does not have the name of a valid owner", imgdir_item.display());
            let owner = Owner::from_str(imgdir_item.file_name().unwrap().to_str().unwrap()).unwrap();
            let valid_block_types = c.get_all_of_owner(owner);
            let mut valid_file_names = c.get_all_block_names_of_owner(owner);
            valid_file_names.insert(owner.to_string());
            let mut used_file_names: HashSet<String> = HashSet::new();

            for ownerdir_item in fs::read_dir(imgdir_item).unwrap() {
                let ownerdir_item = ownerdir_item.unwrap().path();
                if ownerdir_item.file_name().unwrap().to_str().unwrap() == "archive" {continue};
                assert!(ownerdir_item.is_file(), 
                    "dir named {} found in ./res/img/, only a dir named 'archive' is allowed here", ownerdir_item.display());
                let file_stem = ownerdir_item.file_stem().unwrap().to_str().unwrap();
                assert!(valid_file_names.contains(file_stem), "file in dir ./res/img/{} has invalid name {}", owner, file_stem);
                assert!(!used_file_names.contains(file_stem), "multiple files with name {} in ./res/img/{}", file_stem, owner);
                used_file_names.insert(file_stem.to_string());
            }
        }
    }

    pub fn loadFromFilesDir() {
        // check that the ./res/img/ file directory structure is valid
        for imgdir_item in 
            fs::read_dir(SRC_DIR).expect(&format!("Image source directory SRC_DIR = {} could not be opened", SRC_DIR)) 
        {
            let imgdir_item = imgdir_item.unwrap().path();
            

            for ownerdir_item in fs::read_dir(imgdir_item).unwrap() {
                let ownerdir_item = ownerdir_item.unwrap().path();

                let file_stem = ownerdir_item.file_stem().unwrap().to_str().unwrap();

            }
        }
    }

    pub fn assert_self_valid(&self) {
        for o in self.stuffio.iter() {
            assert!(o.1.len() == o.2.len(), "ImageList is not internally consistent for owner {}", o.0);
        }
    }

    pub fn update(&mut self, c: Constellation) {
        self.assert_self_valid();
        Self::assert_file_dir_valid(&c);
        
        // add owner directories and src image files with no entries in self to self.owners and self.src respectively
        for imgdir_item in 
            fs::read_dir(SRC_DIR).expect(&format!("Image source directory SRC_DIR = {} could not be opened", SRC_DIR)) 
        {
            let imgdir_item = imgdir_item.unwrap().path();
            let owner = Owner::from_str(imgdir_item.file_name().unwrap().to_str().unwrap()).unwrap();
            if !self.stuffio.iter().any(|e| e.0 == owner.to_string()) {
                self.stuffio.push((owner.to_string(), vec![], vec![]));
            }

            for ownerdir_item in fs::read_dir(imgdir_item).unwrap() {
                let ownerdir_item: PathBuf = ownerdir_item.unwrap().path();
                if ownerdir_item.file_name().unwrap().to_str().unwrap() == "archive" {continue};
                let file_stem = ownerdir_item.file_stem().unwrap().to_str().unwrap();

                if ! self.stuffio.iter().any(|e| e.1.iter().any(|src_img| src_img.name == file_stem)) {
                    let new_img = Image {
                        name: file_stem.to_string(),
                        format: ImgFormat {ext: ImgExt::Svg, siz: 0},
                        hash: "0".to_string(), // different from actual file hash so it is handled like a changed image 
                        time_created: SystemTime::now(),
                    };
                    let mut inserted = false;
                    for e in &mut self.stuffio {
                        if e.0 == owner.to_string() {
                            e.1.push(new_img.clone());
                            inserted = true;
                            break
                        }
                    }
                    if ! inserted {
                        self.stuffio.push((owner.to_string(), vec![new_img], vec![]));
                    }

                }
            }
        }

        // only keep owners, src_images and dst_images in self which have a corresponding owner directory / image file
        self.stuffio.retain_mut(|e|{

            let src_owner_path = Path::new(SRC_DIR).join(e.0.clone());
            let dst_owner_path: PathBuf = Path::new(DST_DIR).join(e.0.clone());
            if ! Owner::iter().any(|x: Owner| e.0 == x.to_string()) || ! src_owner_path.exists(){
                dbg!("owner {} was removed since last compilation", e.0.clone());
                dbg!("removing directories is currently disabled");
                // if src_owner_path.exists() {
                //     fs::remove_dir_all(src_owner_path);
                // }
                // if dst_owner_path.exists() {
                //     fs::remove_dir_all(dst_owner_path);
                // }
                return false
            }
            
            let mut src_valid_names = c.get_all_block_names_of_owner(Owner::from_str(&e.0).unwrap());
            src_valid_names.insert(e.0.clone());

            e.1.retain(|src_img| {
                return src_valid_names.contains(&src_img.name) && 
                containsfilestem(&src_owner_path, &src_img.name)    //src_owner_path.join(src_img.name.clone()).exists()
            });
            e.2.retain(|dst_img| {
                return e.1.iter().any(|src_img| src_img.name == dst_img.name)
            });

            return e.1.len() > 0
        });

        // convert src images when file hash != entry hash or when dst image not present   and update self.src and self.dst accordingly
        for e in &mut self.stuffio {
            let src_owner_path = Path::new(SRC_DIR).join(e.0.clone());
            let dst_owner_path = Path::new(DST_DIR).join(e.0.clone());
            
            if ! dst_owner_path.exists() {
                fs::create_dir(&dst_owner_path).unwrap();
            }
           
            for j in 0..e.1.len() {
                let mut src_img = &mut e.1[j];
                let src_img_path = src_owner_path.join(src_img.relpath());
                let dst_img_path = dst_owner_path.join(src_img.relpath()); // may or may not exist right now
            
                assert!(src_img_path.exists(), "src_img_path {} does not exist", src_img_path.display());
                
                let mut hasher = sha2::Sha256::new();
                let mut file = fs::File::open(&src_img_path).unwrap();
        
                let bytes_written = io::copy(&mut file, &mut hasher).unwrap();
                let hash_bytes = hasher.finalize();

                if format!("{:x}", hash_bytes) != src_img.hash || !dst_img_path.exists() {
                    // convert
                    src_img.hash = format!("{:x}", hash_bytes).to_string();
                    fs::remove_file(&dst_img_path).unwrap();
                    e.2.retain(|dst_img: &Image|{return dst_img.name != src_img.name});
                    dbg!(format!("converting image {}", &src_img_path.display()));
                    if src_img_path.extension().unwrap() == "svg" {
                        fs::copy(src_img_path, dst_img_path).unwrap();
                        e.2.push(Image {
                            name: src_img.name.clone(),
                            format: src_img.format.clone(),
                            hash: src_img.hash.clone(),
                            time_created: SystemTime::now(),
                        });
                    } else if VALID_SRC_EXTS.contains(&src_img_path.extension().unwrap().to_str().unwrap()) {
                        let img: image::DynamicImage = image::open(src_img_path).unwrap();
                        let img = img.resize(DST_FORMAT.siz, DST_FORMAT.siz, image::imageops::FilterType::Triangle);
                        img.save(
                            dst_img_path.parent().unwrap().to_str().unwrap().to_owned() + 
                            "/" + dst_img_path.file_stem().unwrap().to_str().unwrap() + 
                            "." + DST_FORMAT.ext.str()
                        ).unwrap();
                        e.2.push(Image {
                            name: src_img.name.clone(),
                            format: DST_FORMAT,
                            hash: "1".to_string(), // unimportant, not used
                            time_created: SystemTime::now(),
                        })
                        
                    }
                }
            }
        }
        
        self.assert_self_valid();
        self.store();

    }
}




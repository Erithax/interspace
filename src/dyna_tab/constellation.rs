
use serde;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::STARS_FILE_PATH;
use crate::dyna_tab::component::*;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::StarCache;

use crate::dyna_tab::stage::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::components::{langbridge::*, ui::*, layout::*, paint::*, raster::*, intergfx::*, gfxapi::*, platform::*};
use crate::dyna_tab::tree::*;



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] 
pub struct Constellation {
    pub comps: Vec<Componentus>,
}

const DOLLAR_ID: ComponentId = 9901;
const ASTERISK_ID: ComponentId = 9902;

impl Constellation {

    pub fn generate_skeleton() -> Constellation {
        // info!("generating tree");
        let mut components: Vec<Componentus> = Vec::new();
        
        let mut str_paths: Vec<Vec<Vec<&'static str>>> = Vec::new();
        let mut id_paths: Vec<Vec<Vec<ComponentId>>>;
    

        let r: Vec<(ComponentType, Box<dyn Blockify>)> = vec![
            (ComponentType::Langbridge, Box::new(Langbridge{})),
            (ComponentType::Ui, Box::new(Ui{})),
            (ComponentType::Layout, Box::new(Layout{})),
            (ComponentType::Paint, Box::new(Paint{})),
            (ComponentType::Raster, Box::new(Raster{})),
            (ComponentType::Gfxapi, Box::new(Gfxapi{})),
            (ComponentType::Intergfx, Box::new(Intergfx{})),
            (ComponentType::Platform, Box::new(Platform{})),
        ];

        for (ct, b) in r {
            for (str_ident, info, extra, paths) in b.add_all() {
                assert!(info.source.is_none() || info.source.as_ref().unwrap().stars.is_none());
                components.push(
                    Componentus {
                        str_id: str_ident.to_owned(),
                        id: components.len(),
                        typ: ct,
                        info: info,
                        extra: extra,
                        lefToRigTree: Tree::new_blank(0),
                        rigToLefTree: Tree::new_blank(0),
                        splitdowntree: Tree::new_blank(0),
                        splituptree: Tree::new_blank(0),
                    }
                );
                str_paths.push(paths);
            }
        }

    

        id_paths = str_paths.iter().enumerate().map(
            |(comp_id, str_paths)| 
                str_paths.iter().map(
                    |str_path| 
                        str_path.iter().map(
                            |s| 
                                components.iter().position(|compy| &compy.str_id == s)
                                .or( if s == &"$" {Some(comp_id)} else {None})
                                .or( if s == &"*" {Some(ASTERISK_ID)} else {None})
                                .expect(&format!("comp {} has messed up string paths {:?}", comp_id, str_paths))
                        ).collect()
                ).collect()
        ).collect();

        assert!(components.iter().all(|comp| comp.str_id != "$"));
        assert!(id_paths.iter().all(|id_paths| id_paths.iter().all(|id_path| !id_path.contains(&DOLLAR_ID))));

        id_paths.sort_unstable();
        id_paths.dedup();
        
    
        let mut c = Constellation {
            comps: components,
        };
    

        let expanded_flattened_id_paths = c.expand_paths(id_paths);
        c.gen_trees(&expanded_flattened_id_paths);

        let c2 = c.clone();

        for comp in c.comps.iter_mut() {
            comp.lefToRigTree.set_stage_indices(&c2);
            comp.lefToRigTree.set_grid_lines(&c2);
            comp.lefToRigTree.set_breadth_intervals(&c2);
            comp.rigToLefTree.set_stage_indices(&c2);
            comp.rigToLefTree.set_grid_lines(&c2);
            comp.rigToLefTree.set_breadth_intervals(&c2);
            comp.splitdowntree.set_stage_indices(&c2);
            comp.splitdowntree.set_grid_lines(&c2);
            comp.splitdowntree.set_breadth_intervals(&c2);
            comp.splituptree.set_stage_indices(&c2);
            comp.splituptree.set_grid_lines(&c2);
            comp.splituptree.set_breadth_intervals(&c2);
        }



        return c
    }
    
    pub fn incorporate_stars(&mut self) {
        let stars: Vec<StarCache> = ron::from_str(include_str!("./../../res/state/stars.ron")).unwrap();

        for (comp_id, comp) in self.comps.iter_mut().enumerate() {
            assert!(comp.info.source.is_none() || comp.info.source.as_ref().unwrap().stars.is_none());
            match comp.info.source {
                Some(ref mut r) => {
                    r.stars = stars.iter().find(|star_cache| star_cache.comp_str_id == comp.str_id)
                        .expect(&format!("did not find star cache of component `{}`. Try running `cargo run --bin regen_data` to update cache", comp.str_id))
                        .repo.clone()
                        .expect(&format!("star cache state invalid. Try running `cargo run --bin regen_data` to update cache"))
                        .stars;
                },
                None => {}
            }
        }

    }

    pub fn noop(&self) {}

    pub fn get_comp(&self, comp_ind: ComponentId) -> &Componentus {
        // assert!(self.comps.contains_key(bt), "called Constellation.getNode() with bt = {}", bt);
        return &self.comps[comp_ind]
    }

    pub fn get_comp_by_str_id(&self, str_id: &str) -> &Componentus {
        assert!(self.comps.iter().any(|comp| comp.str_id == str_id));
        return self.comps.iter().find(|comp| comp.str_id == str_id).unwrap();
    }

    pub fn get_all_ids_of_comp_typ(&self, comp_typ: ComponentType) -> Vec<ComponentId> {
        let mut res = Vec::new();
        for (i, comp) in self.comps.iter().enumerate() {
            if comp.typ == comp_typ {
                res.push(i);
            }
        }
        return res
    }

    pub fn get_all_comps_of_comp_typ(&self, comp_typ: ComponentType) -> Vec<&Componentus> {
        let mut res = Vec::new();
        for  comp in self.comps.iter() {
            if comp.typ == comp_typ {
                res.push(comp);
            }
        }
        return res
    }

    pub fn get_all_ids_of_stage(&self, st: Stage) -> Vec<ComponentId> {
        let mut res: Vec<ComponentId> = Vec::new();
        for (id, comp) in self.comps.iter().enumerate() {
            if Stage::from_comp_typ(comp.typ) == st {
                res.push(id);
            }
        }
        return res
    }

    pub fn get_all_comps_of_stage(&self, st: Stage) -> Vec<&Componentus> {
        let mut res = Vec::new();
        for comp in self.comps.iter() {
            if Stage::from_comp_typ(comp.typ) == st {
                res.push(comp);
            }
        }
        return res
    }

    pub fn get_all_ids_of_owner(&self, owner: Owner) -> Vec<ComponentId> {
        return self.comps.iter().clone().enumerate().filter(
            |(_, comp)| comp.info.owner == owner
        ).map(|(id, _)| id).collect()
    }

    pub fn get_all_comps_of_owner(&self, owner: Owner) -> Vec<&Componentus> {
        return self.comps.iter().clone().enumerate().filter(
            |(_, comp)| comp.info.owner == owner
        ).map(|(_, comp)| comp).collect()
    }

    fn expand_paths(&self, id_paths: Vec<Vec<Vec<ComponentId>>>) -> Vec<Vec<ComponentId>>{
        let mut new_paths = Vec::new();
        for (id, _) in self.comps.iter().enumerate() {
            new_paths.append(&mut self.get_expanded_paths_from(&id_paths, id));
        }

        assert!(new_paths.iter().all(|path| path.iter().all(|i| i < &self.comps.len())), "{:?}, self.comps.len() = {}", new_paths.iter().find(|path| !path.iter().any(|i| i < &self.comps.len())), self.comps.len());

        // delete any path with same block twice or more
        new_paths.retain(
            |path| path.iter().enumerate().all(
                |(i1, e1)| path.iter().enumerate().all(
                    |(i2, e2)| e1 != e2 || i1 == i2
                )
            )
        );
        
        // delete any duplicate paths
        let mut uniques = Vec::new();
        new_paths.retain(|path| {
            let is_dupe = uniques.contains(path);
            uniques.push(path.clone()); 
            return !is_dupe
        });

        return new_paths;

    }

    pub fn get_expanded_paths_from(&self, id_paths: &Vec<Vec<Vec<ComponentId>>>, comp_id: ComponentId) -> Vec<Vec<ComponentId>> {
        let mut new_paths: Vec<Vec<BlockId>> = Vec::new();

        for path in id_paths[comp_id].iter() {
            assert!(path.len() > 0, "every path length must be > 0, found {:?}", path);
            let mut base_path = path.clone();
            if *path.last().unwrap() == ASTERISK_ID {
                assert!(path.len() > 2, "there must be a node in front of BlockType::ALL in a branch");
                base_path.remove(base_path.len()-1);
                let last_node = base_path.last().unwrap();
                assert!(*last_node != ASTERISK_ID, "invalid branch contains consecutive ALL {:?}", path);
                for sub_path in self.get_expanded_paths_from(id_paths, *last_node) {
                    let mut new_path = base_path.clone();
                    new_path.append(&mut (&sub_path[1..]).to_vec());
                    new_paths.push(new_path);
                }
            } else {
                new_paths.push(base_path);
            }
            
        }
        return new_paths;
    }

    fn gen_trees(&mut self, paths: &Vec<Vec<ComponentId>>) {
        let coperino = self.clone();
        for (id, comp) in self.comps.iter_mut().enumerate() {
            (comp.lefToRigTree, comp.rigToLefTree) = Tree::full_trees_from_paths_of(&coperino, &paths, id); 
            (comp.splitdowntree, comp.splituptree) = Tree::split_trees_from_paths_of(&paths, id);
        }
    }

    pub fn store(&self) {
        let mut f = std::fs::File::create("./res/state/constellation.bincode").unwrap();
        bincode::serialize_into(&mut f, &self).expect("unable to store constellation");
        // std::fs::write("./res/state/constellation.ron", ron::to_string(self).unwrap())
        //     .expect("was unable to store constellation");
    }

    pub fn load() -> Self {
        bincode::deserialize::<Constellation>(include_bytes!("./../../res/state/constellation.bincode")).unwrap()
        // ron::from_str::<Constellation>(&std::fs::read_to_string("./res/state/constellation.ron").unwrap()).unwrap()
    }
}


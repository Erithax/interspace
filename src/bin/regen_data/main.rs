
use interspace::dyna_tab::constellation::Constellation;

pub mod stars;
use stars::update_stars_cache;


pub fn main() {
    update_stars_cache();

    let mut c = Constellation::generate_skeleton();
    c.incorporate_stars();
    c.store();
}
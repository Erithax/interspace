
use ui_overview::dyna_tab::component::Repo;
use ui_overview::dyna_tab::constellation::Constellation;

use ui_overview::STARS_FILE_PATH;
use ui_overview::StarCache;





#[derive(serde::Deserialize)]
struct StarsResponse {
    stargazers_count: u64,
}

pub fn update_repo(repo: &mut Option<Repo>, new_repo_base: &Option<Repo>) {

    let new_repo = match (repo.clone(), new_repo_base) {
        (None, None) => {None},
        (Some(_), None) => {None},
        (_, Some(r)) => {
            let mut temp = r.clone();
            println!("is github?");
            temp.stars = if temp.url.contains("github.com") {
                println!("making request {}", temp.url.replace("github.com", "api.github.com/repos"));

                static APP_USER_AGENT: &str = concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                );
                
                let client = reqwest::blocking::Client::builder()
                    .user_agent(APP_USER_AGENT)
                    .build()
                    .unwrap();
                if let Ok(req) = client.get(temp.url.replace("github.com", "api.github.com/repos")).send() {
                    println!("got Ok response");
                    println!("{:?}", req);
                    if let Ok(res) = req.json::<StarsResponse>() {
                        println!("json did YES parse!");
                        Some(res.stargazers_count)
                    } else {
                        println!("json did NOT parse!");
                        None
                    }
                } else {
                    println!("got Err response");
                    None
                }
            } else {
                None
            };
            Some(temp)
        }
    };
    
    *repo = new_repo;
}

pub fn update_stars_cache() {
    
    let file_string = std::fs::read_to_string(STARS_FILE_PATH);

    let mut res: Vec<StarCache> = match file_string {
        Err(_) => {
            println!("COULD NOT READ {}, CREATING NEW", STARS_FILE_PATH);
            Vec::new()
        },
        Ok(s) => {
            match ron::from_str(&s) {
                Ok(s) => {s},
                Err(_) => {Vec::new()}
            }
        }
    };   

    for (comp_id, comp) in Constellation::generate_skeleton().comps.iter().enumerate() {
        let prev_star_info_i = res.iter().position(|si: &StarCache| si.comp_str_id == comp.str_id);
        match prev_star_info_i {
            Some(i) => {
                if
                    chrono::Utc::now().signed_duration_since(res[i].update_time).num_seconds() > 60 * 60 * 24 ||
                    match (&res[i].repo, comp.info.source.clone()) {
                        (None, Some(_)) => true,
                        (Some(_), None) => true,
                        (Some(r0), Some(r1)) => {r0.url != r1.url},
                        (None, None) => false,
                    }
                {
                    let repo_base = &comp.info.source;
                    update_repo(&mut res[i].repo, repo_base);
                }
            },
            None => {
                res.push(StarCache{
                    update_time: chrono::Utc::now(),
                    comp_id: comp_id,
                    comp_str_id: comp.str_id.to_owned(),
                    repo: None,
                });
                let repo_base = &comp.info.source;
                update_repo(&mut res.last_mut().unwrap().repo, repo_base);
            }
        }
    }
    std::fs::write(
        STARS_FILE_PATH, 
        ron::ser::to_string_pretty(
            &res, 
            ron::ser::PrettyConfig::default()
        ).unwrap()).expect("failed to write stars to file");
}
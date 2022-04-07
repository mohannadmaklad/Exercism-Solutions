use std::collections::HashMap;

const TEAM_COL_WIDTH: u8 = 30;
const MP_INDEX: usize = 0;
const W_INDEX: usize = 1;
const D_INDEX: usize = 2;
const L_INDEX: usize = 3;
const P_INDEX: usize = 4;

pub fn tally(match_results: &str) -> String {
    let mut header = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut database: HashMap<String, Vec<u8>> = HashMap::new();

    let res = match_results.split('\n');
    for r in res {
        handle_one_result(r, &mut database);
    }

    let mut results: Vec<(String, Vec<u8>)> = database
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    //Sort the results
    results.sort_by(|a, b| {
        if b.1[P_INDEX] != a.1[P_INDEX] {
            b.1[P_INDEX].cmp(&a.1[P_INDEX])
        } else {
            a.0.to_lowercase().cmp(&b.0.to_lowercase())
        }
    });

    //Update the header
    for r in results {
        header = header + &render_result(&r.0.clone(), &r.1);
    }
    header
}

fn render_result(team: &str, res: &Vec<u8>) -> String {
    let spaces: String = (0..(TEAM_COL_WIDTH - team.len() as u8) + 1)
        .map(|_| ' ')
        .collect();

    return format!(
        "\n{team}{spaces}|  {} |  {} |  {} |  {} |  {}",
        res[MP_INDEX], res[W_INDEX], res[D_INDEX], res[L_INDEX], res[P_INDEX]
    );
}

//Writes team result into the hash map
fn set_team_res(team: &str, result: &str, db: &mut HashMap<String, Vec<u8>>) {
    match db.get_mut(team) {
        None => {
            db.insert(team.to_string(), vec![0, 0, 0, 0, 0].clone());
        }
        _ => {}
    }
    update_team_res(db.get_mut(team).unwrap(), result);
}

fn handle_one_result(res: &str, db: &mut HashMap<String, Vec<u8>>) {
    let mut sp = res.split(';');
    let team = sp.next();
    let opp = sp.next();
    let res = sp.next();

    match (team, opp, res) {
        (Some(team), Some(opponent), Some(result)) => match result {
            "win" => {
                set_team_res(team, "win", db);
                set_team_res(opponent, "loss", db);
            }
            "loss" => {
                set_team_res(opponent, "win", db);
                set_team_res(team, "loss", db);
            }
            "draw" => {
                set_team_res(team, result, db);
                set_team_res(opponent, result, db);
            }
            _ => (),
        },
        _ => (),
    }
}

fn update_team_res<'a>(res: &'a mut Vec<u8>, result: &str) -> &'a mut Vec<u8> {
    match result {
        "win" => {
            res[MP_INDEX] = res[MP_INDEX] + 1;
            res[W_INDEX] = res[W_INDEX] + 1;
            res[P_INDEX] = res[P_INDEX] + 3;
        }
        "draw" => {
            res[MP_INDEX] = res[MP_INDEX] + 1;
            res[D_INDEX] = res[D_INDEX] + 1;
            res[P_INDEX] = res[P_INDEX] + 1;
        }
        "loss" => {
            res[MP_INDEX] = res[MP_INDEX] + 1;
            res[L_INDEX] = res[L_INDEX] + 1;
        }
        _ => (),
    }
    res
}

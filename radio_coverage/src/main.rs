use std::collections::{HashSet, HashMap};

fn main() {
    let states_needed = HashSet::from(["mt","wa","or","id","nv","ut","ca","az"]);
    let mut stations = HashMap::new();
    stations.insert("kone", HashSet::from(["id","nv","ut"]));
    stations.insert("ktwo", HashSet::from(["wa","id","mt"]));
    stations.insert("kthree", HashSet::from(["or","nv","ca"]));
    stations.insert("kfour", HashSet::from(["nv","ut"]));
    stations.insert("kfive", HashSet::from(["ca","az"]));

    let best_stations = calc_best_stations(&states_needed, &stations);

    println!("{:?}", best_stations);

}

fn calc_best_stations<'a>(states_needed: &'a HashSet<&str>, stations: &'a HashMap<&str, HashSet<&str>>) -> HashSet<&'a str> {
    let stations = stations.clone();
    let mut states_needed = states_needed.clone();

    let mut final_stations: HashSet<&str> = HashSet::new();

    while !states_needed.is_empty() {
        let mut best_station: Option<&str> = None;
        let mut states_covered: HashSet<&str> = HashSet::new();

        for (station_name, states_for_station) in stations.iter().map(|(k,v)| (*k, v)) {
            let covered = states_for_station.intersection(&states_needed).copied().collect::<HashSet<&str>>();
            if covered.len() > states_covered.len() {
                best_station = Some(station_name);
                states_covered = covered;
            }
        }

        states_needed = states_needed.difference(&states_covered).copied().collect::<HashSet<&str>>();
        final_stations.insert(best_station.unwrap());
    }

    final_stations
}

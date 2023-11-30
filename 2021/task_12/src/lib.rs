use read_file::read_file;
use std::collections::HashMap;

pub fn task_12_1() {
    let mut tunnels: Tunnels = HashMap::new();
    let row_pairs = read_file("./data_files/file12.txt")
        .iter()
        .map(|row| {
            let mut x = row.split(&"-").map(|s| s.to_string());
            (x.next().unwrap(), x.next().unwrap())
        })
        .collect::<Vec<(String, String)>>();
    make_tunnels(&mut tunnels, row_pairs);
    let mut trips: Vec<Vec<String>> = vec![];
    let mut trip: Vec<String> = vec!["start".to_owned()];
    make_trip(&tunnels, &mut trips, &mut trip, "start".to_owned(), true);

    let mut trips_string: Vec<String> = trips.iter().map(|trip| trip.join(",")).collect();
    trips_string.sort();
    trips_string.dedup();
    dbg!(&trips_string);
    dbg!(trips.len());
}

type Tunnels = HashMap<String, Vec<String>>;

fn make_trip(
    tunnels: &Tunnels,
    trips: &mut Vec<Vec<String>>,
    trip: &mut [String],
    key: String,
    mut task_2: bool,
) {
    let connections = tunnels.get(&key).expect("key wasn't found in tunnels");
    let small_cave_limit = task_2 as usize;
    for new_key in connections.clone() {
        let mut trip = trip.to_owned();

        if trip.iter().filter(|&x| x == "c").count() == 1
            && trip.iter().filter(|&x| x == "b").count() == 2
            && new_key == "c"
        {
            dbg!(task_2);
        }

        if trip.contains(&"end".to_owned()) {
            trips.push(trip.to_vec());
            return;
        }

        let new_key_trip_count = trip.iter().filter(|&x| x == &new_key).count();
        if (new_key_trip_count > small_cave_limit) && new_key.to_ascii_lowercase() == new_key {
            if task_2 {
                task_2 = false;
                continue;
            } else {
                continue;
            }
        }

        trip.push(new_key.clone());
        make_trip(tunnels, trips, &mut trip, new_key, task_2);
    }
}

fn make_tunnels(tunnels: &mut Tunnels, row_pairs: Vec<(String, String)>) {
    for (left, right) in row_pairs {
        if right != *"start" && left != *"end" {
            update(tunnels, left.clone(), right.clone());
        }
        if left != *"start" {
            update(tunnels, right, left);
        }
    }
}

fn update(tunnels: &mut HashMap<String, Vec<String>>, cave: String, path: String) {
    if let Some(paths) = tunnels.get_mut(&cave) {
        if !paths.contains(&path) {
            paths.push(path);
        }
    } else {
        tunnels.insert(cave, vec![path]);
    }
}

#[test]
fn task1_check_path_1() {
    let mut tunnels: Tunnels = HashMap::new();
    let row_pairs = vec![
        ("start", "A"),
        ("start", "b"),
        ("A", "c"),
        ("A", "b"),
        ("b", "d"),
        ("A", "end"),
        ("b", "end"),
    ]
    .iter()
    .map(|(x, y)| (x.to_string(), y.to_string()))
    .collect::<Vec<(String, String)>>();

    make_tunnels(&mut tunnels, row_pairs);
    let mut trips: Vec<Vec<String>> = vec![];
    let mut trip: Vec<String> = vec!["start".to_owned()];
    make_trip(&tunnels, &mut trips, &mut trip, "start".to_owned(), false);
    assert_eq!(trips.len(), 10);
}

#[test]
fn task1_check_path_2() {
    let mut tunnels: Tunnels = HashMap::new();
    let row_pairs = [
        ("cz", "end"),
        ("cz", "WR"),
        ("TD", "end"),
        ("TD", "cz"),
        ("start", "UM"),
        ("end", "pz"),
        ("kb", "UM"),
        ("mj", "UM"),
        ("cz", "kb"),
        ("WR", "start"),
        ("WR", "pz"),
        ("kb", "WR"),
        ("TD", "kb"),
        ("mj", "kb"),
        ("TD", "pz"),
        ("UM", "pz"),
        ("kb", "start"),
        ("pz", "mj"),
        ("WX", "cz"),
        ("sp", "WR"),
        ("mj", "WR"),
    ]
    .iter()
    .map(|(x, y)| (x.to_string(), y.to_string()))
    .collect::<Vec<(String, String)>>();
    make_tunnels(&mut tunnels, row_pairs);

    let mut trips: Vec<Vec<String>> = vec![];
    let mut trip: Vec<String> = vec!["start".to_owned()];
    make_trip(&tunnels, &mut trips, &mut trip, "start".to_owned(), false);
    assert_eq!(trips.len(), 3450);
}

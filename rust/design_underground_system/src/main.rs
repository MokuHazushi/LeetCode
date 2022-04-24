// https://leetcode.com/problems/design-underground-system/

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Travel {
    start_station: String,
    end_station: String
}

struct CheckIn {
    station_name: String,
    time: i32,
}

struct UndergroundSystem {
    travel_times: HashMap<Travel, Vec<i32>>,
    check_ins: HashMap<i32, CheckIn>,
}

impl Travel {
    fn new(start_station: String, end_station: String) -> Self {
        Travel {
            start_station: start_station,
            end_station: end_station,
        }
    }
}

impl CheckIn {
    fn new(station_name: String, time: i32) -> Self {
        CheckIn {
            station_name: station_name,
            time: time,
        }
    }
}

impl UndergroundSystem {

    fn new() -> Self {
        UndergroundSystem {
            travel_times: HashMap::new(),
            check_ins: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_ins.insert(id, CheckIn::new(station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let check_in = self.check_ins.remove(&id).unwrap();
        let travel = Travel {
            start_station: check_in.station_name,
            end_station: station_name,
        };

        match self.travel_times.get_mut(&travel) {
            Some(time) => { time.push(t - check_in.time); },
            None => { self.travel_times.insert(travel, vec![t - check_in.time]); },
        }

    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let travel = Travel::new(start_station, end_station);
        let times = self.travel_times.get(&travel).unwrap();
        let mut total_time = 0;

        for time in times {
            total_time += time;
        }

        total_time as f64 / times.len() as f64
    }
}

fn main() {
    println!("Designing an underground railway system!");
    let mut system = UndergroundSystem::new();

    system.check_in(1, String::from("stationA"), 3);
    system.check_out(1, String::from("stationB"), 8);
    println!("Average time from stationA to stationB is {:?}", system.get_average_time(String::from("stationA"), String::from("stationB")));

    system.check_in(1, String::from("stationA"), 10);
    system.check_out(1, String::from("stationB"), 16);
    println!("Average time from stationA to stationB is {:?}", system.get_average_time(String::from("stationA"), String::from("stationB")));

    system.check_in(1, String::from("stationA"), 21);
    system.check_out(1, String::from("stationB"), 30);
    println!("Average time from stationA to stationB is {:?}", system.get_average_time(String::from("stationA"), String::from("stationB")));

}

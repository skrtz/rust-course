mod instructions;
const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "Fall";
    #[allow(warnings)]
    let mut points_scored: i32 = 32;
    points_scored = 35;

    let _event_time: String = "06:00".to_string();
    let event_time: i32 = 6;

    #[allow(unused_variables)]
    let favorite_beverage = "coffee";

    println!("My favorite season is {0}. The team scored {1} points. A touchdown is worth {2} points. event time is at {3}", season, points_scored, TOUCHDOWN_POINTS, event_time);
    
}
extern crate rand;

use rand::Rng;

// struct defining a city and its associated attributes
struct City {
    name: String,
    population: u64,
    latitude: f64,
    longitude: f64,
    landmark: String,
}

// Vector to store the list of cities
let mut cities = Vec::new();

// Adding cities to the vector
cities.push(City {
    name: "Paris".to_string(),
    population: 2141000,
    latitude: 48.8566,
    longitude: 2.3476,
    landmark: "Louvre".to_string(),
});

cities.push(City {
    name: "London".to_string(),
    population: 8538689,
    latitude: 51.5074,
    longitude: 0.1278,
    landmark: "Big Ben".to_string(),
});

cities.push(City {
    name: "New York".to_string(),
    population: 8398748,
    latitude: 40.7128,
    longitude: 74.0060,
    landmark: "Statue of Liberty".to_string(),
});

// Function to print the details of the chosen city
fn print_city(city: &City) {
    println!("Name: {}", city.name);
    println!("Population: {}", city.population);
    println!("Latitude: {}", city.latitude);
    println!("Longitude: {}", city.longitude);
    println!("Landmark: {}", city.landmark);
}


// Function to discover a random city
fn discover_city() {
    // Generate random index
    let index = rand::thread_rng().gen_range(0, cities.len());

    // Get the city at the generated index
    let city = &cities[index];
    println!("You discovered the city of {}!", city.name);

    // Print the details of the discovered city
    print_city(city);
}

fn main() {
    println!("Welcome to Discover Your City!");

    // Discover a random city
    discover_city();
}
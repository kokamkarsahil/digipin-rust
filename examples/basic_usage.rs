use digipin::{get_digipin, get_coordinates_from_digipin, Coordinates};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example coordinates for New Delhi
    let latitude = 28.6139;
    let longitude = 77.2090;
    
    println!("Original coordinates:");
    println!("Latitude: {}, Longitude: {}", latitude, longitude);
    
    // Encode to DIGIPIN
    let digipin = get_digipin(latitude, longitude)?;
    println!("\nEncoded DIGIPIN: {}", digipin);
    
    // Decode back to coordinates
    let decoded_coords = get_coordinates_from_digipin(&digipin)?;
    println!("\nDecoded coordinates:");
    println!("Latitude: {}, Longitude: {}", decoded_coords.latitude, decoded_coords.longitude);
    
    // Calculate precision
    let lat_diff = (decoded_coords.latitude - latitude).abs();
    let lon_diff = (decoded_coords.longitude - longitude).abs();
    println!("\nPrecision:");
    println!("Latitude difference: {:.6}", lat_diff);
    println!("Longitude difference: {:.6}", lon_diff);
    
    // Test with different locations
    println!("\n--- Testing with Mumbai coordinates ---");
    let mumbai_coords = Coordinates::new(19.0760, 72.8777);
    let mumbai_digipin = get_digipin(mumbai_coords.latitude, mumbai_coords.longitude)?;
    let mumbai_decoded = get_coordinates_from_digipin(&mumbai_digipin)?;
    
    println!("Mumbai DIGIPIN: {}", mumbai_digipin);
    println!("Original: {:.4}, {:.4}", mumbai_coords.latitude, mumbai_coords.longitude);
    println!("Decoded:  {:.4}, {:.4}", mumbai_decoded.latitude, mumbai_decoded.longitude);
    
    // Test decoding without hyphens
    println!("\n--- Testing DIGIPIN without hyphens ---");
    let no_hyphens = digipin.replace("-", "");
    let coords_no_hyphens = get_coordinates_from_digipin(&no_hyphens)?;
    println!("DIGIPIN without hyphens: {}", no_hyphens);
    println!("Decoded coordinates: {:.6}, {:.6}", 
             coords_no_hyphens.latitude, coords_no_hyphens.longitude);
    
    Ok(())
}
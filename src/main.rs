use colored::Colorize;

fn main() {
    let dye_type = std::env::args().nth(1).expect("No dye type given");
    let dmw = std::env::args().nth(2).expect("No dry material weight was given");
    let dos = std::env::args().nth(3).expect("No depth of shade was given.");
    let water: String = std::env::args().nth(4).expect("No water amount was given.");


    if dye_type == "wf-acid" {
        calculate_washfast_acid( dmw, dos, water );
    }
}

fn calculate_washfast_acid( dmw: String, dos: String, water: String ) {
    // Set & Figure calculate dmw.
    let dmw_oz: f64 = dmw.parse().unwrap();
    let dmw_lb: f64 = dmw_oz / 16.0;

    // Set the dos.
    let dos_percent: f64 = dos.parse().unwrap();

    // Set and figure the dye weight.
    let dye_oz: f64 = dmw_oz * (dos_percent / 100.0);
    let dye_lb: f64 = dye_oz / 16.0;

    // Convert water to a float and do the math.
    let water_cup: f64 = water.parse().unwrap();
    let water_gal: f64 = water_cup / 16.0;

    // Figure Ammonium Sulfate.
    let ams_oz: f64 = water_cup * 0.01133806;
    let ams_lb: f64 = ams_oz / 16.0;

    // Figure Citric Acid.
    let citric_acid_oz: f64 = water_cup * 0.01133806;
    let citric_acid_lb: f64 = citric_acid_oz / 16.0;

    // Figure Salt.
    let salt_oz: f64 = water_cup * 0.267857143;
    let salt_lb: f64 = salt_oz / 16.0;

    println!("\nDyeing with {0} you will need the following:\n", "Washfast Acid".blue().bold() );
    println!("Dry Material Weight: {dmw_oz} oz --> {dmw_lb} lb");
    println!("     Depth of Shade: {dos_percent}%");
    println!("                Dye: {dye_oz:.4} oz --> {dye_lb:.4} lb");
    println!("              Water: {water_cup} cups --> {water_gal} gals" );
    println!("   Ammonium Sulfate: {ams_oz:.4} oz --> {ams_lb:.4} lb");
    println!("        Citric Acid: {citric_acid_oz:.4} oz --> {citric_acid_lb:.4} lb");
    println!("               Salt: {salt_oz:.4} oz --> {salt_lb:.4} lb");
}

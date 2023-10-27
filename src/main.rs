#![warn(clippy::pedantic, clippy::nursery)]
use std::env::args;

use anyhow::Result;
use colored::Colorize;

const COLOURS: [&str; 12] = [
    "black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white",
    "gold", "silver",
];
const BASE_COLOURS: [&str; 10] = [
    "black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white",
];
const MULTIPLIER_COLOURS: [&str; 10] = [
    "black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "gold", "silver",
];
const TOLERANCE_COLOURS: [&str; 8] = [
    "brown", "red", "green", "blue", "violet", "grey", "gold", "silver",
];
const TEMPERATURE_COEFFICIENT_COLOURS: [&str; 9] = [
    "black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey",
];

fn main() -> Result<()> {
    let bands: Vec<String> = args()
        .skip(1)
        .map(|argument| argument.to_ascii_lowercase())
        .collect();
    if let Some(invalid) = bands.iter().find(|band| !COLOURS.contains(&band.as_str())) {
        return Err(anyhow::anyhow!(
            "Invalid colour (expected one of {}, got {})",
            format!("{COLOURS:?}").green(),
            invalid.red()
        ));
    }
    let number_of_bands = bands.len();
    if !(4..=6).contains(&number_of_bands) {
        return Err(anyhow::anyhow!(
            "Invalid number of bands (expected {}, {} or {}, got {})",
            "4".green(),
            "5".green(),
            "6".green(),
            number_of_bands.to_string().red()
        ));
    }
    let base: u32 = base(number_of_bands, &bands)?;
    let multiplier = multiplier(number_of_bands, &bands)?;
    let tolerance = tolerance(number_of_bands, &bands)?;
    match number_of_bands {
        4 => {
            println!(
                "Resistance: {}Ω ±{}%",
                f64::from(base) * multiplier,
                tolerance
            );
        }
        5 => {
            println!(
                "Resistance: {}Ω ±{}%",
                f64::from(base) * multiplier,
                tolerance
            );
        }
        6 => {
            let temperature_coefficient = temperature_coefficient(&bands)?;

            println!(
                "Resistance: {}Ω ±{}% ({}ppm/K)",
                f64::from(base) * multiplier,
                tolerance,
                temperature_coefficient
            );
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn base(number_of_bands: usize, bands: &[String]) -> Result<u32> {
    let base = match number_of_bands {
        4 => bands.iter().take(2),
        5 | 6 => bands.iter().take(3),
        _ => unreachable!(),
    }
    .enumerate()
    .map(|(index, band)| {
        if !BASE_COLOURS.contains(&band.as_str()) {
            return Err(anyhow::anyhow!(
                "Invalid base colour {} {} {} {}{} (expected one of {}, got {})",
                "(band".dimmed(),
                index + 1,
                "of".dimmed(),
                number_of_bands,
                ")".dimmed(),
                format!("{BASE_COLOURS:?}").green(),
                band.red()
            ));
        }
        Ok(BASE_COLOURS
            .iter()
            .position(|&colour| colour == band)
            .unwrap()
            .to_string())
    })
    .collect::<Result<Vec<String>>>()?;
    Ok(base.join("").parse().unwrap())
}

fn multiplier(number_of_bands: usize, bands: &[String]) -> Result<f64> {
    let multiplier = match number_of_bands {
        4 => bands[2].clone(),
        5 | 6 => bands[3].clone(),
        _ => unreachable!(),
    };
    if !MULTIPLIER_COLOURS.contains(&multiplier.as_str()) {
        return Err(anyhow::anyhow!(
            "Invalid multiplier colour {} {} {} {}{} (expected one of {}, got {})",
            "(band".dimmed(),
            match number_of_bands {
                4 => 3,
                5 | 6 => 4,
                _ => unreachable!(),
            },
            "of".dimmed(),
            number_of_bands,
            ")".dimmed(),
            format!("{MULTIPLIER_COLOURS:?}").green(),
            multiplier.red()
        ));
    }
    Ok(
        match MULTIPLIER_COLOURS
            .iter()
            .position(|colour| colour == &multiplier.as_str())
            .unwrap()
        {
            multiplier @ 0..=7 => 10_f64.powi(i32::try_from(multiplier).unwrap()),
            8 => 0.1,
            9 => 0.01,
            _ => unreachable!(),
        },
    )
}

fn tolerance(number_of_bands: usize, bands: &[String]) -> Result<f64> {
    let tolerance = match number_of_bands {
        4 => bands[3].clone(),
        5 | 6 => bands[4].clone(),
        _ => unreachable!(),
    };
    if !TOLERANCE_COLOURS.contains(&tolerance.as_str()) {
        return Err(anyhow::anyhow!(
            "Invalid tolerance colour {} {} {} {}{} (expected one of {}, got {})",
            "(band".dimmed(),
            match number_of_bands {
                4 => 4,
                5 | 6 => 5,
                _ => unreachable!(),
            },
            "of".dimmed(),
            number_of_bands,
            ")".dimmed(),
            format!("{TOLERANCE_COLOURS:?}").green(),
            tolerance
        ));
    }
    Ok(
        match TOLERANCE_COLOURS
            .iter()
            .position(|colour| colour == &tolerance.as_str())
            .unwrap()
        {
            0 => 1.0,
            1 => 2.0,
            2 => 0.5,
            3 => 0.25,
            4 => 0.1,
            5 => 0.05,
            6 => 5.0,
            7 => 10.0,
            _ => unreachable!(),
        },
    )
}

fn temperature_coefficient(bands: &[String]) -> Result<f32> {
    let temperature_coefficient = bands[5].clone();
    if !TEMPERATURE_COEFFICIENT_COLOURS.contains(&temperature_coefficient.as_str()) {
        return Err(anyhow::anyhow!(
            "Invalid temperature coefficient colour {} (expected one of {}, got {})",
            "(band 6 of 6)".dimmed(),
            format!("{TEMPERATURE_COEFFICIENT_COLOURS:?}").green(),
            temperature_coefficient
        ));
    }
    Ok(
        match TEMPERATURE_COEFFICIENT_COLOURS
            .iter()
            .position(|colour| colour == &temperature_coefficient.as_str())
            .unwrap()
        {
            0 => 100.0,
            1 => 50.0,
            2 => 15.0,
            3 => 25.0,
            _ => unreachable!(),
        },
    )
}

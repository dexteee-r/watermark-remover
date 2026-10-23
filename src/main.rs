//! Point d’entrée du programme watermark‑remover
//! 
//! Ce fichier appelle les fonctions du module `lib` et gère
//! les arguments CLI simples (entrée, watermark, sortie).

use eframe::{egui, epi};
use watermark_remover::lib::{clean_watermark, load_text, write_output};

/// Application de l’interface graphique
struct WatermarkApp {
    input_path: String,
    watermark_pattern: String,
    output_path: String,
}

impl epi::App for WatermarkApp {
    fn name(&self) -> &str {
        "Watermark Remover GUI"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Watermark Remover");

            ui.horizontal(|ui| {
                ui.label("Input file:");
                ui.text_edit_singleline(&mut self.input_path);
            });

            ui.horizontal(|ui| {
                ui.label("Watermark pattern:");
                ui.text_edit_singleline(&mut self.watermark_pattern);
            });

            ui.horizontal(|ui| {
                ui.label("Output file:");
                ui.text_edit_singleline(&mut self.output_path);
            });

            if ui.button("Remove watermark").clicked() {
                let raw = load_text(&self.input_path).expect("Failed to load input");
                let cleaned = clean_watermark(&raw, &self.watermark_pattern);
                write_output(&cleaned, &self.output_path).expect("Failed to write output");
                ui.label("✅ Done!");
            }
        });
    }
}

fn main() {
    let app = WatermarkApp {
        input_path: String::from("input.txt"),
        watermark_pattern: String::from("GPT‑4,"),
        output_path: String::from("output.txt"),
    };
    eframe::run_native(
        Box::new(app),
        eframe::NativeOptions::default(),
    );
}

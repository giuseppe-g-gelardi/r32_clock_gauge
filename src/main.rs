use eframe::egui;

#[derive(Clone, Copy)]
enum Screen {
    Boost,
    OilTemp,
    OilPressure,
    FuelPressure,
    CoolantTemp,
    Afr,
    IgnitionTiming,
    FuelDuty,
    Ethanol,
}

impl Screen {
    fn next(self) -> Screen {
        use Screen::*;
        match self {
            Boost => OilTemp,
            OilTemp => OilPressure,
            OilPressure => FuelPressure,
            FuelPressure => CoolantTemp,
            CoolantTemp => Afr,
            Afr => IgnitionTiming,
            IgnitionTiming => FuelDuty,
            FuelDuty => Ethanol,
            Ethanol => Boost, // wrap around
        }
    }

    fn prev(self) -> Screen {
        use Screen::*;
        match self {
            Boost => Ethanol,
            OilTemp => Boost,
            OilPressure => OilTemp,
            FuelPressure => OilPressure,
            CoolantTemp => FuelPressure,
            Afr => CoolantTemp,
            IgnitionTiming => Afr,
            FuelDuty => IgnitionTiming,
            Ethanol => FuelDuty,
        }
    }
}

struct AppState {
    screen: Screen,
    data: EngineData,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time = ctx.input(|i| i.time as f32);
        self.data = generate_mock_data(time);
        // --- KEY INPUTS FOR SCREEN SWITCHING ---
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.screen = self.screen.next();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            self.screen = self.screen.prev();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let text = match self.screen {
                        Screen::Boost => format!("Boost: {:.1} psi", self.data.boost_psi),
                        Screen::OilTemp => format!("Oil Temp: {:.0} °C", self.data.oil_temp_c),
                        Screen::OilPressure => format!("Oil Pressure: {:.1} bar", self.data.oil_pressure_bar),
                        Screen::FuelPressure => format!("Fuel Pressure: {:.1} bar", self.data.fuel_pressure_bar),
                        Screen::CoolantTemp => format!("Coolant: {:.0} °C", self.data.coolant_temp_c),
                        Screen::Afr => format!("AFR: {:.1}", self.data.afr),
                        Screen::IgnitionTiming => format!("Timing: {:.1}°", self.data.ignition_timing_deg),
                        Screen::FuelDuty => format!("Duty: {:.0}%", self.data.fuel_duty_percent),
                        Screen::Ethanol => format!("Ethanol: {:.0}%", self.data.ethanol_percent),
                    };

                    // Large centered display text
                    ui.label(egui::RichText::new(text).size(42.0));
                },
            );
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(320.0, 240.0))
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "R32 Display",
        options,
        Box::new(|_cc| {
            Box::new(AppState {
                screen: Screen::Boost,
                data: generate_mock_data(0.0),
            })
        }),
    )
}

#[derive(Debug, Clone)]
struct EngineData {
    boost_psi: f32,
    oil_temp_c: f32,
    oil_pressure_bar: f32,
    fuel_pressure_bar: f32,
    coolant_temp_c: f32,
    afr: f32,
    ignition_timing_deg: f32,
    fuel_duty_percent: f32,
    ethanol_percent: f32,
}

fn generate_mock_data(t: f32) -> EngineData {
    EngineData {
        boost_psi: 12.0 + (t * 0.5).sin() * 6.0, // -6 → +6 psi variation
        oil_temp_c: 80.0 + (t * 0.1).sin() * 3.0,
        oil_pressure_bar: 3.8 + (t * 0.7).sin() * 0.5,
        fuel_pressure_bar: 3.5 + (t * 0.3).sin() * 0.3,
        coolant_temp_c: 82.0 + (t * 0.03).sin() * 1.0,
        afr: 11.8 + (t * 0.2).sin() * 0.4,
        ignition_timing_deg: 18.0 + (t * 0.5).sin() * 4.0,
        fuel_duty_percent: 45.0 + (t * 0.8).sin() * 20.0,
        ethanol_percent: 70.0,
    }
}

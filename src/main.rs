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

    // These will later come from CAN data
    boost: f32,
    oil_temp: f32,
    oil_press: f32,
    fuel_press: f32,
    coolant: f32,
    afr: f32,
    ign_timing: f32,
    duty: f32,
    ethanol: f32,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- KEY INPUTS FOR SCREEN SWITCHING ---
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.screen = self.screen.next();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            self.screen = self.screen.prev();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.vertical_centered(
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    // ui.add_space(40.0);

                    let text = match self.screen {
                        Screen::Boost => format!("Boost: {:.1} psi", self.boost),

                        Screen::OilTemp => format!("Oil Temp: {:.0} °C", self.oil_temp),

                        Screen::OilPressure => format!("Oil Pressure: {:.1} bar", self.oil_press),

                        Screen::FuelPressure => {
                            format!("Fuel Pressure: {:.1} bar", self.fuel_press)
                        }

                        Screen::CoolantTemp => format!("Coolant: {:.0} °C", self.coolant),

                        Screen::Afr => format!("AFR: {:.1}", self.afr),

                        Screen::IgnitionTiming => format!("Timing: {:.1}°", self.ign_timing),

                        Screen::FuelDuty => format!("Duty: {:.0}%", self.duty),

                        Screen::Ethanol => format!("Ethanol: {:.0}%", self.ethanol),
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

                // mock data for now
                boost: 18.2,
                oil_temp: 87.0,
                oil_press: 4.3,
                fuel_press: 3.8,
                coolant: 82.0,
                afr: 11.2,
                ign_timing: 18.0,
                duty: 56.0,
                ethanol: 73.0,
            })
        }),
    )
}

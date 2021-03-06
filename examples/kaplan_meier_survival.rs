//! Example of survival function estimated with a Kaplan Meier Estimator
//!
//! The original dataset was found [here](https://github.com/CamDavidsonPilon/lifelines/blob/master/lifelines/datasets/larynx.csv)
//!
//! The survival analysis is done with the lifelines python library.
//!

extern crate astrup;
extern crate csv;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::path::Path;
use failure::Error;

use astrup::{Chart, Figure, Line, LineStyle, Plot, Scatter, StrokeStyle, View};

#[derive(Debug, Deserialize)]
struct SurvivalRecord {
    stage: Option<u8>,
    time: Option<f64>,
    event: Option<u8>,
    survival: Option<f64>,
    ci_lower_95: Option<f64>,
    ci_upper_95: Option<f64>,
}

fn get_survival_data(csv_fname: &Path) -> Result<Vec<SurvivalRecord>, Error> {
    let mut csv_reader = csv::Reader::from_path(csv_fname)?;
    let mut survival_data = Vec::<SurvivalRecord>::new();
    for result in csv_reader.deserialize() {
        let record: SurvivalRecord = result?;
        survival_data.push(record)
    }

    Ok(survival_data)
}

fn missing_in_record(record: &SurvivalRecord) -> bool {
    record.stage == None || record.time == None || record.event == None || record.survival == None
        || record.ci_lower_95 == None || record.ci_upper_95 == None
}

fn extract_data(
    data: &[SurvivalRecord],
    stage: u8,
    event: u8,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut time = Vec::<f64>::new();
    let mut survival = Vec::<f64>::new();
    let mut lower_ci = Vec::<f64>::new();
    let mut upper_ci = Vec::<f64>::new();
    for record in data {
        if !missing_in_record(record) && record.stage.expect("Missing stage") == stage
            && record.event.expect("Missing event") == event
        {
            time.push(record.time.expect("Missing time"));
            survival.push(record.survival.expect("Missing survival"));
            lower_ci.push(record.ci_lower_95.expect("Missing survival"));
            upper_ci.push(record.ci_upper_95.expect("Missing survival"));
        }
    }
    (time, survival, lower_ci, upper_ci)
}

fn survival_charts(
    data: &[SurvivalRecord],
    stage: u8,
    red: f32,
    green: f32,
    blue: f32,
) -> (Line, Scatter) {
    let (time, survival, _, _) = extract_data(data, stage, 1);
    let (cens_time, cens_survival, _, _) = extract_data(data, stage, 0);

    let mut survival_line = Line::new(&time, &survival);
    survival_line.set_color_rgb(red / 255.0, green / 255.0, blue / 255.0)
                 .set_line_style(LineStyle::RightStair);
    let mut censored = Scatter::new(&cens_time, &cens_survival);
    censored.set_color_rgb(red / 255.0, green / 255.0, blue / 255.0)
            .set_point_size(0.01)
            .set_shape("tick");
    (survival_line, censored)
}

fn ci_charts(data: &[SurvivalRecord], stage: u8, red: f32, green: f32, blue: f32) -> (Line, Line) {
    let (time, _, lower_ci, upper_ci) = extract_data(data, stage, 1);

    let mut lower_line = Line::new(&time, &lower_ci);
    lower_line.set_color_rgb(red / 255.0, green / 255.0, blue / 255.0)
              .set_line_style(LineStyle::RightStair)
              .set_stroke_style(StrokeStyle::Dotted);
    let mut upper_line = Line::new(&time, &upper_ci);
    upper_line.set_color_rgb(red / 255.0, green / 255.0, blue / 255.0)
              .set_line_style(LineStyle::RightStair)
              .set_stroke_style(StrokeStyle::Dotted);
    (lower_line.clone(), upper_line.clone())
}

fn main() {
    let analysis_fname = Path::new("assets/larynx_survival_estimate.csv");
    match get_survival_data(analysis_fname) {
        Ok(data) => {
            let (surv_stage_1, cens_stage_1) = survival_charts(&data, 1, 224.0, 52.0, 11.0);
            let (surv_stage_2, cens_stage_2) = survival_charts(&data, 2, 23.0, 108.0, 190.0);
            let (surv_stage_3, cens_stage_3) = survival_charts(&data, 3, 255.0, 200.0, 14.0);
            let (surv_stage_4, cens_stage_4) = survival_charts(&data, 4, 34.0, 174.0, 51.0);

            let mut survival_plot = Plot::new();
            survival_plot.add_chart(&Chart::Line(surv_stage_1.clone()))
                         .add_chart(&Chart::Scatter(cens_stage_1.clone()))
                         .add_chart(&Chart::Line(surv_stage_2))
                         .add_chart(&Chart::Scatter(cens_stage_2))
                         .add_chart(&Chart::Line(surv_stage_3))
                         .add_chart(&Chart::Scatter(cens_stage_3))
                         .add_chart(&Chart::Line(surv_stage_4.clone()))
                         .add_chart(&Chart::Scatter(cens_stage_4.clone()))
                         .set_x_label("Time")
                         .set_y_label("Survival")
                         .set_local_frame(0.0, 1.0, 0.5, 1.0);

            let (lower_stage_1, upper_stage_1) = ci_charts(&data, 1, 224.0, 52.0, 11.0);
            //let (surv_stage_2, cens_stage_2) = survival_charts(&data, 2, 23.0, 108.0, 190.0);
            //let (surv_stage_3, cens_stage_3) = survival_charts(&data, 3, 255.0, 200.0, 14.0);
            let (lower_stage_4, upper_stage_4) = ci_charts(&data, 4, 34.0, 174.0, 51.0);

            let mut ci_plot = Plot::new();
            ci_plot.add_chart(&Chart::Line(surv_stage_1))
                   .add_chart(&Chart::Line(lower_stage_1))
                   .add_chart(&Chart::Line(upper_stage_1))
                   .add_chart(&Chart::Scatter(cens_stage_1))
                   .add_chart(&Chart::Line(surv_stage_4))
                   .add_chart(&Chart::Line(lower_stage_4))
                   .add_chart(&Chart::Line(upper_stage_4))
                   .add_chart(&Chart::Scatter(cens_stage_4))
                   .set_x_label("Time")
                   .set_y_label("Survival")
                   .set_local_frame(0.0, 1.0, 0.0, 0.5);

            let mut fig = Figure::new();
            fig.add_plot(&survival_plot)
               .add_plot(&ci_plot)
               .set_height(1000)
               .set_width(1000);
            //.save("assets/kaplan_meier_survival.png").expect("Could not save kaplan_meier_survival.png");

            match View::with_figure(fig) {
                Ok(view) => view.show(),
                Err(msg) => println!("Error in view: \n {}", msg),
            }
        }
        Err(msg) => println!("Error in getting survival data: \n {}", msg),
    }
}

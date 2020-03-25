use crate::app::App;
use crate::colors;
use crate::helpers::ID;
use crate::info::trip::trip_details;
use crate::info::InfoTab;
use ezgui::{EventCtx, Line, TextExt, Widget};
use map_model::Map;
use sim::{Person, PersonID, PersonState, TripResult};
use std::collections::HashMap;

pub fn info(
    ctx: &mut EventCtx,
    app: &App,
    id: PersonID,
    // If None, then the panel is embedded
    header_btns: Option<Widget>,
    action_btns: Vec<Widget>,
    hyperlinks: &mut HashMap<String, (ID, InfoTab)>,
    warpers: &mut HashMap<String, ID>,
) -> Vec<Widget> {
    let mut rows = vec![];

    // Header
    if let Some(btns) = header_btns {
        rows.push(Widget::row(vec![
            Line(format!("Person #{}", id.0)).roboto_bold().draw(ctx),
            btns,
        ]));
    } else {
        rows.push(Line(format!("Person #{}", id.0)).roboto_bold().draw(ctx));
    }
    // TODO None of these right now
    rows.extend(action_btns);

    let map = &app.primary.map;
    let sim = &app.primary.sim;
    let person = sim.get_person(id);

    // I'm sorry for bad variable names
    let mut wheres_waldo = true;
    for t in &person.trips {
        match sim.trip_to_agent(*t) {
            TripResult::TripNotStarted => {
                if wheres_waldo {
                    wheres_waldo = false;
                    rows.push(current_status(ctx, person, map));
                }
            }
            TripResult::Ok(_) | TripResult::ModeChange => {
                // ongoing
                assert!(wheres_waldo);
                wheres_waldo = false;
            }
            TripResult::TripDone => {
                assert!(wheres_waldo);
            }
            TripResult::TripDoesntExist => unreachable!(),
        }
        rows.push(
            Widget::col(vec![
                Line(format!("Trip #{}", t.0)).roboto_bold().draw(ctx),
                trip_details(ctx, app, *t, None, warpers).0,
            ])
            .bg(colors::SECTION_BG)
            .margin(10),
        );
    }
    if wheres_waldo {
        rows.push(current_status(ctx, person, map));
    }

    // TODO All the colorful side info

    rows
}

fn current_status(ctx: &EventCtx, person: &Person, map: &Map) -> Widget {
    match person.state {
        PersonState::Inside(b) => {
            // TODO hyperlink
            format!("Currently inside {}", map.get_b(b).just_address(map)).draw_text(ctx)
        }
        PersonState::Trip(_) => unreachable!(),
        PersonState::OffMap => "Currently outside the map boundaries".draw_text(ctx),
        PersonState::Limbo => "Currently in limbo -- they broke out of the Matrix! Woops. (A bug \
                               occurred)"
            .draw_text(ctx),
    }
}

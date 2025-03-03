use chrono::{Local, TimeZone};
use glib::Propagation;
use gtk4::{
    gdk, prelude::*, Align, ApplicationWindow, Box, EventControllerKey, Grid, Label, Orientation,
    Separator,
};
use std::time::Duration;

use crate::game::stats_manager::StatsManager;
use crate::model::{Difficulty, GameStats};

pub struct StatsDialog;

impl StatsDialog {
    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    fn create_high_scores_grid(
        difficulty: Difficulty,
        this_game_stats: Option<&GameStats>,
        stats_manager: &StatsManager,
    ) -> Grid {
        let scores_grid = Grid::new();
        scores_grid.set_row_spacing(5);
        scores_grid.set_column_spacing(10);
        scores_grid.set_margin_start(10);

        // Add headers
        let headers = ["Rank", "Time", "Hints", "Grid Size", "Difficulty", "Date"];
        for (i, header) in headers.iter().enumerate() {
            let label = Label::new(Some(header));
            label.set_markup(&format!("<b>{}</b>", header));
            scores_grid.attach(&label, i as i32, 0, 1, 1);
        }

        // Add high scores

        // pub fn get_high_scores(&self) -> Vec<GameStats> {
        //     self.stats_manager.get_high_scores(20).unwrap_or_default()
        // }

        // pub fn get_global_stats(&self) -> Option<super::stats::GlobalStats> {
        //     self.stats_manager.get_global_stats().ok()
        // }
        //     let high_scores = stats_manager.get_high_scores();
        for (i, score) in stats_manager
            .get_high_scores(difficulty, 20)
            .into_iter()
            .enumerate()
        {
            let is_current_playthrough = this_game_stats
                .map(|stats| stats.playthrough_id == score.playthrough_id)
                .unwrap_or(false);

            let row_index = (i + 1) as i32;

            let rank = Label::new(Some(&format!("{}.", i + 1)));
            rank.set_halign(Align::End);
            if is_current_playthrough {
                rank.add_css_class("highlight-score");
            }
            scores_grid.attach(&rank, 0, row_index, 1, 1);

            let time = Label::new(Some(&Self::format_duration(score.completion_time)));
            time.set_halign(Align::End);
            if is_current_playthrough {
                time.add_css_class("highlight-score");
            }
            scores_grid.attach(&time, 1, row_index, 1, 1);

            let hints = Label::new(Some(&format!("{}", score.hints_used)));
            hints.set_halign(Align::End);
            if is_current_playthrough {
                hints.add_css_class("highlight-score");
            }
            scores_grid.attach(&hints, 2, row_index, 1, 1);

            let size = Label::new(Some(&format!("{}x{}", score.grid_size, score.grid_size)));
            size.set_halign(Align::End);
            if is_current_playthrough {
                size.add_css_class("highlight-score");
            }
            scores_grid.attach(&size, 3, row_index, 1, 1);

            let difficulty = Label::new(Some(&format!("{:?}", score.difficulty)));
            difficulty.set_halign(Align::End);
            if is_current_playthrough {
                difficulty.add_css_class("highlight-score");
            }
            scores_grid.attach(&difficulty, 4, row_index, 1, 1);

            let date = Local
                .timestamp_opt(score.timestamp, 0)
                .single()
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            let date_label = Label::new(Some(&date));
            date_label.set_halign(Align::Start);
            if is_current_playthrough {
                date_label.add_css_class("highlight-score");
            }
            scores_grid.attach(&date_label, 5, row_index, 1, 1);
        }

        scores_grid
    }

    fn create_global_stats_grid(stats_manager: &StatsManager, difficulty: Difficulty) -> Grid {
        let stats = stats_manager.get_global_stats(difficulty);
        let stats_grid = Grid::new();
        stats_grid.set_row_spacing(5);
        stats_grid.set_column_spacing(10);
        stats_grid.set_margin_start(10);

        let total_games = Label::new(Some("Total Games:"));
        total_games.set_halign(Align::Start);
        stats_grid.attach(&total_games, 0, 0, 1, 1);
        let total_games_value = Label::new(Some(&stats.total_games_played.to_string()));
        total_games_value.set_halign(Align::End);
        stats_grid.attach(&total_games_value, 1, 0, 1, 1);

        let total_time = Label::new(Some("Total Time:"));
        total_time.set_halign(Align::Start);
        stats_grid.attach(&total_time, 0, 1, 1, 1);
        let total_time_value = Label::new(Some(&Self::format_duration(stats.total_time_played)));
        total_time_value.set_halign(Align::End);
        stats_grid.attach(&total_time_value, 1, 1, 1, 1);

        let avg_time = Label::new(Some("Average Time:"));
        avg_time.set_halign(Align::Start);
        stats_grid.attach(&avg_time, 0, 2, 1, 1);
        let avg_duration = if stats.total_games_played > 0 {
            Duration::from_secs_f64(
                stats.total_time_played.as_secs_f64() / stats.total_games_played as f64,
            )
        } else {
            Duration::from_secs(0)
        };
        let avg_time_value = Label::new(Some(&Self::format_duration(avg_duration)));
        avg_time_value.set_halign(Align::End);
        stats_grid.attach(&avg_time_value, 1, 2, 1, 1);

        let total_hints = Label::new(Some("Total Hints Used:"));
        total_hints.set_halign(Align::Start);
        stats_grid.attach(&total_hints, 0, 3, 1, 1);
        let total_hints_value = Label::new(Some(&stats.total_hints_used.to_string()));
        total_hints_value.set_halign(Align::End);
        stats_grid.attach(&total_hints_value, 1, 3, 1, 1);

        stats_grid
    }

    pub fn show<F>(
        window: &ApplicationWindow,
        difficulty: Difficulty,
        stats_manager: &StatsManager,
        this_game_stats: Option<&GameStats>,
        on_close: F,
    ) where
        F: Fn() + 'static,
    {
        let vbox = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_start(20)
            .margin_end(20)
            .margin_top(20)
            .margin_bottom(20)
            .build();
        let modal = gtk4::Window::builder()
            .title("Game Statistics")
            .modal(true)
            .default_width(400)
            .child(&vbox)
            .transient_for(window)
            .build();

        // Add title for high scores
        let high_scores_label = Label::new(Some("Best Times"));
        high_scores_label.set_markup("<b>Best Times</b>");
        high_scores_label.set_margin_bottom(10);
        vbox.append(&high_scores_label);

        // Add high scores grid
        let scores_grid = Self::create_high_scores_grid(difficulty, this_game_stats, stats_manager);
        vbox.append(&scores_grid);

        // Add separator
        let separator = Separator::new(Orientation::Horizontal);
        separator.set_margin_top(20);
        separator.set_margin_bottom(20);
        vbox.append(&separator);

        // Add global stats
        let global_stats_label = Label::new(Some("Global Statistics"));
        global_stats_label.set_markup("<b>Global Statistics</b>");
        global_stats_label.set_margin_bottom(10);
        vbox.append(&global_stats_label);

        let stats_grid = Self::create_global_stats_grid(stats_manager, difficulty);
        vbox.append(&stats_grid);

        let button_box = gtk4::Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::End)
            .build();
        let close_button = gtk4::Button::builder().label("Close").build();
        button_box.append(&close_button);

        vbox.append(&button_box);

        close_button.connect_clicked({
            let modal = modal.clone();
            move |_| {
                modal.close();
            }
        });
        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed({
            let modal = modal.clone();
            move |_, keyval, _, _| {
                if keyval == gdk::Key::Escape {
                    modal.close();
                    return Propagation::Stop;
                }
                Propagation::Proceed
            }
        });

        modal.connect_close_request({
            move |_| {
                on_close();
                Propagation::Proceed
            }
        });
        modal.add_controller(key_controller);

        modal.present();
    }
}

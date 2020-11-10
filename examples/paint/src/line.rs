use bevy::prelude::*;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Default)]
pub struct LineMaterial(pub Handle<ColorMaterial>);

pub struct LineDrawingState {
    pub cursor_event_reader: EventReader<CursorMoved>,
    pub touch_event_reader: EventReader<TouchInput>,
    pub cursor_curve: VecDeque<Vec2>,
    pub camera_entity: Entity,
}

impl LineDrawingState {
    pub fn new(camera_entity: Entity) -> Self {
        LineDrawingState {
            cursor_event_reader: Default::default(),
            touch_event_reader: Default::default(),
            cursor_curve: Default::default(),
            camera_entity,
        }
    }

    pub fn pop_line_segments(&mut self) -> Vec<(Vec2, Vec2)> {
        const SEGMENT_LENGTH: f32 = 15.0;

        // Downsample the cursor curve by length.
        let mut line_segments = Vec::new();
        let mut segment_start = if let Some(back) = self.cursor_curve.back() {
            *back
        } else {
            return line_segments;
        };
        let mut curve_length = 0.0;
        let mut segment_points = 0;
        let mut confirmed_segment_points = 0;
        for (p1, p2) in self.cursor_curve.iter().rev().tuple_windows() {
            let p1 = *p1;
            let p2 = *p2;
            segment_points += 1;
            let diff = p2 - p1;
            curve_length += diff.length();
            if curve_length >= SEGMENT_LENGTH {
                if segment_start != p2 {
                    line_segments.push((segment_start, p2));
                }
                segment_start = p2;
                confirmed_segment_points += segment_points;
                curve_length = 0.0;
                segment_points = 0;
            }
        }
        // Remove the points belonging to the segments we've gathered.
        self.cursor_curve
            .truncate(self.cursor_curve.len() - confirmed_segment_points);
        line_segments
    }
}

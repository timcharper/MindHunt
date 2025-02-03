use gdk_pixbuf::{InterpType, Pixbuf};
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use crate::model::Tile;

// TODO - use value from LayoutManager
const SOLUTION_IMG_SIZE: i32 = 128;
const CANDIDATE_IMG_SIZE: i32 = 64;

#[derive(Clone)]
pub struct OriginalIcons {
    icons: HashMap<(i32, i32), Rc<Pixbuf>>,
    negative_assertion: Rc<Pixbuf>,
    left_of: Rc<Pixbuf>,
    maybe_assertion: Rc<Pixbuf>,
}

pub struct ScaledIcons {
    solution_scale_icons: HashMap<(i32, i32), Rc<Pixbuf>>,
    candidate_scale_icons: HashMap<(i32, i32), Rc<Pixbuf>>,
    scaled_negative_assertion: Rc<Pixbuf>,
    scaled_left_of: Rc<Pixbuf>,
    scaled_maybe_assertion: Rc<Pixbuf>,
}

pub struct ImageSet {
    original_icons: OriginalIcons,
    scaled_icons: ScaledIcons,
}

impl ImageSet {
    pub fn new() -> Self {
        let mut original_icons: HashMap<(i32, i32), Rc<Pixbuf>> = HashMap::new();

        // Load all icon variants (8x8 grid of icons)
        for row in 0..8 {
            for col in 0..8 {
                let resource_path = format!("/org/mindhunt/assets/icons/{}/{}.png", row, col);
                let original_image = Pixbuf::from_resource(&resource_path)
                    .expect(&format!("Failed to load icon {} {}", row, col));
                original_icons.insert((row, col), Rc::new(original_image));
            }
        }

        // Load special icons
        let negative_assertion =
            Pixbuf::from_resource("/org/mindhunt/assets/icons/negative-assertion.png")
                .expect("Failed to load negative assertion icon");
        let negative_assertion = Rc::new(negative_assertion);

        let left_of = Pixbuf::from_resource("/org/mindhunt/assets/icons/left-of.png")
            .expect("Failed to load left-of icon");
        let left_of = Rc::new(left_of);

        let maybe_assertion =
            Pixbuf::from_resource("/org/mindhunt/assets/icons/maybe-assertion.png")
                .expect("Failed to load maybe assertion icon");
        let maybe_assertion = Rc::new(maybe_assertion);

        let original_icons = OriginalIcons {
            icons: original_icons,
            negative_assertion,
            left_of,
            maybe_assertion,
        };

        let scaled_icons =
            ImageSet::rescale_icons(&original_icons, CANDIDATE_IMG_SIZE, SOLUTION_IMG_SIZE);

        Self {
            original_icons,
            scaled_icons,
        }
    }

    fn rescale_icons(
        original_icons: &OriginalIcons,
        candidate_tile_size: i32,
        solution_tile_size: i32,
    ) -> ScaledIcons {
        let mut solution_scale_icons = HashMap::new();
        let mut candidate_scale_icons = HashMap::new();

        // Load all icon variants (8x8 grid of icons)
        for row in 0..8 {
            for col in 0..8 {
                let original_icon = original_icons.icons.get(&(row, col)).unwrap();
                let candidate_size = ImageSet::rescale_icon(original_icon, candidate_tile_size);
                let solution_size = ImageSet::rescale_icon(original_icon, solution_tile_size);
                candidate_scale_icons.insert((row, col), Rc::new(candidate_size));
                solution_scale_icons.insert((row, col), Rc::new(solution_size));
            }
        }

        // Load special icons
        let scaled_negative_assertion =
            ImageSet::rescale_icon(&original_icons.negative_assertion, candidate_tile_size);

        let scaled_left_of = ImageSet::rescale_icon(&original_icons.left_of, candidate_tile_size);

        let scaled_maybe_assertion =
            ImageSet::rescale_icon(&original_icons.maybe_assertion, candidate_tile_size);

        let scaled_icons = ScaledIcons {
            solution_scale_icons,
            candidate_scale_icons,
            scaled_negative_assertion: Rc::new(scaled_negative_assertion),
            scaled_left_of: Rc::new(scaled_left_of),
            scaled_maybe_assertion: Rc::new(scaled_maybe_assertion),
        };

        scaled_icons
    }

    pub fn optimized_image_set(
        &self,
        candidate_tile_size: i32,
        solution_tile_size: i32,
    ) -> ImageSet {
        let scaled_icons = ImageSet::rescale_icons(
            &self.original_icons,
            candidate_tile_size,
            solution_tile_size,
        );
        let image_set = ImageSet {
            original_icons: self.original_icons.clone(),
            scaled_icons,
        };

        image_set
    }

    fn rescale_icon(pixbuf: &Pixbuf, size: i32) -> Pixbuf {
        let scaled_image = pixbuf.scale_simple(size, size, InterpType::Bilinear);
        scaled_image.expect("Failed to scale icon")
    }

    pub fn get_candidate_icon(&self, tile: &Tile) -> Option<Rc<Pixbuf>> {
        self.scaled_icons
            .candidate_scale_icons
            .get(&(tile.row as i32, tile.variant as i32 - 'a' as i32))
            .cloned()
    }

    pub fn get_solution_icon(&self, tile: &Tile) -> Option<Rc<Pixbuf>> {
        self.scaled_icons
            .solution_scale_icons
            .get(&(tile.row as i32, tile.variant as i32 - 'a' as i32))
            .cloned()
    }

    pub fn get_negative_assertion(&self) -> Rc<Pixbuf> {
        Rc::clone(&self.scaled_icons.scaled_negative_assertion)
    }

    pub fn get_left_of(&self) -> Rc<Pixbuf> {
        Rc::clone(&self.scaled_icons.scaled_left_of)
    }

    pub fn get_maybe_assertion(&self) -> Rc<Pixbuf> {
        Rc::clone(&self.scaled_icons.scaled_maybe_assertion)
    }
}

impl Debug for ImageSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceSet")
    }
}
